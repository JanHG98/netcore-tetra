/* NetCore shell: existing controls/telemetry keep their IDs and authority. */
(() => {
  'use strict';
  const byId = id => document.getElementById(id);
  const make = (tag, className, text) => {
    const node = document.createElement(tag);
    node.className = className;
    if (text) node.textContent = text;
    return node;
  };
  const groups = {
    home: ['home'],
    radio: ['stations','calls','lastheard','sdslog','packetdata','rf'],
    network: ['maps','neighbors'],
    diagnostics: ['health','log','services'],
    administration: ['system','config','wifi','asterisk','audio','recordings','telegram','dapnet','echolink','meshcom','geoalarm','help']
  };
  const labels = {home:'Hauptseite',radio:'Funkbetrieb',network:'Netzkarte',diagnostics:'Diagnose',administration:'Verwaltung'};
  const pages = {
    home:['Funklage','Deine Basisstation im Überblick.'],
    stations:['Funkgeräte','Registrierte Teilnehmer, Rufgruppen und Signalwerte.'],
    calls:['Rufe','Aktive Gruppen- und Einzelrufe auf den verfügbaren Verkehrskanälen.'],
    lastheard:['Zuletzt gehört','Aktivität aus lokalem Funkbetrieb und angebundenen Netzen.'],
    sdslog:['SDS-Protokoll','Nachrichtenverlauf und übermittelte Positionsdaten.'],
    packetdata:['Paketdaten','SNDCP-Kontexte, PDCH-Bearer und Legacy-WAP über SDS.'],
    rf:['RF-Monitor','Live-TX-DSP vor dem Leistungsverstärker · keine kalibrierte Antennenmessung.'],
    maps:['Netzkarte','Übermittelte Positionen auf der Karte.'],
    neighbors:['Nachbarzellen','Konfigurierte Nachbarzellen und ihre TETRA-Kennungen.'],
    health:['Systemzustand','Zustand, Ursachen und Handlungsempfehlungen der Basisstation.'],
    log:['Live-Protokoll','Laufende Systemmeldungen mit Schweregradfilter und Export.'],
    services:['Dienste','Zentrale NetCore-Dienste und die Auswirkungen eines lokalen Fallbacks.'],
    system:['System','Hostinformationen, Hardware, Konfigurationsprofile und SDS-Aussendungen.'],
    config:['Konfiguration','Aktive config.toml, Zugangsrichtlinien und Wetterdienste.'],
    wifi:['WLAN','Netzwerkverbindung über den vorhandenen NetworkManager.'],
    asterisk:['Asterisk SIP','SIP-Anbindung und Snom-Benachrichtigungen.'],
    audio:['Audio-Zentrale','Medienbibliothek, Vorschau und Aussendung über TETRA.'],
    recordings:['Aufzeichnungen','Lokale Rufaufnahmen und Übergabe an das Server-Archiv.'],
    telegram:['Telegram','Bot, Empfänger und Benachrichtigungsregeln.'],
    dapnet:['DAPNET','Pager-Routing, Nachrichten und CallOut-Zuordnung.'],
    echolink:['EchoLink','Verzeichnis, Verbindungen und TETRA-Routing.'],
    meshcom:['MeshCom','UDP-Anbindung, Nachrichten, Knoten und Weiterleitung.'],
    geoalarm:['GeoAlarm','Eintritt in die konfigurierte Alarmzone und Weiterleitung.'],
    help:['Hilfe','Orientierung in der Basisstation. Weitere Hilfetexte folgen.'],
    public:['Öffentlicher Status','Freigegebene Übersicht mit Lesezugriff. Für Bedienung und Details anmelden.']
  };
  let currentPage = 'home';
  let selectedRadio = null;
  let neighbors = null;
  let sessionReady = false;

  // The old sidebar becomes an invisible storage area for context navigation.
  const pool = document.querySelector('#sidebar .sidebar-nav');
  pool.querySelectorAll('.nav-section-label').forEach(node => node.remove());
  const subnav = make('nav','nc-subnav');
  subnav.id = 'nc-subnav';
  subnav.setAttribute('aria-label','Bereichsseiten');
  byId('topbar').after(subnav);
  const heading = make('div','nc-page-heading');
  heading.innerHTML = '<div><div class="nc-eyebrow">BASISSTATION</div><div id="nc-heading-title"></div><p class="nc-page-description" id="nc-page-description"></p></div><div class="nc-heading-actions"><div id="nc-heading-chips"></div><button type="button" class="btn" id="nc-refresh">Aktualisieren</button></div>';
  byId('content').before(heading);
  byId('nc-heading-title').append(byId('topbar-title'));
  byId('topbar-title').setAttribute('role','heading');
  byId('topbar-title').setAttribute('aria-level','1');
  byId('nc-heading-chips').append(document.querySelector('.topbar-chips'));
  const brand = make('a','nc-brand');
  brand.href = '#home';
  brand.setAttribute('aria-label','NetCore-Tetra Hauptseite');
  brand.innerHTML = '<span class="nc-brand-art"><span class="nc-logo-mark"><img src="/assets/netcore-logo.png" alt=""></span><span class="nc-logo-wordmark"><img src="/assets/netcore-logo.png" alt="NetCore-Tetra"></span></span><span class="nc-station-name" id="nc-station-name">Basisstation</span>';
  brand.addEventListener('click',event => {event.preventDefault();if(sessionReady&&!document.body.classList.contains('nc-public'))showPage('home');});
  byId('topbar').prepend(brand);
  const mainNav = make('nav','nc-main-nav');
  mainNav.id = 'nc-main-nav';
  mainNav.setAttribute('aria-label','Hauptnavigation');
  const lastPages = Object.fromEntries(Object.entries(groups).map(([key,value])=>[key,value[0]]));
  for (const key of Object.keys(groups)) {
    const button = make('button','nc-main-link',labels[key]);
    button.type = 'button';button.dataset.group = key;button.disabled=true;
    button.addEventListener('click',()=>showPage(lastPages[key]));
    mainNav.append(button);
  }
  brand.after(mainNav);
  const content = byId('content');
  for (const name of ['home','recordings','services','neighbors','help']) {
    const page = make('div','page');page.id='page-'+name;content.append(page);
    const link = make('div','nav-item');link.id='nav-'+name;
    link.append(make('span','nav-label',name==='home'?'Hauptseite':pages[name][0]));
    link.addEventListener('click',()=>showPage(name,link));pool.append(link);
  }
  // Names in the horizontal context navigation match the approved design.
  const navigationNames = Object.fromEntries(Object.entries(pages).map(([name,value])=>[name,name==='home'?'Hauptseite':value[0]]));
  Object.entries(navigationNames).forEach(([name,label])=>{
    const node=byId('nav-'+name)?.querySelector('.nav-label');if(node){node.removeAttribute('data-i18n');node.textContent=label;}
  });
  document.querySelectorAll('.nav-item').forEach(node=>{
    node.setAttribute('role','button');node.tabIndex=0;
    node.addEventListener('keydown',event=>{if(event.key==='Enter'||event.key===' '){event.preventDefault();node.click();}});
  });

  const home=byId('page-home');
  home.innerHTML='<div id="nc-home-metrics" class="stat-grid nc-home-metrics"></div><div class="nc-home-layout"><div class="nc-home-main" id="nc-home-main"></div><aside class="nc-home-rail" id="nc-home-rail"></aside></div><div class="nc-shortcuts"><button class="nc-link" onclick="showPage(\'calls\')">Rufe öffnen →</button><button class="nc-link" onclick="showPage(\'stations\')">Funkgeräte verwalten →</button><button class="nc-link" onclick="showPage(\'log\')">Logs ansehen →</button></div>';
  const stats=byId('stat-ms').closest('.stat-grid');
  byId('nc-home-metrics').append(...Array.from(stats.children));stats.remove();
  // Two additional cards use the existing privileged system snapshot, never examples.
  for(const [id,label] of [['temp','CPU-Temperatur'],['uptime','Laufzeit']]){
    const card=make('div','stat-card');
    card.innerHTML='<div class="stat-label">'+label+'</div><div class="stat-value is-text" id="nc-home-'+id+'">—</div><div class="stat-sub" id="nc-home-'+id+'-sub">Warte auf Systemdaten</div>';
    byId('nc-home-metrics').append(card);
  }
  const channels=byId('ts-grid').closest('.card');
  byId('nc-home-main').append(channels);
  const preview=make('div','card');
  preview.innerHTML='<div class="card-head"><div class="card-title">Funkgeräte</div><button class="nc-link" onclick="showPage(\'stations\')">Alle anzeigen →</button></div><div class="table-wrap"><table><thead><tr><th>ISSI / Rufzeichen</th><th>Gruppen</th><th>Signal</th><th>Status</th></tr></thead><tbody id="nc-home-radios"></tbody></table></div>';
  byId('nc-home-main').append(preview);
  const status=make('div','card');
  status.innerHTML='<div class="card-head"><div class="card-title">Betriebszustand</div></div><div class="card-body"><div class="nc-status-line"><span class="nc-status-dot" id="nc-local-dot"></span><div><div class="nc-status-title" id="nc-local-title">Verbindung wird aufgebaut</div><div class="nc-status-desc" id="nc-local-desc">Warte auf Telemetrie.</div></div></div><div class="nc-status-line"><span class="nc-status-dot" id="nc-core-dot"></span><div><div class="nc-status-title" id="nc-core-title">Zentrale Dienste</div><div class="nc-status-desc" id="nc-core-desc">Warte auf Dienstestatus.</div><button class="nc-link" onclick="showPage(\'services\')">Details anzeigen →</button></div></div><div id="nc-home-hardware"></div></div>';
  byId('nc-home-rail').append(status);
  byId('nc-home-hardware').append(document.querySelector('.hw-status'));
  byId('nc-home-rail').append(byId('bts-tx').closest('.card'));
  const activity=make('div','card');
  activity.innerHTML='<div class="card-head"><div class="card-title">Zuletzt passiert</div></div><div class="card-body"><ul class="nc-events" id="nc-home-events"></ul><button class="nc-link" onclick="showPage(\'lastheard\')">Aktivität öffnen →</button></div>';
  byId('nc-home-rail').append(activity);

  const radioTable=byId('ms-tbody').closest('.card');
  const radioLayout=make('div','nc-radio-layout');radioTable.before(radioLayout);radioLayout.append(radioTable);
  const radioDetail=make('aside','card nc-radio-detail');
  radioDetail.innerHTML='<div class="card-head"><div class="card-title">Gerätedetails</div></div><div class="card-body" id="nc-radio-detail"><p class="help-text">Ein Funkgerät in der Tabelle auswählen.</p></div>';
  radioLayout.append(radioDetail);
  const radioFilter=make('input','form-input');radioFilter.id='nc-radio-filter';radioFilter.type='search';radioFilter.placeholder='ISSI, Rufzeichen oder Gruppe suchen';radioFilter.setAttribute('aria-label','Funkgeräte filtern');
  radioTable.querySelector('.card-head').append(radioFilter);
  radioFilter.addEventListener('input',applyRadioFilter);

  const audio=byId('page-audio');
  const recordingStart=Array.from(audio.children).find(node=>node.classList.contains('section-label')&&node.textContent.includes('AUFZEICHNUNGEN'));
  if(recordingStart){let node=recordingStart;while(node){const next=node.nextElementSibling;byId('page-recordings').append(node);node=next;}}
  byId('page-services').append(byId('core-services-card'));
  byId('page-neighbors').innerHTML='<div class="card"><div class="card-head"><div><div class="card-title">Konfigurierte Nachbarzellen</div><p class="card-sub">Konfigurationswerte · Verfügbarkeit und Handover werden hier nicht gemessen.</p></div><span class="pill pill-info" id="nc-neighbor-count">—</span></div><div class="table-wrap"><table><thead><tr><th>Zellkennung</th><th>Hauptträger</th><th>MCC / MNC</th><th>Location Area</th><th>Synchronisiert</th><th>Konfigurierte Zelllast</th></tr></thead><tbody id="nc-neighbor-rows"><tr><td colspan="6">Warte auf Konfiguration…</td></tr></tbody></table></div></div><p class="help-text">Änderungen erfolgen weiterhin in der Konfiguration. Live-Detailansichten und weitere Mobilitätsfunktionen sind für eine spätere Erweiterung vorgesehen.</p>';
  byId('page-help').innerHTML='<div class="nc-placeholder"><span class="pill pill-info">In Vorbereitung</span><h2 style="margin-top:18px">Hilfe zur Basisstation</h2><p>Die ausführlichen Hilfetexte werden später ergänzt. Die folgenden Bereiche führen bereits zu den vorhandenen Einstellungen und Diagnoseansichten.</p><div class="nc-help-grid"><div class="nc-help-topic"><h3>Funkbetrieb</h3><p>Teilnehmer, Rufe und TX-DSP-Messwerte.</p><button class="nc-link" onclick="showPage(\'stations\')">Funkgeräte öffnen →</button></div><div class="nc-help-topic"><h3>Diagnose</h3><p>Systemzustand, Protokoll und zentrale Dienste.</p><button class="nc-link" onclick="showPage(\'health\')">Systemzustand öffnen →</button></div><div class="nc-help-topic"><h3>Verwaltung</h3><p>Konfiguration, Host und angebundene Dienste.</p><button class="nc-link" onclick="showPage(\'config\')">Konfiguration öffnen →</button></div></div></div>';

  const footer=make('footer','nc-footer');
  const footerStatus=make('div','nc-footer-status');
  footerStatus.append(document.querySelector('.conn-status-row'),document.querySelector('.brew-status-row'));
  footer.append(footerStatus,document.querySelector('.sidebar-copyright'));
  byId('main').append(footer);

  const originalShowPage=window.showPage;
  window.showPage=function(name,el){
    if(!sessionReady||document.body.classList.contains('nc-public'))return;
    if(!pages[name])name='home';
    if(HIDDEN_INTEGRATIONS.includes(name)&&!hiddenIntegrationsVisible)name='home';
    originalShowPage(name,el);
    currentPage=name;
    const group=Object.keys(groups).find(key=>groups[key].includes(name))||'home';
    lastPages[group]=name;
    mainNav.querySelectorAll('button').forEach(button=>{
      const active=button.dataset.group===group;button.classList.toggle('active',active);button.setAttribute('aria-current',active?'page':'false');
    });
    while(subnav.firstChild)pool.append(subnav.firstChild);
    if(group!=='home')for(const pageName of groups[group]){
      const link=byId('nav-'+pageName);if(link)subnav.append(link);
    }
    document.querySelectorAll('.nav-item').forEach(node=>node.setAttribute('aria-current',node.id==='nav-'+name?'page':'false'));
    byId('topbar-title').textContent=pages[name][0];byId('nc-page-description').textContent=pages[name][1];
    if(name==='home'){loadBtsInfo();loadSystemInfo();renderHome();}
    if(name==='neighbors')loadNeighbors();
    if(name==='services')loadEdgeFallback(true);
    if(name==='recordings'){loadAudioPage(false);loadRecordings(true);}
    if(name==='audio'||name==='recordings'){
      for(const id of ['audio-context-menu','audio-send-card'])byId('page-'+name).append(byId(id));
    }
    window.scrollTo({top:0,behavior:'instant'});
    document.dispatchEvent(new CustomEvent('netcore:page',{detail:{name}}));
  };

  function applyRadioFilter(){
    const query=radioFilter.value.toLocaleLowerCase().trim();
    for(const row of byId('ms-tbody').rows)row.hidden=!!query&&!row.textContent.toLocaleLowerCase().includes(query);
  }
  function renderRadioDetail(){
    const device=selectedRadio&&state.ms[selectedRadio];
    if(!device){selectedRadio=null;byId('nc-radio-detail').innerHTML='<p class="help-text">Ein registriertes Funkgerät in der Tabelle auswählen.</p>';return;}
    const body=byId('nc-radio-detail');body.replaceChildren();
    const title=make('h3','',deviceInlineName(device.issi)||String(device.issi));title.style.marginBottom='18px';body.append(title);
    const details=make('dl','');
    const values=[['ISSI',device.issi],['Gruppen',(device.groups||[]).join(', ')||'—'],['Signal',device.rssi_dbfs!=null?Number(device.rssi_dbfs).toFixed(1)+' dBFS':'—'],['Zuletzt gesehen',device._last_seen_ts?Math.max(0,Math.floor((Date.now()-device._last_seen_ts)/1000))+' s':device.last_seen_secs_ago!=null?device.last_seen_secs_ago+' s':'—']];
    for(const [label,value]of values)details.append(make('dt','',label),make('dd','',String(value)));
    body.append(details,make('p','help-text','SDS, DGNA und weitere Geräteaktionen sind in der Tabellenzeile verfügbar.'));
  }
  function syncRadios(){
    const rows=Array.from(byId('ms-tbody').rows);
    const devices=Object.values(state.ms||{}).sort((a,b)=>a.issi-b.issi);
    rows.forEach((row,index)=>{
      if(!devices[index]||row.cells.length<7)return;
      const issi=String(devices[index].issi);row.dataset.ncIssi=issi;row.tabIndex=0;
      row.classList.toggle('nc-selected',selectedRadio===issi);
      row.onclick=event=>{if(event.target.closest('button,a,input,select'))return;selectedRadio=issi;syncRadios();};
      row.onkeydown=event=>{if(event.target===row&&(event.key==='Enter'||event.key===' ')){event.preventDefault();selectedRadio=issi;syncRadios();}};
    });
    applyRadioFilter();renderRadioDetail();
    const previewBody=byId('nc-home-radios');previewBody.replaceChildren();
    for(const row of rows.slice(0,5)){
      const copy=make('tr','');
      for(const index of [0,1,3,4]){
        if(!row.cells[index])continue;
        const cell=row.cells[index].cloneNode(true);
        cell.querySelectorAll('[id]').forEach(node=>node.removeAttribute('id'));
        cell.querySelectorAll('[onclick]').forEach(node=>node.removeAttribute('onclick'));
        copy.append(cell);
      }
      previewBody.append(copy);
    }
  }
  function renderHome(){
    if(sysData){
      byId('nc-station-name').textContent=sysData.hostname||'Basisstation';
      byId('nc-home-temp').textContent=sysData.cpu_temp_c!=null?Number(sysData.cpu_temp_c).toFixed(1)+' °C':'—';
      byId('nc-home-temp-sub').textContent=sysData.cpu_pct!=null?'CPU-Auslastung '+sysData.cpu_pct+' %':'Temperatur nicht verfügbar';
      byId('nc-home-uptime').textContent=byId('sysUptime').textContent||'—';
      byId('nc-home-uptime-sub').textContent='Host-Laufzeit';
    }
    const connection=byId('connText').textContent;
    const connected=!!ws&&ws.readyState===WebSocket.OPEN;
    byId('nc-local-dot').className='nc-status-dot'+(connected?' ok':'');
    byId('nc-local-title').textContent=connected?'Telemetrie verbunden':'Telemetrie getrennt';
    byId('nc-local-desc').textContent=byId('health-badge-label').textContent!=='—'?byId('health-badge-label').textContent:connection;
    if(edgeFallbackData){
      const meta=edgeModeMeta(edgeFallbackData);
      byId('nc-core-dot').className='nc-status-dot'+(edgeFallbackData.mode==='online'?' ok':' warn');
      byId('nc-core-title').textContent=meta.label;
      byId('nc-core-desc').textContent=coreReasonText(edgeFallbackData.reason)||meta.title;
    }
    const list=byId('nc-home-events');list.replaceChildren();
    for(const entry of (state.lastHeard||[]).slice(0,4)){
      const node=make('li','');
      node.append(make('time','',entry.ts||'—'),make('span','',String(entry.issi||'—')+' · '+(t('act_'+entry.activity)||entry.activity)+' · '+(entry.dest||'—')));list.append(node);
    }
    if(!list.children.length)list.append(make('li','help-text','Noch keine Aktivität empfangen.'));
  }
  async function loadNeighbors(){
    try{
      const response=await fetch('/api/btsinfo',{credentials:'same-origin'});
      if(!response.ok)throw new Error('HTTP '+response.status);
      const data=await response.json();neighbors=Array.isArray(data.neighbors)?data.neighbors:null;
      byId('nc-neighbor-count').textContent=(data.neighbor_count??neighbors?.length??0)+' Nachbarn';
      const body=byId('nc-neighbor-rows');body.replaceChildren();
      if(!neighbors?.length){const row=make('tr','');const cell=make('td','',neighbors?'Keine Nachbarzellen konfiguriert.':'Nachbarzellendetails sind auf diesem Server noch nicht verfügbar.');cell.colSpan=6;row.append(cell);body.append(row);return;}
      for(const neighbor of neighbors){
        const row=make('tr','');
        const vals=[neighbor.cell_identifier_ca,neighbor.main_carrier_number,[neighbor.mcc??'—',neighbor.mnc??'—'].join(' / '),neighbor.location_area,neighbor.neighbor_cell_synchronized==null?'—':neighbor.neighbor_cell_synchronized?'Ja':'Nein',neighbor.cell_load_ca];
        for(const value of vals)row.append(make('td','',value==null?'—':String(value)));body.append(row);
      }
    }catch(error){byId('nc-neighbor-rows').innerHTML='<tr><td colspan="6">Nachbarzellen konnten nicht geladen werden. Erneut aktualisieren.</td></tr>';}
  }

  // Extend existing render hooks, preserving their implementation and return values.
  function after(name,callback){const original=window[name];if(typeof original!=='function')return;window[name]=function(...args){const result=original.apply(this,args);callback();return result;};}
  after('renderStations',syncRadios);after('renderLastHeard',renderHome);after('renderEdgeFallback',renderHome);after('updateSystemUptime',renderHome);
  after('updateConnState',renderHome);
  const originalPublic=window.enterPublicMode;
  window.enterPublicMode=function(){
    document.body.classList.add('nc-public');
    originalPublic();
    byId('topbar-title').textContent=pages.public[0];byId('nc-page-description').textContent=pages.public[1];
    byId('chip-bs').style.display='none';byId('chip-brew').style.display='none';
    byId('nc-home-radios').replaceChildren();
  };
  byId('nc-refresh').addEventListener('click',()=>showPage(currentPage));
  window.netcoreSessionReady=function(){sessionReady=true;mainNav.querySelectorAll('button').forEach(button=>button.disabled=false);showPage(hiddenIntegrationsVisible&&HIDDEN_INTEGRATIONS.includes(requestedHiddenPage)?requestedHiddenPage:'home');};
  // A cheap state update for timers/disconnects; no additional network requests.
  setInterval(()=>{if(sessionReady&&!document.hidden&&!document.body.classList.contains('nc-public')){renderHome();if(currentPage==='stations'&&selectedRadio)renderRadioDetail();}},2000);
  syncRadios();renderHome();
})();
