'use strict';
const $ = id => document.getElementById(id);
const form = $('alert-form');
const fields = form.elements;
let token = sessionStorage.getItem('netcore-alert-token') || '';
let snapshot = null;
let busy = false;
let selected = null;
let preview = null;
let firstMapFit = true;
let pendingDelete = null;
const map = L.map('map').setView([51.1, 10.4], 6);
L.tileLayer('https://tile.openstreetmap.org/{z}/{x}/{y}.png', {
  maxZoom: 18, attribution: '&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a>'
}).on('tileerror', () => { $('map-note').textContent = 'Kartenhintergrund nicht erreichbar · Gebiete bleiben sichtbar'; }).addTo(map);
const areas = L.featureGroup().addTo(map);
const devices = L.layerGroup().addTo(map);
const labels = {pending:'Ausstehend',submitted:'Beim SDS-Router',accepted:'Von TBS angenommen',uncertain:'Unklar · keine Wiederholung',failed:'Fehlgeschlagen',cancelled:'Gestoppt',expired:'Abgelaufen'};
const severityNames = {Minor:'Gering',Moderate:'Erhöht',Severe:'Hoch',Extreme:'Extrem',Unknown:'Unbekannt'};
const date = value => value ? new Date(typeof value === 'number' ? value * 1000 : value).toLocaleString('de-DE', {dateStyle:'short',timeStyle:'short'}) : '–';
function element(tag, text, cls) { const n = document.createElement(tag); if(text !== undefined) n.textContent=text; if(cls) n.className=cls; return n; }
function notice(message, error=false) { $('notice').textContent=message; $('notice').className=error?'error':''; $('notice').hidden=!message; }
async function api(path, options={}) {
  const response = await fetch('/api/v1/' + path, {...options, headers:{'Authorization':'Bearer '+token, 'Content-Type':'application/json', ...options.headers}});
  let data;
  try { data=await response.json(); } catch { throw new Error('Ungültige Antwort der Warnzentrale'); }
  if(response.status===401) { $('login').hidden=false; throw new Error('Zugriffsschlüssel eingeben oder prüfen.'); }
  if(!response.ok) throw new Error(data.error || 'Anfrage fehlgeschlagen');
  return data;
}
function areaLayer(alert, options={}) {
  const style={color:alert.source==='manual'?'#235cf2':'#e28b34',weight:2,fillOpacity:.16,...options};
  if(alert.geometry.type==='Circle') return L.circle([alert.geometry.coordinates[1],alert.geometry.coordinates[0]],{...style,radius:alert.geometry.radius_m});
  return L.geoJSON(alert.geometry,{style});
}
function popup(alert) { const n=element('div'); n.append(element('strong',alert.title),element('span',(severityNames[alert.severity]||alert.severity)+' · '+(alert.provider||alert.source))); return n; }
function fitAreas() { if(areas.getLayers().length && areas.getBounds().isValid()) map.fitBounds(areas.getBounds(),{padding:[35,35],maxZoom:13}); }
function renderDeviceChecks() {
  const panel=$('device-checks');
  panel.hidden=!Array.isArray(snapshot.device_diagnostics);
  if(panel.hidden) return;
  const rows=$('device-check-list'); rows.replaceChildren();
  const waiting=snapshot.last_cycle==null;
  const failed=Boolean(snapshot.errors.control_room);
  $('device-check-summary').textContent=failed?'Geräteabgleich fehlgeschlagen':waiting?'Erster Geräteabgleich läuft …':`${snapshot.subscribers_seen} vom Control Room als online gemeldet · ${snapshot.devices.length} für Warnungen verfügbar`;
  const age=value=>typeof value==='number'?(value<0?'Zeit liegt in der Zukunft':value<60?Math.round(value)+' s':value<3600?Math.round(value/60)+' min':(value/3600).toLocaleString('de-DE',{maximumFractionDigits:1})+' h'):'–';
  for(const d of snapshot.device_diagnostics) {
    const row=element('tr');
    row.append(element('td',d.issi==null?'Unbekannt':String(d.issi)),element('td',d.node_id||'–'),element('td',d.reason),element('td',age(d.gps_age_seconds)),element('td',age(d.node_age_seconds)));
    rows.append(row);
  }
  if(!snapshot.device_diagnostics.length) {
    const message=failed?'Verbindung zum Control Room prüfen.':waiting?'Die Geräte werden noch abgefragt.':snapshot.subscribers_seen===0?'Der Control Room meldet keine angemeldeten Geräte. Anmeldung des Testgeräts dort prüfen.':'Alle gemeldeten Geräte erfüllen die Voraussetzungen für den Gebietsabgleich.';
    const cell=element('td',message,'empty'); cell.colSpan=5; const row=element('tr'); row.append(cell); rows.append(row);
  }
}
function render() {
  if(!snapshot) return;
  const active=snapshot.alerts.filter(a=>a.active);
  $('count-alerts').textContent=active.length;
  $('count-devices').textContent=snapshot.devices.length;
  $('count-deliveries').textContent=snapshot.deliveries.filter(d=>d.state==='accepted').length;
  $('send-mode').textContent=snapshot.delivery_enabled?'Aktiv':'Pausiert';
  $('send-mode').style.color=snapshot.delivery_enabled?'#228c60':'#a77427';
  $('poll-time').textContent='Geprüft: '+date(snapshot.last_cycle);
  const errors=Object.entries(snapshot.errors).map(([key,value])=>key+': '+value);
  const waiting=snapshot.last_cycle==null;
  const noDevices=!waiting && snapshot.delivery_enabled && active.some(a=>a.eligible) && !snapshot.devices.length;
  $('connection').textContent=errors.length?'Verbindung prüfen':waiting?'Abgleich läuft':noDevices?'Keine Empfänger':'Verbunden';
  $('connection').className='pill '+(errors.length||waiting||noDevices?'bad':'good');
  notice(errors.length?errors.join('\n'):waiting?'Der erste Geräteabgleich läuft.':!snapshot.delivery_enabled?'Vorschau aktiv. Automatischen Versand in der Dienstkonfiguration einschalten, sobald die Einrichtung geprüft ist.':noDevices?'Versand ist aktiv, aber es ist kein Gerät mit verwendbarer Position verfügbar. Anmeldung, GPS und TBS-Verbindung in der Geräteprüfung unten prüfen.':'');
  renderDeviceChecks();
  areas.clearLayers(); devices.clearLayers();
  for(const a of active) { try { areaLayer(a).bindPopup(popup(a)).addTo(areas); } catch(err) { console.warn('Warngebiet nicht darstellbar',err); } }
  for(const d of snapshot.devices) L.circleMarker([d.latitude,d.longitude],{radius:5,color:'#fff',weight:2,fillColor:'#1d937e',fillOpacity:1}).bindPopup(element('span','ISSI '+d.issi+' · '+d.node_id+' · GPS '+date(d.updated_at))).addTo(devices);
  if(firstMapFit && areas.getLayers().length) { fitAreas(); firstMapFit=false; }
  const list=$('alert-list'); list.replaceChildren();
  const shown=snapshot.alerts.filter(a=>$('show-history').checked||a.active);
  if(!shown.length) list.append(element('p','Keine aktiven Meldungen im Feed oder aus eigener Erstellung.','empty'));
  for(const a of shown) {
    const card=element('article',undefined,'alert-card');
    const meta=element('div',undefined,'meta');
    meta.append(element('span',a.source==='manual'?'EIGENE MELDUNG':(a.provider||'NINA').toUpperCase(),'badge '+(a.active?(a.source==='manual'?'own':''):'inactive')),element('span',a.active?(severityNames[a.severity]||a.severity):'Archiviert'));
    card.append(meta,element('h3',a.title));
    if(a.description) card.append(element('p',a.description.length>260?a.description.slice(0,260)+'…':a.description));
    const detail=document.createElement('details');detail.append(element('summary','Volltext & Funktext'));
    detail.append(element('p',(a.description||'')+(a.instruction?'\n'+a.instruction:'')),element('p','Funktext: '+a.radio_text));card.append(detail);
    const actions=element('div',undefined,'actions');
    const zoom=element('button','Auf Karte','quiet');zoom.onclick=()=>{try{const l=areaLayer(a);if(l.getBounds().isValid())map.fitBounds(l.getBounds(),{padding:[40,40],maxZoom:14});}catch{notice('Für diese Meldung ist kein Gebiet verfügbar.');}};
    actions.append(zoom,element('small',a.expires_at?'Bis '+date(a.expires_at):'Bis zur Rücknahme'));
    if(a.source==='manual' && !a.removed) { const remove=element('button','Löschen','quiet danger'); remove.onclick=()=>{pendingDelete=a.id;$('delete-title').textContent=a.title;$('delete-dialog').showModal();};actions.append(remove); }
    card.append(actions); list.append(card);
  }
  const tbody=$('delivery-list');tbody.replaceChildren();
  $('history-count').textContent=snapshot.deliveries.length+' Einträge';
  const titles=new Map(snapshot.alerts.map(a=>[a.id,a.title]));
  for(const d of snapshot.deliveries.slice(0,500)) {const tr=element('tr');tr.append(element('td',titles.get(d.alert_id)||d.alert_id),element('td',String(d.issi)),element('td',labels[d.state]||d.state),element('td',date(d.updated_at)),element('td',d.last_error||(d.state==='accepted'?'Annahme zum Senden, keine Lesebestätigung':'–')));tbody.append(tr);}
  if(!snapshot.deliveries.length) {const td=element('td','Noch keine Zustellungen.','empty');td.colSpan=5;const tr=element('tr');tr.append(td);tbody.append(tr);}
}
async function refresh() { if(busy)return;busy=true;try{snapshot=await api('status');$('login').hidden=true;render();}catch(e){notice(e.message,true);$('connection').textContent='Nicht verbunden';$('connection').className='pill bad';}finally{busy=false;} }
function openComposer(latlng) {
  $('composer').hidden=false;
  selected=latlng||selected||map.getCenter();
  fields.latitude.value=selected.lat.toFixed(6);fields.longitude.value=selected.lng.toFixed(6);
  if(!fields.expires_at.value){const t=new Date(Date.now()+2*3600000);fields.expires_at.value=new Date(t.getTime()-t.getTimezoneOffset()*60000).toISOString().slice(0,16);}
  previewArea();
  $('composer').scrollIntoView({behavior:'smooth',block:'nearest'});
}
function previewArea(){ if(preview)map.removeLayer(preview);preview=null;const lat=Number(fields.latitude.value),lon=Number(fields.longitude.value),radius=Number(fields.radius_m.value);if(!Number.isFinite(lat)||!Number.isFinite(lon)||!Number.isFinite(radius)||Math.abs(lat)>90||Math.abs(lon)>180||radius<50||radius>200000)return;preview=L.circle([lat,lon],{radius,color:'#235cf2',dashArray:'5 6',weight:2,fillOpacity:.08}).bindTooltip('Neue Meldung · '+Math.round(radius)+' m').addTo(map);}
map.on('click',e=>openComposer(e.latlng));
$('new-alert').onclick=()=>openComposer();
$('close-composer').onclick=()=>{$('composer').hidden=true;if(preview)map.removeLayer(preview);preview=null;};
$('fit-map').onclick=fitAreas;
$('cancel-delete').onclick=()=>{$('delete-dialog').close();pendingDelete=null;};
$('confirm-delete').onclick=async()=>{if(!pendingDelete)return;const button=$('confirm-delete');button.disabled=true;try{await api('alerts/'+encodeURIComponent(pendingDelete),{method:'DELETE'});$('delete-dialog').close();pendingDelete=null;await refresh();}catch(e){$('delete-dialog').close();notice(e.message,true);}finally{button.disabled=false;}};
$('show-history').onchange=render;
for(const name of ['latitude','longitude','radius_m'])fields[name].addEventListener('input',previewArea);
$('login-form').onsubmit=async e=>{e.preventDefault();token=$('token').value.trim();sessionStorage.setItem('netcore-alert-token',token);$('token').value='';await refresh();};
$('logout').onclick=()=>{sessionStorage.removeItem('netcore-alert-token');token='';location.reload();};
form.onsubmit=async e=>{e.preventDefault();const b=$('save-alert');b.disabled=true;try{const data=Object.fromEntries(new FormData(form));for(const k of ['latitude','longitude','radius_m'])data[k]=Number(data[k]);data.expires_at=new Date(data.expires_at).toISOString();await api('alerts',{method:'POST',body:JSON.stringify(data)});$('close-composer').click();form.reset();await refresh();}catch(err){notice(err.message,true);}finally{b.disabled=false;}};
refresh();setInterval(refresh,10000);
