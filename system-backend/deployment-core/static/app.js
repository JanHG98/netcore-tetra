'use strict';
const $ = id => document.getElementById(id);
let status = null, catalog = [], jobs = [], selectedJob = '', currentPlan = null, initialized = false, busy = false;
let imageState = null, selectedImageJob = '', imageSubmitting = false;
let savedProfiles = [], profileDeleting = '', profileRevision = 0;
let discoveryRevision = 0, discoveryMutating = '';
let bulkPlan = null, updateBatches = [], selectedBatch = '', bulkPlanning = false, bulkStarting = false, bulkRevision = 0;
let selectedJobDetail = null, selectedJobLoading = false;
const labels = {partial:'Teilerfolg',uncertain:'Ungeklärt',not_attempted:'Nicht gestartet',planned:'Vorgesehen',queued:'Wartet',running:'Läuft',succeeded:'Erfolgreich',failed:'Fehlgeschlagen',interrupted:'Unterbrochen'};
const esc = value => String(value ?? '').replace(/[&<>"']/g, c => ({'&':'&amp;','<':'&lt;','>':'&gt;','"':'&quot;',"'":'&#39;'}[c]));
const short = value => value ? value.slice(0, 8) : 'Unbekannt';
function notice(message, error = false) { $('notice').textContent = message; $('notice').className = error ? 'error' : ''; $('notice').hidden = false; }
async function api(path, data) {
  const options = data === undefined ? {} : {method:'POST', headers:{'Content-Type':'application/json'}, body:JSON.stringify(data)};
  const response = await fetch(path, options);
  const result = await response.json();
  if (!response.ok) { const error = new Error(result.error || `HTTP ${response.status}`); error.status = response.status; error.result = result; throw error; }
  return result;
}
function guarded(fn) { return async event => { if (event) event.preventDefault(); try { await fn(event); } catch (error) { notice(error.message, true); } }; }
function selectOptions(el, rows, label, value) {
  const previous = el.value;
  const fragment = document.createDocumentFragment();
  rows.forEach(row => { const option = document.createElement('option'); option.value = value(row); option.textContent = label(row); fragment.appendChild(option); });
  el.replaceChildren(fragment);
  if ([...el.options].some(o => o.value === previous)) el.value = previous;
}
function discoveredNodes() {
  const nodes = [...(status?.peers || [])];
  if (status?.role === 'agent') nodes.unshift({node_id:status.node_id, role:'agent', agent_url:location.origin, online:true, services:status.services || []});
  return nodes;
}

function serviceLink(service, node) {
  try {
    const url = new URL(service.url || node.agent_url);
    if (!['http:', 'https:'].includes(url.protocol)) return '';
    if (!service.url) {
      if (!service.port) return '';
      url.port = String(service.port);
      url.pathname = '/'; url.search = ''; url.hash = '';
    }
    return url.href;
  } catch (_) { return ''; }
}

function rememberedFocus(container, action) {
  const active = document.activeElement;
  const keys = ['removeNode', 'restoreNode', 'bulkJob'];
  const key = container.contains(active) && keys.find(name => active.dataset[name]);
  const value = key && active.dataset[key];
  action();
  if (key) {
    const attribute = key.replace(/[A-Z]/g, letter => '-' + letter.toLowerCase());
    container.querySelector(`[data-${attribute}="${CSS.escape(value)}"]:not(:disabled)`)?.focus({preventScroll:true});
  }
}

function renderForgotten() {
  const forgotten = status?.forgotten || [];
  $('removed-nodes').hidden = forgotten.length === 0;
  $('removed-node-count').textContent = forgotten.length;
  rememberedFocus($('removed-node-list'), () => {
    $('removed-node-list').innerHTML = forgotten.map(node => `<div class="forgotten-node"><div><strong>${esc(node.node_id)}</strong><small>${esc(node.agent_url || '')}</small></div><button type="button" class="secondary" data-restore-node="${esc(node.node_id)}" ${discoveryMutating?'disabled':''}>${discoveryMutating==='restore:'+node.node_id?'Wird wieder zugelassen …':'Wiederherstellen'}</button></div>`).join('');
  });
}

function renderNodes() {
  if (!status) return;
  const nodes = discoveredNodes();
  const services = nodes.flatMap(node => (node.services || []).map(service => ({...service, node})));
  $('host-count').textContent = nodes.filter(node => node.online && node.role !== 'controller').length;
  $('service-count').textContent = services.filter(service => service.ready && service.node.online).length;
  $('drift-count').textContent = status.desired.commit ? services.filter(service => service.commit && service.commit !== status.desired.commit).length : '-';
  $('commit').textContent = short(status.desired.commit);
  $('target-ref').textContent = status.desired.ref || status.settings.ref;
  $('instance').textContent = `${status.node_id} · ${status.environment} · ${status.role === 'controller' ? 'Deployment Core' : 'Discovery-Agent'}`;
  const query = $('filter').value.trim().toLowerCase();
  const kind = $('filter-kind').value, role = $('filter-service').value, availability = $('filter-online').value;
  const allRows = nodes.flatMap(node => ((node.services || []).length ? node.services : [{name:'Agent bereit',ready:node.online,commit:'',url:''}]).map(service => ({...service,node})));
  const rows = allRows.filter(service =>
    (!query || `${service.name} ${service.node.node_id} ${service.node.agent_url}`.toLowerCase().includes(query)) &&
    (kind === 'all' || (kind === 'tbs' ? service.name === 'tbs' : service.name !== 'tbs')) &&
    (!role || service.name === role) &&
    (availability === 'all' || Boolean(service.node.online) === (availability === 'online')));
  $('nodes-count').textContent = `${rows.length} von ${allRows.length} Einträgen`;
  const shown = new Set();
  rememberedFocus($('nodes'), () => {
    $('nodes').innerHTML = rows.map(service => {
      const online = service.node.online && service.ready;
      const drift = service.commit && status.desired.commit && service.commit !== status.desired.commit;
      const url = serviceLink(service, service.node);
      const first = !shown.has(service.node.node_id); shown.add(service.node.node_id);
      const removable = first && service.node.node_id !== status.node_id && service.node.role !== 'controller';
      let discoveryUrl = '';
      try {
        const address = new URL(service.node.agent_url);
        if (['http:', 'https:'].includes(address.protocol)) discoveryUrl = new URL('/?scan=1', address).href;
      } catch (_) {}
      return `<tr class="${first?'node-first':''}"><td><strong>${esc(service.name)}</strong><small>${esc(service.node.node_id)}${service.name==='tbs'?' · Basisstation':''}</small></td><td><span class="badge ${online?'ok':'warn'}">${online?'Bereit':service.node.online?'Nicht bereit':'Offline'}</span></td><td><span class="${drift?'badge warn':''}">${esc(short(service.commit))}</span>${drift?' <small>Abweichend</small>':''}</td><td class="node-links">${url?`<a href="${esc(url)}" target="_blank" rel="noopener">WebUI ↗</a>`:''}${discoveryUrl?`<a href="${esc(discoveryUrl)}" target="_blank" rel="noopener">Discovery ↗</a>`:''}</td><td>${removable?`<button type="button" class="quiet peer-remove" data-remove-node="${esc(service.node.node_id)}" aria-label="Discovery-Eintrag für ${esc(service.node.node_id)} löschen" ${discoveryMutating?'disabled':''}>${discoveryMutating==='remove:'+service.node.node_id?'Wird gelöscht …':'Löschen'}</button>`:''}</td></tr>`;
    }).join('');
  });
  const forgotten = status.forgotten || [];
  $('empty').hidden = nodes.length > 0 || forgotten.length > 0;
  $('filter-empty').hidden = rows.length > 0 || (!nodes.length && !forgotten.length);
  $('filter-empty-text').textContent = nodes.length ? 'Keine Einträge passen zu diesen Filtern.' : 'Alle Discovery-Einträge sind ausgeblendet. Du kannst sie unten wiederherstellen.';
  const conflicts = Object.entries(status.conflicts || {}).map(([name, values]) => `${name}: ${values.join(', ')}`);
  const notes = [...conflicts.map(conflict => 'Mehrdeutige Zuordnung – ' + conflict), ...(status.multicast_error ? ['Multicast: '+status.multicast_error+'. Feste Gegenstellen bleiben aktiv.'] : [])];
  $('discovery-note').hidden = notes.length === 0;
  $('discovery-note').textContent = notes.join(' · ');
  selectOptions($('target-node'), (status.peers || []).filter(node => node.online && node.role === 'agent'), node => node.node_id, node => node.node_id);
  renderForgotten();
}
function batchJob(batch, target) {
  return (batch.jobs || []).find(job => target.job_id ? job.id === target.job_id : job.request?.node_id === target.node_id && job.request?.service === target.service);
}

function targetState(batch, target) {
  const job = batchJob(batch, target);
  if (job?.result?.remote_uncertain && !['queued','running'].includes(job.status)) return 'uncertain';
  if (job?.result?.not_attempted && !['queued','running'].includes(job.status)) return 'not_attempted';
  return job?.status || (batch.status === 'queued' ? 'queued' : 'not_attempted');
}

function stateClass(value) {
  return value === 'succeeded' ? 'ok' : ['failed','interrupted','uncertain'].includes(value) ? 'bad' : 'warn';
}

function skipReason(value) {
  const reasons = {offline:'Host offline',controller:'Controller wird separat aktualisiert',unsupported_service:'Dienst wird nicht unterstützt',not_installed:'Dienst ist nicht installiert',discovery:'Discovery-Agent wird separat aktualisiert'};
  return reasons[value] || value || 'Kein Update-Ziel';
}

function renderSkipped(container, items) {
  container.innerHTML = (items || []).map(item => `<li><strong>${esc(item.node_id || 'Unbekannter Host')}${item.service?' · '+esc(item.service):''}</strong><span>${esc(skipReason(item.reason))}</span></li>`).join('');
}

function findJob(id) {
  const full = [...jobs, selectedJobDetail].filter(job => job?.id === id && typeof job.log === 'string');
  full.sort((left, right) => Number(right.updated || 0) - Number(left.updated || 0));
  return full[0] || updateBatches.flatMap(batch => batch.jobs || []).find(job => job.id === id);
}

function renderBulk() {
  const targets = bulkPlan?.targets || [];
  $('bulk-plan').disabled = bulkPlanning || bulkStarting;
  $('bulk-plan').textContent = bulkPlanning ? 'Vorschau wird vorbereitet …' : bulkPlan ? 'Vorschau erneuern' : 'Alle Updates anzeigen';
  $('bulk-preview').hidden = !bulkPlan;
  $('bulk-execute').disabled = bulkStarting || bulkPlanning || !targets.length || updateBatches.some(batch => ['queued','running'].includes(batch.status));
  $('bulk-cancel').disabled = bulkStarting;
  $('bulk-execute').textContent = bulkStarting ? 'Lauf wird angelegt …' : `${targets.length} ${targets.length===1?'Dienst':'Dienste'} jetzt aktualisieren`;
  if (bulkPlan) {
    const hosts = new Set(targets.map(target => target.node_id)).size;
    $('bulk-plan-summary').textContent = `${targets.length} ${targets.length===1?'Dienst':'Dienste'} auf ${hosts} ${hosts===1?'Host':'Hosts'} · ${(bulkPlan.skipped || []).length} ausgelassen`;
    $('bulk-commit').textContent = bulkPlan.commit;
    $('bulk-targets').innerHTML = targets.map(target => `<tr><td><strong>${esc(target.service)}</strong></td><td>${esc(target.node_id)}</td><td>${esc(short(target.installed_commit))}</td><td><span class="badge">Vorgesehen</span></td></tr>`).join('');
    $('bulk-no-targets').hidden = targets.length > 0;
    $('bulk-skipped').hidden = !(bulkPlan.skipped || []).length;
    $('bulk-skipped-count').textContent = (bulkPlan.skipped || []).length;
    renderSkipped($('bulk-skipped-list'), bulkPlan.skipped);
  }
  if (!selectedBatch && updateBatches.length) selectedBatch = updateBatches[0].id;
  const batch = updateBatches.find(item => item.id === selectedBatch);
  $('bulk-status').hidden = !batch;
  if (!batch) return;
  selectOptions($('bulk-batch'), updateBatches, item => `${new Date(item.created*1000).toLocaleString('de-DE')} · ${short(item.commit)} · ${labels[item.status] || item.status}`, item => item.id);
  $('bulk-batch').value = batch.id;
  $('bulk-status-label').textContent = labels[batch.status] || batch.status;
  $('bulk-status-label').className = 'badge ' + stateClass(batch.status);
  $('bulk-status-commit').textContent = short(batch.commit);
  const batchTargets = batch.targets || [];
  const states = batchTargets.map(target => targetState(batch, target));
  const succeeded = states.filter(value => value === 'succeeded').length;
  const failed = states.filter(value => ['failed','interrupted'].includes(value)).length;
  const uncertain = states.filter(value => value === 'uncertain').length;
  const active = states.filter(value => ['queued','running'].includes(value)).length;
  const pending = states.filter(value => value === 'not_attempted').length;
  $('bulk-status-summary').textContent = `${succeeded}/${batchTargets.length} erfolgreich${active?' · '+active+' offen':''}${failed?' · '+failed+' fehlgeschlagen/unterbrochen':''}${uncertain?' · '+uncertain+' ungeklärt':''}${pending?' · '+pending+' nicht gestartet':''}`;
  $('bulk-progress').max = Math.max(1, batchTargets.length);
  $('bulk-progress').value = batchTargets.filter(target => {
    const job = batchJob(batch, target);
    return job && !['queued','running'].includes(job.status);
  }).length;
  rememberedFocus($('bulk-status-body'), () => {
    $('bulk-status-body').innerHTML = batchTargets.map(target => {
      const job = batchJob(batch, target), state = targetState(batch, target);
      return `<tr data-node-id="${esc(target.node_id)}" data-service="${esc(target.service)}" data-update-state="${esc(state)}"><td><strong>${esc(target.service)}</strong><small>${esc(target.node_id)}</small></td><td><span class="badge ${stateClass(state)}">${esc(labels[state] || state)}</span></td><td>${esc(short(job?.result?.commit || batch.commit))}</td><td>${job?`<button type="button" class="quiet" data-bulk-job="${esc(job.id)}">Protokoll</button>`:'Noch kein Auftrag'}</td></tr>`;
    }).join('');
  });
  $('bulk-run-skipped').hidden = !(batch.skipped || []).length;
  $('bulk-run-skipped-count').textContent = (batch.skipped || []).length;
  renderSkipped($('bulk-run-skipped-list'), batch.skipped);
}

function renderJobs() {
  $('job-list').innerHTML = jobs.length ? jobs.map(job => `<button class="job ${selectedJob===job.id?'selected':''}" data-job="${esc(job.id)}"><b>${esc(job.request.service || 'Git-Stand prüfen')}</b><small>${esc(job.request.node_id || job.request.ref || job.request.action || '')} · ${new Date(job.created*1000).toLocaleTimeString('de-DE')}</small><span class="badge ${stateClass(job.status)}">${labels[job.status] || esc(job.status)}</span></button>`).join('') : '<p class="empty">Noch keine Aufträge.</p>';
  const job = findJob(selectedJob);
  if (job) { $('log-title').textContent = `${labels[job.status] || job.status} · ${job.request.node_id || job.id.slice(0,8)}${job.request.service?' · '+job.request.service:''}`; $('log').textContent = selectedJobLoading ? 'Protokoll wird geladen …' : job.log || (job.status==='queued'?'Auftrag wartet auf Ausführung.':'Noch keine Logausgabe.'); }
}
const gib = bytes => `${(bytes / 1024 ** 3).toLocaleString('de-DE', {maximumFractionDigits:1})} GiB`;
const desktopLabels = {kde:'KDE Plasma', xfce:'XFCE', gnome:'GNOME', pios:'Raspberry Pi OS Desktop', none:'Ohne Desktop (Lite)'};
const desktopLabel = desktop => desktopLabels[desktop ?? 'none'] || desktop;
function renderDesktopLogin() {
  const graphical = $('image-desktop').value !== 'none';
  $('image-password').required = graphical;
  $('image-password-note').textContent = graphical ? `Für die Anmeldung an ${desktopLabel($('image-desktop').value)}, SSH und Konsole des Pi. Ein SSH-Key allein genügt nicht für die grafische Anmeldung.` : 'Für SSH und Konsole des Pi. Ohne Desktop genügt auch ein SSH-Public-Key.';
}
let vpnNeedsCredentials = false;
function renderVpnCredentials() {
  const selected = Boolean($('image-vpn').files[0]);
  const username = $('image-vpn-username'), password = $('image-vpn-password');
  $('image-vpn-credentials').hidden = !selected;
  username.disabled = password.disabled = !selected;
  username.required = selected && (vpnNeedsCredentials || Boolean(password.value));
  password.required = selected && (vpnNeedsCredentials || Boolean(username.value));
  $('image-vpn-auth-note').textContent = vpnNeedsCredentials ? 'Dieses VPN-Profil benötigt Benutzer und Passwort für die Verbindung.' : 'Falls das VPN Benutzer und Passwort benötigt, beide Felder ausfüllen. Bei reiner Zertifikatsanmeldung oder eingebetteten Zugangsdaten können sie leer bleiben.';
}
$('image-vpn').onchange = guarded(async()=>{
  const file = $('image-vpn').files[0];
  vpnNeedsCredentials = Boolean(file);
  if (!file) { $('image-vpn-username').value=''; $('image-vpn-password').value=''; }
  renderVpnCredentials();
  const content = file ? await file.text() : '';
  if ($('image-vpn').files[0] !== file) return;
  const hasAuth = /^\s*(?:--)?auth-user-pass(?:\s|$)/m.test(content) || /^\s*<auth-user-pass>[ \t]*$/m.test(content);
  const completeInlineAuth = /^\s*<auth-user-pass>[ \t]*\r?\n[^\r\n]+\r?\n[^\r\n]+\r?\n[ \t]*<\/auth-user-pass>[ \t]*$/m.test(content);
  vpnNeedsCredentials = hasAuth && !completeInlineAuth;
  renderVpnCredentials();
});
$('image-vpn-username').oninput = $('image-vpn-password').oninput = renderVpnCredentials;
function renderProfileList() {
  $('profile-list').innerHTML = savedProfiles.length ? savedProfiles.map(p=>`<article class="saved-profile"><a href="/bootstrap.sh?profile=${encodeURIComponent(p.name)}" download="${esc(p.name)}-bootstrap.sh">${esc(p.name)} · Bootstrap ↓</a><button type="button" class="quiet profile-remove" data-remove-profile="${esc(p.name)}" ${profileDeleting?'disabled':''}>${profileDeleting===p.name?'Wird gelöscht …':'Profil löschen'}</button></article>`).join('') : '<div class="profile-empty"><p>Noch keine TBS-Profile gespeichert.</p><a href="#profile-form">TBS-Profil anlegen ↑</a></div>';
}
function renderProfiles(profiles) {
  savedProfiles = profiles;
  selectOptions($('target-profile'), [{name:''},...profiles], p=>p.name || 'Keins', p=>p.name);
  selectOptions($('image-profile'), [{name:''},...profiles], p=>p.name || 'Profil auswählen', p=>p.name);
  if (!$('image-profile').value) $('image-hostname').value = '';
  else if (!$('image-hostname').value) $('image-hostname').value = $('image-profile').value.toLowerCase().replace(/[^a-z0-9-]/g,'-').slice(0,63).replace(/-+$/,'');
  renderProfileList();
  renderImages();
}
function renderImages() {
  const state = imageState;
  if (!state) return;
  $('builder-state').textContent = state.available ? '● Imagebuilder bereit' : 'Imagebuilder nicht verfügbar';
  $('builder-state').className = 'badge ' + (state.available ? 'ok' : 'warn');
  $('builder-note').hidden = Boolean(state.available);
  $('builder-note').textContent = state.error || '';
  $('image-space').textContent = state.available ? gib(state.free_bytes) : '—';
  $('image-build').disabled = !state.available || !$('image-profile').value || imageSubmitting;
  const builds = state.jobs || [];
  if (!selectedImageJob && builds.length) selectedImageJob = builds[0].id;
  $('image-job-list').innerHTML = builds.length ? builds.map(j => `<button class="job ${selectedImageJob===j.id?'selected':''}" data-image-job="${esc(j.id)}"><b>${esc(j.request.profile?.name || 'Pi-Image')}</b><small>${esc(short(j.request.commit))} · ${esc(desktopLabel(j.request.desktop))} · ${new Date(j.created*1000).toLocaleString('de-DE')}</small><span class="badge ${j.status==='succeeded'?'ok':j.status==='failed'?'bad':'warn'}">${labels[j.status] || esc(j.status)}</span></button>`).join('') : '<p class="empty">Dein erster Image-Build beginnt hier.</p>';
  const job = builds.find(j => j.id === selectedImageJob);
  if (job) { $('image-log-title').textContent = `${labels[job.status] || job.status} · ${job.request.profile?.name || 'Pi-Image'}`; $('image-log').textContent = job.log || 'Der Build wartet auf einen freien Platz.'; }
  const artifacts = state.artifacts || [];
  $('image-count').textContent = `${artifacts.length} ${artifacts.length === 1 ? 'Image' : 'Images'} verfügbar`;
  $('image-artifacts').innerHTML = artifacts.map(a => `<article class="panel image-card"><div><span class="badge ok">Download bereit</span><h3>${esc(a.profile.name)}</h3><p>${esc(short(a.commit))} · ${esc(desktopLabel(a.desktop))} · ${gib(a.size_bytes)} Download · ${gib(a.uncompressed_bytes)} auf SD</p><p>${a.sip_setup==='configured'?'SIP eingerichtet':a.sip_setup==='remote_configured'?'Externer SIP-Zugang übernommen':'SIP: Einrichtung erforderlich'}</p><small>${new Date(a.created*1000).toLocaleString('de-DE')} · Boot-/Funkprüfung auf dem Pi noch offen</small></div><div class="image-actions"><a class="button" href="/api/v1/images/${esc(a.id)}/image" download>Image ↓</a><a href="/api/v1/images/${esc(a.id)}/sha256" download>SHA256</a><a href="/api/v1/images/${esc(a.id)}/manifest" download>Manifest</a><button class="quiet" data-remove-image="${esc(a.id)}">Löschen</button></div></article>`).join('');
}
async function refresh() {
  if (busy) return;
  busy = true;
  try {
    const requestedProfileRevision = profileRevision;
    const requestedDiscoveryRevision = discoveryRevision;
    const requestedBulkRevision = bulkRevision;
    const [freshStatus, freshJobs, profiles] = await Promise.all([api('/api/v1/status'),api('/api/v1/jobs'),api('/api/v1/profiles')]);
    if (!status || requestedDiscoveryRevision === discoveryRevision) status = freshStatus;
    jobs = freshJobs;
    if (selectedJob && !jobs.some(job => job.id === selectedJob) && !selectedJobLoading) {
      const requestedJob = selectedJob;
      const detail = await api('/api/v1/jobs/' + encodeURIComponent(requestedJob));
      if (selectedJob === requestedJob) selectedJobDetail = detail;
    }
    document.body.classList.toggle('agent', status.role === 'agent');
    $('connection').textContent = '✓ Verbunden'; $('connection').className = 'badge ok';
    if (!initialized) {
      $('settings-ref').value = status.settings.ref; $('deploy-ref').value = status.settings.ref;
      $('image-controller').value = status.advertise_url || location.origin;
      $('seeds').value = status.settings.seeds.join('\n'); $('bindings').value = Object.entries(status.settings.bindings).map(([key,value])=>`${key}=${value}`).join('\n'); initialized = true;
    }
    renderTemplateState();
    if (requestedProfileRevision === profileRevision) renderProfiles(profiles);
    renderNodes(); renderJobs();
    if (status.role === 'controller') {
      const [images, batches] = await Promise.all([api('/api/v1/images'),api('/api/v1/updates')]);
      imageState = images;
      if (requestedBulkRevision === bulkRevision) updateBatches = batches;
      renderImages(); renderBulk(); renderJobs();
    }
  } catch (error) { $('connection').textContent = 'Verbindung verloren'; $('connection').className = 'badge bad'; notice(error.message,true); }
  finally { busy = false; }
}

async function reconcileDiscovery() {
  const revision = discoveryRevision;
  const fresh = await api('/api/v1/status');
  if (revision === discoveryRevision) { status = fresh; renderNodes(); }
}

async function changePeer(action, nodeId) {
  if (discoveryMutating) return;
  const question = action === 'remove'
    ? `Discovery-Eintrag für "${nodeId}" löschen? Der Host und seine Dienste laufen weiter. Der Eintrag bleibt bis zur Wiederherstellung ausgeblendet; feste Dienstzuordnungen bleiben bestehen.`
    : `Discovery-Eintrag für "${nodeId}" wieder zulassen? Ein neuer Suchlauf lädt den aktuellen Stand des Hosts.`;
  if (!confirm(question)) return;
  discoveryMutating = action + ':' + nodeId;
  discoveryRevision++;
  renderNodes();
  let message = '', failed = false;
  try {
    const result = await api('/api/v1/peers/' + action, {node_id:nodeId});
    discoveryRevision++;
    message = action === 'remove' ? `Discovery-Eintrag "${nodeId}" gelöscht. Der Host läuft weiter.` : `Discovery-Eintrag "${nodeId}" wieder zugelassen. Erreichbare Hosts erscheinen nach dem Suchlauf.`;
    if (result.retained_bindings?.length) message += ` Feste Zuordnungen bleiben erhalten und sind bis zur Wiederherstellung ungeklärt: ${result.retained_bindings.join(', ')}.`;
  } catch (error) {
    failed = true; discoveryRevision++;
    if (error.result?.inventory_change_applied === true) {
      message = `Die Discovery-Änderung wurde übernommen; dauerhafte Speicherung oder Cache-Abgleich konnten nicht bestätigt werden. ${error.message}`;
    } else if (error.result?.inventory_change_applied === null || !error.status || error.status >= 500) {
      message = `Die Rückmeldung zur Discovery-Änderung ist unklar. Der aktuelle Stand wird neu geladen. ${error.message}`;
    } else message = error.message;
  }
  try { await reconcileDiscovery(); }
  catch (error) { failed = true; message += ` Der aktuelle Discovery-Stand konnte nicht geladen werden: ${error.message}`; }
  finally { discoveryMutating = ''; renderNodes(); notice(message, failed); }
}

function configureUpdateControls() {
  for (const id of ['filter-kind','filter-service','filter-online']) $(id).onchange = () => renderNodes();
  $('filter-reset').onclick = () => {
    $('filter').value = ''; $('filter-kind').value = 'all'; $('filter-service').value = ''; $('filter-online').value = 'all'; renderNodes(); $('filter').focus();
  };
  $('nodes').onclick = event => {
    const button = event.target.closest('[data-remove-node]');
    if (button) changePeer('remove', button.dataset.removeNode);
  };
  $('removed-node-list').onclick = event => {
    const button = event.target.closest('[data-restore-node]');
    if (button) changePeer('restore', button.dataset.restoreNode);
  };
  $('bulk-plan').onclick = guarded(async () => {
    if (bulkPlanning || bulkStarting) return;
    bulkPlanning = true; bulkPlan = null; renderBulk();
    try { bulkPlan = await api('/api/v1/updates/plan', {}); }
    finally { bulkPlanning = false; renderBulk(); }
  });
  $('bulk-cancel').onclick = () => { if (!bulkStarting) { bulkPlan = null; renderBulk(); } };
  $('bulk-execute').onclick = async () => {
    if (!bulkPlan || bulkStarting || bulkPlanning) return;
    const planId = bulkPlan.plan_id;
    bulkStarting = true; renderBulk();
    try {
      const batch = await api('/api/v1/updates/run', {plan_id:planId,confirm_restart:true});
      bulkRevision++; updateBatches = [batch,...updateBatches.filter(item => item.id !== batch.id)];
      selectedBatch = batch.id; bulkPlan = null;
      notice('Sammelupdate gestartet. Der Status je Dienst aktualisiert sich automatisch.');
    } catch (error) {
      let message = error.message;
      // Reconcile an ambiguous response, but never send a second Run request.
      try {
        const accepted = await api('/api/v1/updates/' + encodeURIComponent(planId));
        if (accepted.status !== 'planned') {
          bulkRevision++; updateBatches = [accepted,...updateBatches.filter(batch => batch.id !== accepted.id)];
          selectedBatch = accepted.id; bulkPlan = null; message = `Der Sammellauf ist bereits angelegt. Sein aktueller Stand wird angezeigt. ${message}`;
        }
      } catch (refreshError) { message += ` Der Laufstatus konnte nicht geladen werden: ${refreshError.message}`; }
      notice(message, true);
    } finally { bulkStarting = false; renderBulk(); renderJobs(); }
  };
  $('bulk-batch').onchange = () => { selectedBatch = $('bulk-batch').value; renderBulk(); };
  $('bulk-status-body').onclick = guarded(async event => {
    const button = event.target.closest('[data-bulk-job]');
    if (!button) return;
    const requestedJob = button.dataset.bulkJob;
    selectedJob = requestedJob; selectedJobLoading = true; renderJobs(); location.hash = 'jobs';
    try {
      const detail = await api('/api/v1/jobs/' + encodeURIComponent(requestedJob));
      if (selectedJob === requestedJob) selectedJobDetail = detail;
    } finally {
      if (selectedJob === requestedJob) { selectedJobLoading = false; renderJobs(); }
    }
  });
}
$('scan').onclick = guarded(async()=>{ await api('/api/v1/discovery/scan',{}); notice('Suchlauf gestartet. Die Übersicht aktualisiert sich automatisch.'); });
$('check').onclick = guarded(async()=>{ const j=await api('/api/v1/check',{}); selectedJob=j.id; notice('Git-Stand wird geprüft.'); await refresh(); });
$('filter').oninput = ()=>status && renderNodes();
configureUpdateControls();
$('job-list').onclick = event => { const button=event.target.closest('[data-job]'); if(button) {selectedJob=button.dataset.job;selectedJobLoading=false;renderJobs();} };
$('deploy-form').onsubmit = guarded(async()=>{
  const data=Object.fromEntries(new FormData($('deploy-form')));
  currentPlan=await api('/api/v1/plan',data);
  $('plan-text').textContent = `${currentPlan.action} · ${currentPlan.service} auf ${currentPlan.node_id}\nGit-Referenz: ${currentPlan.ref}\nNeustart: ${currentPlan.restarts.join(', ')}\nAbhängigkeiten: ${currentPlan.dependencies.map(d=>d.name+(d.resolved?' ✓':' – noch nicht aufgelöst')).join(', ') || 'Keine'}\nDer Commit wird vor dem Rollout aufgelöst und im Protokoll festgehalten.`;
  $('plan').hidden=false;
});
$('cancel-plan').onclick=()=>{currentPlan=null;$('plan').hidden=true;};
$('execute').onclick=guarded(async()=>{
  if (!currentPlan) return;
  $('execute').disabled=true;
  try {const j=await api('/api/v1/deploy',currentPlan);selectedJob=j.id;currentPlan=null;$('plan').hidden=true;notice('Deployment-Auftrag angelegt.');await refresh();} finally {$('execute').disabled=false;}
});
$('settings-form').onsubmit=guarded(async()=>{
  const bindings={};
  for(const line of $('bindings').value.split('\n').map(x=>x.trim()).filter(Boolean)) {const parts=line.split('=');if(parts.length!==2)throw Error('Zuordnung erwartet Rolle=Node-ID');bindings[parts[0].trim()]=parts[1].trim();}
  await api('/api/v1/settings',{ref:$('settings-ref').value,seeds:$('seeds').value.split('\n').map(x=>x.trim()).filter(Boolean),bindings});
  $('deploy-ref').value=$('settings-ref').value;notice('Einstellungen gespeichert.');await refresh();
});
let templateSubmitting=false,templateRevision=0;
function renderTemplateState(){
  $('template-status').textContent=status.has_template?'Standort-Template vorhanden. RF- und SDR-Einstellungen werden daraus übernommen.':'Noch kein Standort-Template. Erstelle es hier im Formular oder importiere eine vorhandene TBS-Konfiguration.';
  $('template-edit').disabled=!status.has_template||templateEditorBusy;
  $('template-new').disabled=status.has_template||templateSubmitting;
  $('template-new').title=status.has_template?'Ein Standort-Template ist bereits vorhanden. Du kannst es anzeigen und bearbeiten oder eine andere geprüfte Vorlage importieren.':'';
}
function templateValues(){
  const fields=$('template-form').elements;
  const number=name=>Number(fields.namedItem(name).value);
  return {main_carrier:number('main_carrier'),secondary_carrier:$('template-dual').checked?number('secondary_carrier'):null,
    sample_rate:number('sample_rate'),device:fields.namedItem('device').value,
    freq_band:number('freq_band'),freq_offset:number('freq_offset'),duplex_spacing:number('duplex_spacing'),
    custom_duplex_spacing:fields.namedItem('custom_duplex_spacing').value===''?null:number('custom_duplex_spacing'),
    reverse_operation:fields.namedItem('reverse_operation').checked};
}
async function previewTemplate(){
  if(!$('template-form').reportValidity())return;
  const revision=++templateRevision;
  $('template-error').textContent='';
  try{
    const result=await api('/api/v1/template/preview',templateValues());
    if(revision!==templateRevision)return;
    const mhz=value=>(value/1000000).toLocaleString('de-DE',{maximumFractionDigits:6})+' MHz';
    $('template-rf').textContent='TBS-TX (Downlink): '+mhz(result.rf.tx_freq)+' · TBS-RX (Uplink): '+mhz(result.rf.rx_freq)+' · SDR-Mitten TX/RX: '+mhz(result.rf.tx_center_freq)+' / '+mhz(result.rf.rx_center_freq);
    $('template-preview').textContent=result.toml;$('template-toml').hidden=false;
  }catch(error){if(revision===templateRevision){$('template-error').textContent=error.message;$('template-rf').textContent='';$('template-toml').hidden=true;}}
}
$('template-new').onclick=guarded(async()=>{
  $('template-form').reset();$('template-secondary-carrier').disabled=false;
  $('template-form').querySelector('details').open=false;$('template-toml').open=false;$('template-toml').hidden=true;
  $('template-error').textContent='';$('template-rf').textContent='Frequenzen werden berechnet …';
  $('template-dialog').showModal();await previewTemplate();
});
$('template-form').oninput=()=>{
  templateRevision++;$('template-secondary-carrier').disabled=!$('template-dual').checked;
  $('template-toml').hidden=true;$('template-rf').textContent='Einstellungen geändert. Vorschau neu anzeigen.';$('template-error').textContent='';
};
$('template-preview-button').onclick=previewTemplate;
$('template-cancel').onclick=()=>{$('template-dialog').close();templateRevision++;};
$('template-form').onsubmit=async event=>{
  event.preventDefault();if(templateSubmitting)return;templateSubmitting=true;templateRevision++;
  $('template-error').textContent='';
  for(const id of ['template-save','template-preview-button','template-cancel'])$(id).disabled=true;
  try{
    await api('/api/v1/template/create',templateValues());
    status.has_template=true;renderTemplateState();$('template-dialog').close();
    notice('Standort-Template erstellt. Du kannst jetzt ein TBS-Profil anlegen.');await refresh();
  }catch(error){$('template-error').textContent=error.message;if(error.status===409)await refresh();}
  finally{templateSubmitting=false;for(const id of ['template-save','template-preview-button','template-cancel'])$(id).disabled=false;renderTemplateState();}
};
let templateEditorBusy=false,templateEditorRevision='',templateEditorOriginal='';
function renderTemplateEditor(){
  const dirty=$('template-source').value!==templateEditorOriginal;
  $('template-source').disabled=templateEditorBusy;
  $('template-editor-save').disabled=templateEditorBusy||!dirty;
  $('template-editor-reload').disabled=templateEditorBusy;
  $('template-editor-cancel').disabled=templateEditorBusy;
  $('template-editor-cancel').textContent=dirty?'Abbrechen':'Schließen';
}
async function loadTemplateEditor(){
  const document=await api('/api/v1/template');
  $('template-source').value=document.toml;
  templateEditorOriginal=$('template-source').value;templateEditorRevision=document.revision;
  $('template-editor-error').textContent='';
}
$('template-edit').onclick=guarded(async()=>{
  if(templateEditorBusy)return;templateEditorBusy=true;renderTemplateState();
  try{await loadTemplateEditor();$('template-editor-dialog').showModal();}
  finally{templateEditorBusy=false;renderTemplateEditor();renderTemplateState();}
});
$('template-source').oninput=()=>{ $('template-editor-error').textContent='';renderTemplateEditor(); };
$('template-editor-cancel').onclick=()=>$('template-editor-dialog').close();
$('template-editor-dialog').addEventListener('cancel',event=>{if(templateEditorBusy)event.preventDefault();});
$('template-editor-dialog').addEventListener('close',()=>{
  $('template-source').value='';templateEditorOriginal='';templateEditorRevision='';$('template-editor-error').textContent='';
});
$('template-editor-reload').onclick=async()=>{
  if(templateEditorBusy)return;
  if($('template-source').value!==templateEditorOriginal&&!confirm('Ungespeicherte Änderungen verwerfen und die gespeicherte Template-Version neu laden?'))return;
  templateEditorBusy=true;renderTemplateEditor();
  try{await loadTemplateEditor();}catch(error){$('template-editor-error').textContent=error.message;}
  finally{templateEditorBusy=false;renderTemplateEditor();renderTemplateState();}
};
$('template-editor-form').onsubmit=async event=>{
  event.preventDefault();if(templateEditorBusy)return;templateEditorBusy=true;renderTemplateEditor();
  try{
    await api('/api/v1/template/update',{revision:templateEditorRevision,toml:$('template-source').value});
    $('template-editor-dialog').close();notice('Standort-Template aktualisiert. Neue Imageaufträge übernehmen die Änderungen.');await refresh();
  }catch(error){$('template-editor-error').textContent=error.message;}
  finally{templateEditorBusy=false;renderTemplateEditor();renderTemplateState();}
};
$('template').onchange=guarded(async()=>{const file=$('template').files[0];if(file){await api('/api/v1/template',{toml:await file.text()});notice('Standort-Template gespeichert.');await refresh();}});
$('profile-list').onclick=async event=>{
  const button=event.target.closest('[data-remove-profile]');if(!button||profileDeleting)return;
  event.preventDefault();
  const name=button.dataset.removeProfile;
  if(!confirm(`Das gespeicherte TBS-Profil „${name}“ löschen? Bereits installierte Basisstationen, Images und Buildprotokolle bleiben erhalten.`))return;
  profileDeleting=name;renderProfileList();
  try{
    await api('/api/v1/profiles/remove',{name});
    profileRevision++;
    renderProfiles(savedProfiles.filter(p=>p.name!==name));
    notice(`TBS-Profil „${name}“ gelöscht.`);
    await refresh();
  }catch(error){
    const applied=error.result?.profile_change_applied;
    const uncertain=applied===null||error.result?.durability_uncertain||!error.status||error.status>=500;
    let message=error.message;
    if(applied===true){
      profileRevision++;
      renderProfiles(savedProfiles.filter(p=>p.name!==name));
      message=`TBS-Profil „${name}“ wurde gelöscht, aber die dauerhafte Speicherung konnte nicht bestätigt werden. ${error.message}`;
    }
    if(applied===true||uncertain){
      if(applied!==true)profileRevision++;
      try{
        const revision=profileRevision;
        const profiles=await api('/api/v1/profiles');
        if(revision===profileRevision)renderProfiles(profiles);
        await refresh();
      }catch(refreshError){message+=` Der aktuelle Profilstand konnte nicht geladen werden: ${refreshError.message}`;}
    }
    notice(message,true);
  }
  finally{profileDeleting='';renderProfileList();}
};
$('profile-form').onsubmit=guarded(async()=>{const data=Object.fromEntries(new FormData($('profile-form')));for(const k of ['mcc','mnc','issi','la','cc'])data[k]=Number(data[k]);await api('/api/v1/profiles',data);notice('TBS-Profil angelegt. Du kannst jetzt ein Pi-Image erstellen oder den Bootstrap verwenden.');await refresh();$('image-profile').value=data.name;$('image-profile').dispatchEvent(new Event('change'));});
$('image-profile').onchange=()=>{$('image-hostname').value=$('image-profile').value.toLowerCase().replace(/[^a-z0-9-]/g,'-').slice(0,63).replace(/-+$/,'');renderImages();};
$('image-desktop').onchange=renderDesktopLogin;
$('image-job-list').onclick=event=>{const button=event.target.closest('[data-image-job]');if(button){selectedImageJob=button.dataset.imageJob;renderImages();}};
$('image-form').onsubmit=guarded(async()=>{
  if(imageSubmitting)return;
  imageSubmitting=true;renderImages();$('image-build').textContent='Build wird vorbereitet …';
  try {
    const data=Object.fromEntries(new FormData($('image-form')));
    data.trusted_ssids=$('image-trusted-ssids').value.split('\n').map(x=>x.trim()).filter(Boolean);
    const vpn=$('image-vpn').files[0], sip=$('image-sip').files[0];
    data.vpn_config=vpn ? await vpn.text() : '';
    data.sip_fallback_config=sip ? await sip.text() : '';
    const job=await api('/api/v1/images/build',data);selectedImageJob=job.id;
    $('image-form').elements.password.value='';$('image-form').elements.wifi_password.value='';$('image-vpn-username').value='';$('image-vpn-password').value='';$('image-vpn').value='';$('image-sip').value='';vpnNeedsCredentials=false;renderVpnCredentials();
    notice('Image-Build eingereiht. Fortschritt und Download erscheinen unter Builds & Downloads.');
    await refresh();
  } finally {imageSubmitting=false;$('image-build').textContent='Image erstellen →';renderImages();}
});
$('image-artifacts').onclick=async event=>{
  const button=event.target.closest('[data-remove-image]');if(!button)return;
  event.preventDefault();
  if(!confirm('Dieses herunterladbare Image von der VM löschen? Die Basisstation bleibt unverändert.'))return;
  try{await api('/api/v1/images/remove',{id:button.dataset.removeImage});notice('Image gelöscht. Das Buildprotokoll bleibt erhalten.');await refresh();}catch(error){notice(error.message,true);}
};
(async()=>{try{catalog=await api('/api/v1/catalog');selectOptions($('target-service'),catalog,s=>s.name,s=>s.name);selectOptions($('filter-service'),[{name:''},...catalog],s=>s.name || 'Alle Rollen',s=>s.name);await refresh();if(new URLSearchParams(location.search).get('scan')==='1'){await api('/api/v1/discovery/scan',{});notice('Auto Discovery gestartet.');}}catch(error){notice(error.message,true);}setInterval(refresh,4000);})();


