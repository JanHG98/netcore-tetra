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
  let neighborRevision=null,neighborDirty=false,neighborBusy=false,neighborConflict=false,neighborEditing=null;
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
  const discoveryLink=make('a','btn btn-sm','Auto Discovery ↗');
  const discoveryUrl=new URL(window.location.href);
  discoveryUrl.protocol='http:';discoveryUrl.port='8321';discoveryUrl.pathname='/';
  discoveryUrl.search='?scan=1';discoveryUrl.hash='';discoveryUrl.username='';discoveryUrl.password='';
  discoveryLink.href=discoveryUrl.href;discoveryLink.target='_blank';discoveryLink.rel='noopener';
  discoveryLink.dataset.netcoreDiscovery='';discoveryLink.hidden=true;
  byId('core-services-card').querySelector('.card-actions').prepend(discoveryLink);
  byId('page-neighbors').innerHTML=`
    <div class="card">
      <div class="card-head"><div><div class="card-title">Konfigurierte Nachbarzellen</div><p class="card-sub">Konfigurationswerte – Verfügbarkeit und Handover werden hier nicht gemessen.</p></div><span class="pill pill-info" id="nc-neighbor-count">–</span></div>
      <div class="card-body nc-neighbor-toolbar"><button type="button" class="btn" id="nc-neighbor-add" disabled>Nachbarzelle anlegen</button><button type="button" class="btn btn-primary" id="nc-neighbor-save" disabled>Speichern und übernehmen</button><button type="button" class="btn" id="nc-neighbor-reload">Neu laden</button></div>
      <div class="card-body nc-neighbor-notice" id="nc-neighbor-status" role="status" aria-live="polite">Nachbarzellen werden geladen.</div>
      <div class="table-wrap"><table><thead><tr><th>Zellkennung</th><th>Hauptträger</th><th>MCC / MNC</th><th>Location Area</th><th>Synchronisiert</th><th>Zelllast</th><th>Bearbeiten</th></tr></thead><tbody id="nc-neighbor-rows"></tbody></table></div>
      <div class="card-body"><p class="help-text">Bis zu 7 Nachbarzellen. Änderungen bleiben als Entwurf erhalten, bis du speicherst. Beim Übernehmen wird der Funkdienst kurz neu gestartet; laufende Rufe werden unterbrochen.</p></div>
    </div>
    <div class="card nc-neighbor-editor" id="nc-neighbor-editor" hidden>
      <div class="card-head"><div class="card-title" id="nc-neighbor-editor-title">Nachbarzelle bearbeiten</div></div>
      <form class="card-body" id="nc-neighbor-form">
        <div class="nc-neighbor-fields" id="nc-neighbor-fields"></div>
        <label class="nc-neighbor-check"><input type="checkbox" id="nc-n-neighbor_cell_synchronized"> Zelle ist synchronisiert</label>
        <details class="nc-neighbor-advanced"><summary>Erweiterte Parameter</summary><div class="nc-neighbor-fields" id="nc-neighbor-advanced-fields"></div><p class="help-text">Leere optionale Felder übernehmen die Standardwerte der Funkkonfiguration. Weitere bestehende Dienstparameter bleiben erhalten.</p></details>
        <div class="config-msg" id="nc-neighbor-form-msg" role="alert"></div>
        <div class="nc-neighbor-toolbar"><button type="submit" class="btn btn-primary">In Entwurf übernehmen</button><button type="button" class="btn" id="nc-neighbor-cancel">Abbrechen</button></div>
      </form>
    </div>`;
  const neighborFields=[
    ['cell_identifier_ca','Zellkennung (CA)',31,true],
    ['main_carrier_number','Hauptträgernummer',4095,true],
    ['mcc','MCC',1023,false],
    ['mnc','MNC',16383,false],
    ['location_area','Location Area',16383,false],
    ['cell_reselection_types_supported','Zellneuauswahl-Typen',3,true],
    ['cell_load_ca','Konfigurierte Zelllast',3,true],
    ['main_carrier_number_extension','Trägernummer-Erweiterung',1023,false,true],
    ['tdma_frame_offset','TDMA-Frame-Offset',63,false,true],
    ['maximum_ms_transmit_power','Maximale MS-Sendeleistung',7,false,true],
    ['minimum_rx_access_level','Mindest-Empfangspegel für Zugang',15,false,true],
    ['subscriber_class','Teilnehmerklasse',65535,false,true],
    ['timeshare_cell_information_or_security_parameters','Timeshare-/Sicherheitsparameter',31,false,true]
  ];
  for(const [key,label,max,required,advanced] of neighborFields){
    const field=make('label','nc-neighbor-field');
    const input=make('input','form-input');input.id='nc-n-'+key;input.type='number';input.min='0';input.max=String(max);input.step='1';input.required=required;
    field.append(make('span','',label+(required?' *':'')),input,make('small','help-text','0–'+max+(required?'':' · optional')));
    byId(advanced?'nc-neighbor-advanced-fields':'nc-neighbor-fields').append(field);
  }
  byId('nc-neighbor-add').addEventListener('click',()=>editNeighbor(null));
  byId('nc-neighbor-reload').addEventListener('click',()=>{
    if(neighborDirty&&!window.confirm('Nicht gespeicherte Nachbarzellen verwerfen und neu laden?'))return;
    loadNeighbors(true);
  });
  byId('nc-neighbor-save').addEventListener('click',saveNeighbors);
  byId('nc-neighbor-cancel').addEventListener('click',closeNeighborEditor);
  byId('nc-neighbor-form').addEventListener('submit',event=>{event.preventDefault();applyNeighborDraft();});
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
  function neighborStatus(message,error=false){
    byId('nc-neighbor-status').textContent=message;
    byId('nc-neighbor-status').classList.toggle('is-error',error);
  }
  function renderNeighbors(){
    const body=byId('nc-neighbor-rows');body.replaceChildren();
    byId('nc-neighbor-count').textContent=(neighbors?.length??0)+' Nachbarn'+(neighborDirty?' · Entwurf':'');
    byId('nc-neighbor-add').disabled=neighborBusy||!neighbors||neighbors.length>=7||neighborEditing!==null;
    byId('nc-neighbor-save').disabled=neighborBusy||!neighborDirty||neighborConflict||!neighborRevision||neighborEditing!==null;
    byId('nc-neighbor-reload').disabled=neighborBusy;
    if(!neighbors?.length){const row=make('tr','');const cell=make('td','help-text',neighbors?'Keine Nachbarzellen konfiguriert.':'Nachbarzellen werden geladen.');cell.colSpan=7;row.append(cell);body.append(row);return;}
    neighbors.forEach((neighbor,index)=>{
      const row=make('tr','');
      const vals=[neighbor.cell_identifier_ca,neighbor.main_carrier_number,[neighbor.mcc??'–',neighbor.mnc??'–'].join(' / '),neighbor.location_area,neighbor.neighbor_cell_synchronized?'Ja':'Nein',neighbor.cell_load_ca];
      for(const value of vals)row.append(make('td','',value==null?'–':String(value)));
      const actions=make('td','nc-neighbor-row-actions');
      const edit=make('button','btn btn-sm','Bearbeiten');edit.type='button';edit.disabled=neighborBusy||neighborEditing!==null;edit.setAttribute('aria-label','Zellkennung '+neighbor.cell_identifier_ca+' bearbeiten');edit.addEventListener('click',()=>editNeighbor(index));
      const remove=make('button','btn btn-sm btn-warn','Löschen');remove.type='button';remove.disabled=neighborBusy||neighborEditing!==null;remove.setAttribute('aria-label','Zellkennung '+neighbor.cell_identifier_ca+' löschen');remove.addEventListener('click',()=>{
        neighbors.splice(index,1);neighborDirty=true;closeNeighborEditor();renderNeighbors();neighborStatus('Nachbarzelle aus dem Entwurf gelöscht. Zum Übernehmen speichern.');
      });
      actions.append(edit,remove);row.append(actions);body.append(row);
    });
  }
  function closeNeighborEditor(){
    neighborEditing=null;byId('nc-neighbor-editor').hidden=true;byId('nc-neighbor-form-msg').textContent='';renderNeighbors();
  }
  function editNeighbor(index){
    if(neighborBusy||!neighbors)return;
    neighborEditing=index===null?-1:index;
    const row=index===null?{cell_identifier_ca:Array.from({length:32},(_,i)=>i).find(i=>!neighbors.some(n=>n.cell_identifier_ca===i)),cell_reselection_types_supported:0,cell_load_ca:0}:neighbors[index];
    for(const [key] of neighborFields)byId('nc-n-'+key).value=row[key]??'';
    byId('nc-n-neighbor_cell_synchronized').checked=row.neighbor_cell_synchronized===true;
    byId('nc-neighbor-form-msg').textContent='';
    byId('nc-neighbor-editor-title').textContent=index===null?'Nachbarzelle anlegen':'Nachbarzelle bearbeiten';
    byId('nc-neighbor-editor').hidden=false;renderNeighbors();byId('nc-n-cell_identifier_ca').focus();
    byId('nc-neighbor-editor').scrollIntoView({block:'nearest',behavior:'smooth'});
  }
  function applyNeighborDraft(){
    if(neighborBusy||neighborEditing===null||!byId('nc-neighbor-form').reportValidity())return;
    const row=neighborEditing<0?{}:JSON.parse(JSON.stringify(neighbors[neighborEditing]));
    for(const [key,label,max,required] of neighborFields){
      const value=byId('nc-n-'+key).value.trim();
      if(!value){if(required)return;delete row[key];continue;}
      const number=Number(value);
      if(!Number.isInteger(number)||number<0||number>max){byId('nc-neighbor-form-msg').textContent=label+': Ganze Zahl von 0 bis '+max+' erforderlich.';return;}
      row[key]=number;
    }
    if(neighbors.some((n,i)=>i!==neighborEditing&&n.cell_identifier_ca===row.cell_identifier_ca)){byId('nc-neighbor-form-msg').textContent='Diese Zellkennung ist bereits vorhanden. Jede CA-Kennung darf nur einmal vorkommen.';byId('nc-n-cell_identifier_ca').focus();return;}
    row.neighbor_cell_synchronized=byId('nc-n-neighbor_cell_synchronized').checked;
    if(neighborEditing<0)neighbors.push(row);else neighbors[neighborEditing]=row;
    neighborDirty=true;closeNeighborEditor();neighborStatus('Entwurf geändert. Zum Übernehmen speichern.');
  }
  async function loadNeighbors(force=false){
    if(neighborBusy||(!force&&(neighborDirty||neighborEditing!==null)))return;
    neighborBusy=true;renderNeighbors();neighborStatus('Nachbarzellen werden geladen.');
    try{
      const response=await fetch('/api/neighbours',{credentials:'same-origin',cache:'no-store'});
      if(!response.ok)throw new Error('HTTP '+response.status);
      const data=await response.json();
      if(!Array.isArray(data.neighbors)||typeof data.revision!=='string')throw new Error('Ungültige Antwort');
      neighbors=JSON.parse(JSON.stringify(data.neighbors));neighborRevision=data.revision;neighborDirty=false;neighborConflict=false;
      neighborEditing=null;byId('nc-neighbor-editor').hidden=true;
      neighborStatus(data.restart_required?'Gespeicherte Nachbarzellen weichen noch vom laufenden Funkdienst ab.':'Nachbarzellen geladen. Du kannst sie anlegen, bearbeiten und löschen.');
    }catch(error){neighborStatus('Nachbarzellen konnten nicht geladen werden. Bitte erneut laden. ('+error.message+')',true);}
    finally{neighborBusy=false;renderNeighbors();}
  }
  async function saveNeighbors(){
    if(neighborBusy||!neighborDirty||neighborConflict||!neighborRevision||neighborEditing!==null)return;
    neighborBusy=true;renderNeighbors();neighborStatus('Nachbarzellen werden gespeichert.');
    try{
      const response=await fetch('/api/neighbours',{method:'POST',credentials:'same-origin',headers:{'Content-Type':'application/json'},body:JSON.stringify({neighbors,revision:neighborRevision})});
      const data=await response.json();
      if(response.status===409){neighborConflict=true;neighborStatus('Die Konfiguration wurde inzwischen geändert. Lade die Nachbarzellen neu und prüfe deine Änderungen.',true);return;}
      if(!response.ok||data.ok===false)throw new Error(typeof data.error==='string'?data.error:'HTTP '+response.status);
      neighborDirty=false;neighborRevision=null;neighborStatus(data.restarting?'Gespeichert. Der Funkdienst startet kurz neu und übernimmt die Nachbarzellen.':'Nachbarzellen gespeichert.');
      if(data.restarting)setTimeout(()=>loadNeighbors(true),4000);else await loadNeighborsAfterSave();
    }catch(error){neighborStatus('Speichern fehlgeschlagen: '+error.message+'. Der Entwurf bleibt erhalten.',true);}
    finally{neighborBusy=false;renderNeighbors();}
  }
  async function loadNeighborsAfterSave(){neighborBusy=false;await loadNeighbors(true);}

  // Extend existing render hooks, preserving their implementation and return values.
  function after(name,callback){const original=window[name];if(typeof original!=='function')return;window[name]=function(...args){const result=original.apply(this,args);callback();return result;};}
  after('renderStations',syncRadios);after('renderLastHeard',renderHome);after('renderEdgeFallback',renderHome);after('updateSystemUptime',renderHome);
  after('updateConnState',renderHome);
  const originalPublic=window.enterPublicMode;
  window.enterPublicMode=function(){
    document.body.classList.add('nc-public');
    discoveryLink.hidden=true;
    originalPublic();
    byId('topbar-title').textContent=pages.public[0];byId('nc-page-description').textContent=pages.public[1];
    byId('chip-bs').style.display='none';byId('chip-brew').style.display='none';
    byId('nc-home-radios').replaceChildren();
  };
  byId('nc-refresh').addEventListener('click',()=>showPage(currentPage));
  window.netcoreSessionReady=function(){sessionReady=true;discoveryLink.hidden=false;mainNav.querySelectorAll('button').forEach(button=>button.disabled=false);showPage(hiddenIntegrationsVisible&&HIDDEN_INTEGRATIONS.includes(requestedHiddenPage)?requestedHiddenPage:'home');};
  // A cheap state update for timers/disconnects; no additional network requests.
  setInterval(()=>{if(sessionReady&&!document.hidden&&!document.body.classList.contains('nc-public')){renderHome();if(currentPage==='stations'&&selectedRadio)renderRadioDetail();}},2000);
  syncRadios();renderHome();
})();

