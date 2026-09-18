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
let deviceFilter = 'all';
const deviceMarkers = new Map();
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
const age = value => typeof value !== 'number' || !Number.isFinite(value) ? 'Unbekannt' : value < 0 ? 'Zeit liegt in der Zukunft' : value < 60 ? Math.round(value) + ' s' : value < 3600 ? Math.round(value / 60) + ' min' : (value / 3600).toLocaleString('de-DE', {maximumFractionDigits:1}) + ' h';
function validPosition(device) {
  return Number.isFinite(device.latitude) && Number.isFinite(device.longitude) && Math.abs(device.latitude) <= 90 && Math.abs(device.longitude) <= 180;
}
const deviceMarkerKey = device => JSON.stringify([device.issi, device.node_id, device.latitude, device.longitude]);
function deviceOverview() {
  if(Array.isArray(snapshot.device_overview)) return snapshot.device_overview;
  // Older services expose usable devices and rejected records separately. Keep both visible,
  // but do not infer a geographic match from an earlier delivery or a missing diagnostic.
  const rows = new Map();
  for(const [index, device] of (snapshot.device_diagnostics || []).entries()) {
    rows.set(device.issi == null ? 'unknown-' + index : String(device.issi), {...device, available:false, area_status:'unknown', matching_alerts:[], legacy:true});
  }
  for(const device of snapshot.devices || []) {
    rows.set(String(device.issi), {...device, available:true, gps_age_seconds:device.age_seconds, area_status:'unknown', matching_alerts:[], legacy:true});
  }
  return [...rows.values()].sort((a, b) => (a.issi ?? Infinity) - (b.issi ?? Infinity));
}
function needsDeviceCheck(device) {
  return !device.available || (device.matching_alerts || []).some(alert => ['failed', 'uncertain'].includes(alert.delivery_state));
}
function deliveryStatus(alert) {
  const state = alert.delivery_state;
  // A stored result takes precedence over current eligibility and the global sending switch.
  if(state) {
    const terminal = ['failed', 'uncertain', 'cancelled', 'expired'].includes(state);
    return {
      text: labels[state] || state,
      tone: state === 'accepted' ? 'good' : terminal ? 'warn' : 'neutral',
      detail: alert.delivery_error || (state === 'accepted' ? 'Keine Empfangsbestätigung' : terminal ? 'Keine automatische Wiederholung' : ''),
      terminal,
    };
  }
  if(!alert.eligible) return {text:'Versand ausgesetzt', tone:'warn', detail:alert.eligibility_reason || 'Diese Warnung ist derzeit nicht für den Versand freigegeben.'};
  if(!snapshot.delivery_enabled) return {text:'Versand pausiert', tone:'neutral', detail:'Automatischer Versand ist ausgeschaltet.'};
  if(!snapshot.router_ready) return {text:'SDS-Router nicht bereit', tone:'warn', detail:'Versand wartet auf die Verbindung zum SDS-Router.'};
  return {text:'Versand ausstehend', tone:'neutral', detail:'Wird beim nächsten Abgleich geprüft.'};
}
function focusDevice(device) {
  if(!validPosition(device)) return;
  map.setView([device.latitude, device.longitude], 15);
  deviceMarkers.get(deviceMarkerKey(device))?.openPopup();
  $('map').scrollIntoView({behavior:'smooth', block:'center'});
}
function renderDeviceChecks() {
  const overview = deviceOverview();
  const rows=$('device-check-list'); rows.replaceChildren();
  const waiting=snapshot.last_cycle==null;
  const failed=Boolean(snapshot.errors?.control_room);
  const available = overview.filter(d => d.available).length;
  const affected = overview.filter(d => d.area_status === 'affected').length;
  const check = overview.filter(needsDeviceCheck).length;
  const shown = overview.filter(d => deviceFilter === 'all' || (deviceFilter === 'affected' ? d.area_status === 'affected' : needsDeviceCheck(d)));
  $('device-check-summary').textContent = `${overview.length} Geräte · ${available} verfügbar · ${affected} im Warngebiet · ${check} mit Prüfbedarf`;
  for(const [filter, name, count] of [['all', 'Alle', overview.length], ['affected', 'Im Warngebiet', affected], ['check', 'Prüfung nötig', check]]) {
    const button = $('device-filter-' + filter);
    button.textContent = name + ' (' + count + ')';
    button.setAttribute('aria-pressed', String(deviceFilter === filter));
  }
  const state = $('device-check-state');
  state.textContent = failed ? 'Geräteabgleich fehlgeschlagen. Verbindung zum Control Room prüfen.' : waiting ? 'Der erste Geräteabgleich läuft. Geräte erscheinen nach der Antwort des Control Rooms.' : !Array.isArray(snapshot.device_overview) ? 'Der Warn-Dienst liefert noch die bisherige Geräteansicht. Geräte werden angezeigt; der Gebietsstatus ist nach dem Dienstupdate verfügbar.' : '';
  state.className = 'device-state' + (failed ? ' warning' : '');
  state.hidden = !state.textContent;
  for(const d of shown) {
    const row = element('tr');
    const identity = element('td');
    identity.append(element('strong', d.issi == null ? 'ISSI unbekannt' : 'ISSI ' + d.issi, 'device-identity'), element('span', d.node_id || 'TBS unbekannt', 'device-detail'));
    if(d.node_age_seconds != null) identity.append(element('span', 'TBS-Meldung: ' + age(d.node_age_seconds), 'device-detail'));
    if(!d.available) identity.append(element('span', d.reason || 'Gerätedaten prüfen', 'device-issue'));
    const position = element('td');
    position.append(element('span', validPosition(d) ? `${d.latitude.toFixed(5)}, ${d.longitude.toFixed(5)}` : 'Keine nutzbare Position', 'device-position'));
    position.append(element('span', 'GPS-Alter: ' + age(d.gps_age_seconds), 'device-detail'));
    if(d.updated_at) position.append(element('span', date(d.updated_at), 'device-detail'));
    if(!d.available && validPosition(d)) position.append(element('span', 'Letzte bekannte Position', 'device-detail'));
    const area = element('td');
    const areaText = {affected:'Im Warngebiet', outside:'Außerhalb', no_alerts:'Keine aktiven Warnungen', unknown:'Nicht geprüft'};
    area.append(element('span', areaText[d.area_status] || 'Nicht geprüft', 'status-tag ' + (d.area_status === 'affected' ? 'affected' : 'neutral')));
    const matches = d.matching_alerts || [];
    const areaDetail = d.area_status === 'affected' ? `${matches.length} ${matches.length === 1 ? 'aktive Warnung' : 'aktive Warnungen'}` : d.area_status === 'outside' ? 'Keine aktive Warnung am Standort' : d.area_status === 'no_alerts' ? 'Derzeit kein Gebiet zu prüfen' : d.legacy ? 'Dienstupdate erforderlich' : 'Zuerst Gerätedaten prüfen';
    area.append(element('span', areaDetail, 'device-detail'));
    const delivery = element('td');
    for(const alert of matches) {
      const status = deliveryStatus(alert);
      const item = element('div', undefined, 'device-delivery');
      item.append(element('strong', alert.title || 'Warnung ohne Titel', 'device-alert-title'), element('span', status.text, 'status-tag ' + status.tone));
      if(status.detail) item.append(element('span', status.detail, 'device-detail' + (status.tone === 'warn' ? ' device-issue' : '')));
      if(status.terminal && alert.delivery_error) item.append(element('span', 'Keine automatische Wiederholung', 'device-detail'));
      if(alert.delivery_updated_at) item.append(element('span', date(alert.delivery_updated_at), 'device-detail'));
      delivery.append(item);
    }
    if(!matches.length) delivery.append(element('span', d.legacy ? 'Siehe Zustellhistorie' : d.available ? 'Kein Versand erforderlich' : 'Wartet auf Gerätedaten', 'device-detail'));
    const action = element('td');
    if(validPosition(d)) {
      const button = element('button', 'Auf Karte', 'quiet');
      button.type = 'button';
      button.setAttribute('aria-label', 'Position von ' + (d.issi == null ? 'unbekanntem Gerät' : 'ISSI ' + d.issi) + ' auf Karte anzeigen');
      button.onclick = () => focusDevice(d);
      action.append(button);
    } else action.append(element('span', '–'));
    row.append(identity, position, area, delivery, action);
    rows.append(row);
  }
  if(!shown.length) {
    const message=failed?'Geräte konnten nicht geladen werden.':waiting?'Die Geräte werden noch abgefragt.':!overview.length?'Der Control Room meldet keine angemeldeten Geräte. Funkgerät anmelden und GPS senden.':deviceFilter === 'affected'?'Keine Geräte mit geprüftem Standort im Warngebiet.':'Für diese Geräte ist derzeit keine Prüfung nötig.';
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
  notice(errors.length?errors.join('\n'):waiting?'Der erste Geräteabgleich läuft.':!snapshot.delivery_enabled?'Vorschau aktiv. Automatischen Versand in der Dienstkonfiguration einschalten, sobald die Einrichtung geprüft ist.':noDevices?'Versand ist aktiv, aber es ist kein Gerät mit verwendbarer Position verfügbar. Anmeldung, GPS und TBS-Verbindung unter „Geräte & Warnstatus“ prüfen.':'');
  renderDeviceChecks();
  areas.clearLayers(); devices.clearLayers(); deviceMarkers.clear();
  for(const a of active) { try { areaLayer(a).bindPopup(popup(a)).addTo(areas); } catch(err) { console.warn('Warngebiet nicht darstellbar',err); } }
  for(const d of deviceOverview()) {
    if(!validPosition(d)) continue;
    const content = element('div');
    content.append(element('strong', 'ISSI ' + (d.issi ?? 'unbekannt')), element('span', (d.node_id || 'TBS unbekannt') + ' · GPS ' + date(d.updated_at)), element('p', d.available ? 'Für Warnungen verfügbar' : d.reason || 'Gerätedaten prüfen'));
    const marker = L.circleMarker([d.latitude,d.longitude],{radius:6,color:'#fff',weight:2,fillColor:d.available?'#1d937e':'#a77427',fillOpacity:1,bubblingMouseEvents:false}).bindPopup(content).addTo(devices);
    // Do not let a device-marker click place the centre of a new warning.
    marker.on('click', event => { if(event.originalEvent) L.DomEvent.stopPropagation(event.originalEvent); });
    deviceMarkers.set(deviceMarkerKey(d), marker);
  }
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
async function refresh() { if(busy)return;busy=true;try{snapshot=await api('status');$('login').hidden=true;render();}catch(e){notice(e.message,true);$('connection').textContent='Nicht verbunden';$('connection').className='pill bad';const state=$('device-check-state');state.textContent=snapshot?'Warnstatus konnte nicht aktualisiert werden. Die angezeigten Gerätedaten sind möglicherweise veraltet.':'Geräte konnten nicht geladen werden. Verbindung und Zugriffsschlüssel prüfen.';state.className='device-state warning';state.hidden=false;}finally{busy=false;} }
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
for(const filter of ['all', 'affected', 'check']) $('device-filter-' + filter).onclick = () => { deviceFilter = filter; if(snapshot) renderDeviceChecks(); };
for(const name of ['latitude','longitude','radius_m'])fields[name].addEventListener('input',previewArea);
$('login-form').onsubmit=async e=>{e.preventDefault();token=$('token').value.trim();sessionStorage.setItem('netcore-alert-token',token);$('token').value='';await refresh();};
$('logout').onclick=()=>{sessionStorage.removeItem('netcore-alert-token');token='';location.reload();};
form.onsubmit=async e=>{e.preventDefault();const b=$('save-alert');b.disabled=true;try{const data=Object.fromEntries(new FormData(form));for(const k of ['latitude','longitude','radius_m'])data[k]=Number(data[k]);data.expires_at=new Date(data.expires_at).toISOString();await api('alerts',{method:'POST',body:JSON.stringify(data)});$('close-composer').click();form.reset();await refresh();}catch(err){notice(err.message,true);}finally{b.disabled=false;}};
refresh();setInterval(refresh,10000);
