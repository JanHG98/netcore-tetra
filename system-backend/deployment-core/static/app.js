'use strict';
const $ = id => document.getElementById(id);
let status = null, catalog = [], jobs = [], selectedJob = '', currentPlan = null, initialized = false, busy = false;
const labels = {queued:'Wartet',running:'Läuft',succeeded:'Erfolgreich',failed:'Fehlgeschlagen',interrupted:'Unterbrochen'};
const esc = value => String(value ?? '').replace(/[&<>"']/g, c => ({'&':'&amp;','<':'&lt;','>':'&gt;','"':'&quot;',"'":'&#39;'}[c]));
const short = value => value ? value.slice(0, 8) : 'Unbekannt';
function notice(message, error = false) { $('notice').textContent = message; $('notice').className = error ? 'error' : ''; $('notice').hidden = false; }
async function api(path, data) {
  const options = data === undefined ? {} : {method:'POST', headers:{'Content-Type':'application/json'}, body:JSON.stringify(data)};
  const response = await fetch(path, options);
  const result = await response.json();
  if (!response.ok) throw new Error(result.error || `HTTP ${response.status}`);
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
function renderNodes() {
  const nodes = [...status.peers];
  if (status.role === 'agent') nodes.unshift({node_id:status.node_id, agent_url:location.origin, online:true, services:status.services});
  const services = nodes.flatMap(n => n.services.map(s => ({...s, node:n})));
  $('host-count').textContent = nodes.filter(n => n.online && n.role !== 'controller').length;
  $('service-count').textContent = services.filter(s => s.ready && s.node.online).length;
  $('drift-count').textContent = status.desired.commit ? services.filter(s => s.commit && s.commit !== status.desired.commit).length : '—';
  $('commit').textContent = short(status.desired.commit);
  $('target-ref').textContent = status.desired.ref || status.settings.ref;
  $('instance').textContent = `${status.node_id} · ${status.environment} · ${status.role === 'controller' ? 'Deployment Core' : 'Discovery-Agent'}`;
  const q = $('filter').value.toLowerCase();
  const rows = nodes.flatMap(n => (n.services.length ? n.services : [{name:'Agent bereit',ready:n.online,commit:'',url:''}]).map(s => ({...s,node:n}))).filter(s => `${s.name} ${s.node.node_id}`.toLowerCase().includes(q));
  $('nodes').innerHTML = rows.map(s => {
    const online = s.node.online && s.ready, drift = s.commit && status.desired.commit && s.commit !== status.desired.commit;
    const url = s.url || (s.port ? `${new URL(s.node.agent_url).protocol}//${new URL(s.node.agent_url).hostname}:${s.port}` : '');
    return `<tr><td><strong>${esc(s.name)}</strong><small>${esc(s.node.node_id)}</small></td><td><span class="badge ${online?'ok':'warn'}">${online?'Bereit':s.node.online?'Nicht bereit':'Offline'}</span></td><td><span class="${drift?'badge warn':''}">${esc(short(s.commit))}</span>${drift?' <small>Abweichend</small>':''}</td><td>${url?`<a href="${esc(url)}" target="_blank" rel="noopener">WebUI ↗</a>`:''}<a href="${esc(s.node.agent_url)}/?scan=1" target="_blank" rel="noopener">Discovery ↗</a></td></tr>`;
  }).join('');
  $('empty').hidden = nodes.length > 0;
  const conflicts = Object.entries(status.conflicts).map(([k,v]) => `${k}: ${v.join(', ')}`);
  const notes = [...conflicts.map(c => 'Mehrdeutige Zuordnung – ' + c), ...(status.multicast_error ? ['Multicast: '+status.multicast_error+'. Feste Gegenstellen bleiben aktiv.'] : [])];
  $('discovery-note').hidden = notes.length === 0;
  $('discovery-note').textContent = notes.join(' · ');
  selectOptions($('target-node'), status.peers.filter(n => n.online && n.role === 'agent'), n => n.node_id, n => n.node_id);
}
function renderJobs() {
  $('job-list').innerHTML = jobs.length ? jobs.map(j => `<button class="job ${selectedJob===j.id?'selected':''}" data-job="${esc(j.id)}"><b>${esc(j.request.service || 'Git-Stand prüfen')}</b><small>${esc(j.request.node_id || j.request.ref || j.request.action || '')} · ${new Date(j.created*1000).toLocaleTimeString('de-DE')}</small><span class="badge ${j.status==='succeeded'?'ok':j.status==='failed'?'bad':'warn'}">${labels[j.status] || esc(j.status)}</span></button>`).join('') : '<p class="empty">Noch keine Aufträge.</p>';
  const job = jobs.find(j => j.id === selectedJob);
  if (job) { $('log-title').textContent = `${labels[job.status]} · ${job.id.slice(0,8)}`; $('log').textContent = job.log || 'Auftrag wartet auf Ausführung.'; }
}
async function refresh() {
  if (busy) return;
  busy = true;
  try {
    const [s, j, profiles] = await Promise.all([api('/api/v1/status'),api('/api/v1/jobs'),api('/api/v1/profiles')]);
    status = s; jobs = j;
    document.body.classList.toggle('agent', s.role === 'agent');
    $('connection').textContent = '● Verbunden'; $('connection').className = 'badge ok';
    if (!initialized) {
      $('settings-ref').value = s.settings.ref; $('deploy-ref').value = s.settings.ref;
      $('seeds').value = s.settings.seeds.join('\n'); $('bindings').value = Object.entries(s.settings.bindings).map(([k,v])=>`${k}=${v}`).join('\n'); initialized = true;
    }
    $('template-status').textContent = s.has_template ? 'Standort-Template vorhanden. RF- und SDR-Einstellungen werden daraus übernommen.' : 'Noch kein Standort-Template. Importiere eine geprüfte TBS-Konfiguration; sie wird nicht per Discovery verteilt.';
    selectOptions($('target-profile'), [{name:''},...profiles], p=>p.name || 'Keins', p=>p.name);
    $('profile-list').innerHTML = profiles.map(p=>`<a href="/bootstrap.sh?profile=${encodeURIComponent(p.name)}" download="${esc(p.name)}-bootstrap.sh">${esc(p.name)} · Bootstrap ↓</a>`).join('');
    renderNodes(); renderJobs();
  } catch (error) { $('connection').textContent = 'Verbindung verloren'; $('connection').className = 'badge bad'; notice(error.message,true); }
  finally { busy = false; }
}
$('scan').onclick = guarded(async()=>{ await api('/api/v1/discovery/scan',{}); notice('Suchlauf gestartet. Die Übersicht aktualisiert sich automatisch.'); });
$('check').onclick = guarded(async()=>{ const j=await api('/api/v1/check',{}); selectedJob=j.id; notice('Git-Stand wird geprüft.'); await refresh(); });
$('filter').oninput = ()=>status && renderNodes();
$('job-list').onclick = event => { const button=event.target.closest('[data-job]'); if(button) {selectedJob=button.dataset.job;renderJobs();} };
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
$('template').onchange=guarded(async()=>{const file=$('template').files[0];if(file){await api('/api/v1/template',{toml:await file.text()});notice('Standort-Template gespeichert.');await refresh();}});
$('profile-form').onsubmit=guarded(async()=>{const data=Object.fromEntries(new FormData($('profile-form')));for(const k of ['mcc','mnc','issi','la','cc'])data[k]=Number(data[k]);await api('/api/v1/profiles',data);notice('TBS-Profil angelegt. Bootstrap herunterladen und auf dem Pi ausführen.');await refresh();});
(async()=>{try{catalog=await api('/api/v1/catalog');selectOptions($('target-service'),catalog,s=>s.name,s=>s.name);await refresh();if(new URLSearchParams(location.search).get('scan')==='1'){await api('/api/v1/discovery/scan',{});notice('Auto Discovery gestartet.');}}catch(error){notice(error.message,true);}setInterval(refresh,4000);})();
