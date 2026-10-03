#!/usr/bin/env node
// Browser checks for the actual standalone Python pages and all ten Rust Brew routes.
// Fixtures remain isolated here; no network service or radio hardware is contacted.
import assert from 'node:assert/strict';
import http from 'node:http';
import { readFile, mkdir } from 'node:fs/promises';
import { execFileSync } from 'node:child_process';
import { createRequire } from 'node:module';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const require = createRequire(import.meta.url);
let playwright;
try { playwright = require('playwright'); }
catch (e) { if (!process.env.CODEX_PRIMARY_RUNTIME_NODE_MODULES) throw e; playwright = require(path.join(process.env.CODEX_PRIMARY_RUNTIME_NODE_MODULES, 'playwright')); }
const output = process.env.NETCORE_AUX_PREVIEW_DIR || path.join(root, 'target/service-ui-preview/auxiliary');
const brewDir = process.env.NETCORE_BREW_HTML_DIR || path.join(output, 'brew-html');
await mkdir(brewDir, { recursive: true });
if (!process.env.NETCORE_BREW_HTML_DIR) execFileSync(process.env.CARGO || 'cargo', ['test', '--manifest-path', 'misc/brew-server/Cargo.toml', '--locked', 'designed_pages_keep_routes_and_operational_controls'], { cwd: root, stdio: 'inherit', env: { ...process.env, NETCORE_UI_EXPORT_DIR: brewDir } });
const pages = JSON.parse(execFileSync(process.env.PYTHON || 'python3', ['-c', `import ast,json,pathlib
root=pathlib.Path('.')
pages={}
for service,fn,constants in [('assets','system-backend/asset-management/src/netcore_asset_management.py',['ASSET_MANAGEMENT_HTML']),('directory','system-backend/directory/netcore-directory.py',['INDEX_HTML']),('sip-switch','system-backend/sip-switch/src/netcore_sip_switch.py',['INDEX_HTML']),('tbs-connect','system-backend/tbs-connect/server.py',['DASHBOARD_HTML','LOGIN_HTML'])]:
 tree=ast.parse((root/fn).read_text())
 for node in tree.body:
  if isinstance(node,ast.Assign):
   for name in node.targets:
    if isinstance(name,ast.Name) and name.id in constants:pages[service+('-login' if name.id=='LOGIN_HTML' else '')]=node.value.value
print(json.dumps(pages))`], { cwd: root, maxBuffer: 10 * 1024 * 1024 }));
const brewRoutes = ['/', '/calls', '/sds', '/telemetry-sds', '/registrations', '/connections', '/map', '/sip', '/sip-config', '/settings'];
for (const route of brewRoutes) pages['brew' + route] = await readFile(path.join(brewDir, 'netcore-brew-' + (route === '/' ? 'index' : route.slice(1)) + '.html'), 'utf8');
const now = Date.now();
let current = 'assets', admin = true, requests = [];
const assets = [{ asset_id: 'test-radio', inventory_id: 'HRT-TEST', serial_number: 'TEST-001', kind: 'tetra_radio', manufacturer: 'Test', model: 'HRT', issi: 990001, status: 'in_stock', current_assignment_id: null, network_snapshot: { mobility: { serving_node: 'TBS-TEST' } } }];
const people = [{ person_id: 'test-person', display_name: 'Testperson', username: 'test', organization: 'Test', role: 'Technik', rui_username: 'test', rui_issi: 990001, active: true }];
const directory = { '/api/devices': [{ issi: 990001, name: 'Testfunkgerät', short: 'TEST', type: 'HRT', owner: 'Test', role: 'Technik', visible: true }], '/api/basestations': [{ issi: 990002, name: 'Teststation', short: 'TBS', location: 'Test', mcc: '901', mnc: '99', visible: true }], '/api/groups': [{ gssi: 990100, name: 'Testgruppe', short: 'TEST', type: 'talkgroup', owner: 'Test', visible: true }], '/api/device-groups': [{ group_id: 1, opta: 'TEST', name: 'Testgruppe', short: 'TEST', type: 'vehicle', members: [990001], status_sync: true, visible: true }], '/api/status': [{ code: 1, label: 'Teststatus', severity: 'info', description: 'Test', visible: true }] };
const tbsStatus = { uptime: 3600, connections: 1, subscribers: 1, active_calls: 1, active_group_calls: 1, total_group_calls: 5, total_registrations: 10, total_calls: 12, total_messages: 8 };
const tbsGroups = [{ source_issi: 990001, dest_gssi: 990100, frame_count: 20, start_time: new Date(now).toISOString(), uuid: 'test-group' }];
const tbsConnections = [{ uuid: 'test-connection', subscribers: [990001], connected_at: new Date(now).toISOString(), active_calls: 1 }];
const tbsLogs = [{ level: 'INFO', timestamp: new Date(now).toISOString(), issi: 990001, message: 'Testverbindung steht' }];
const brewStatus = { connected_basestations: 1, subscribers: 1, groups: 1, active_calls: [{ kind: 'group', source: 990001, destination: 990100, priority: 1, started_at_ms: now - 1000, voice_frames: 20, uuid: 'test-call' }], total_calls: 5, total_sds: 10, recent_calls: [{ kind: 'group', source: 990001, destination: 990100, started_at_ms: now - 5000, ended_at_ms: now, voice_frames: 90 }], recent_sds: [{ at_ms: now, source: 990001, destination: 990100, reports: 1, uuid: 'test-sds' }] };
const sip = { enabled: true, listen: '127.0.0.1:5060', realm: 'test', registrations: [], trunks: [], active_calls: [], total_calls: 0 };
const sipConfig = { enabled: true, listen: '127.0.0.1:5060', advertised_host: '127.0.0.1', realm: 'test', rtp_port_min: 16000, rtp_port_max: 17000, registration_ttl_seconds: 300, extensions: [], trunks: [], routes: [] };
const brewFixtures = { '/api/status': brewStatus, '/api/telemetry': [], '/api/rssi': {}, '/api/control': ['TBS-TEST'], '/api/registrations': [{ at_ms: now, bts: 'TBS-TEST', issi: 990001, kind: 'register' }], '/api/connections': { brew_clients: [], mobile_stations: [], sip: { enabled: true, registrations: [], trunks: [] } }, '/api/positions': [{ issi: 990001, lat: 52.1, lon: 9.1, bts: 'TBS-TEST', at_ms: now, source_text: 'Testposition' }], '/api/bts-locations': [{ username: '990002', name: 'Teststation', lat: 52.2, lon: 9.2, ip: '192.0.2.1', connected: true }], '/api/sip': sip, '/api/sip/config': sipConfig, '/api/config/sip/full': { extensions: {}, trunks: {}, routes: [] } };
function api(url) {
 const p = url.pathname;
 if (current === 'assets') return ({ '/api/v1/status': { assets_total: assets.length, persons_total: people.length, active_assignments: 0, maintenance_total: 1, maintenance_due: 0, mqtt_connected: true, external_last_sync_at: new Date(now).toISOString(), upstreams: { subscriber_core: { healthy: true }, mobility_core: { healthy: true } } }, '/api/v1/assets': assets, '/api/v1/persons': people, '/api/v1/assignments': [], '/api/v1/maintenance': [{ record_id: 'test-maint', asset_id: 'test-radio', title: 'Testwartung', kind: 'inspection', status: 'planned', due_at: new Date(now).toISOString() }], '/api/v1/events': [] })[p] ?? {};
 if (current === 'directory') return directory[p] ?? (p.startsWith('/api/dmr/user/') ? { results: [directory['/api/devices'][0]] } : {});
 if (current === 'sip-switch') return ({ '/api/v1/status': { health: { asterisk: true, mobility_core: true, pbx: true }, calls_active: 1, calls_total: 5, tbs_configured: 1, tbs_registered: 1, metrics: { routes_resolved: 5, routes_rejected: 0 }, media_mode: 'edge_media', pbx_mode: 'registration' }, '/api/v1/tbs': [{ node_id: 'TBS-TEST', endpoint_id: 'tbs-test', username: 'test', runtime: { registered: true, detail: 'Kontakt erreichbar' } }], '/api/v1/decisions': [], '/api/v1/calls': [], '/api/v1/events': [], '/api/v1/resolve': { action: 'tbs', reason: 'implicit_issi', node_id: 'TBS-TEST' } })[p] ?? {};
 if (current.startsWith('tbs')) return ({ '/api/status': tbsStatus, '/api/group_calls': tbsGroups, '/api/connections': tbsConnections, '/api/logs': tbsLogs })[p] ?? {};
 if (p === '/api/whoami') return { username: 'test-user', admin };
 return brewFixtures[p] ?? {};
}
const server = http.createServer(async (req, res) => {
 const url = new URL(req.url, 'http://localhost');
 let body = ''; for await (const chunk of req) body += chunk;
 requests.push({ method: req.method, path: url.pathname, body });
 res.setHeader('Cache-Control', 'no-store');
 if (url.pathname === '/api/events' && current.startsWith('tbs')) {
  res.writeHead(200, { 'Content-Type': 'text/event-stream', Connection: 'keep-alive' });
  for (const [event, data] of Object.entries({ stats: tbsStatus, connections: tbsConnections, group_calls: tbsGroups, logs: tbsLogs })) res.write(`event: ${event}\ndata: ${JSON.stringify(data)}\n\n`);
  return;
 }
 if (req.method === 'POST' && url.pathname === '/login') { res.writeHead(302, { Location: '/dashboard' }); res.end(); return; }
 if (url.pathname.startsWith('/api/')) {
  if (req.method !== 'GET') {
   if (current === 'assets' && url.pathname === '/api/v1/assets') assets.push({ ...JSON.parse(body), current_assignment_id: null });
   if (current === 'directory' && directory[url.pathname] && req.method === 'POST') directory[url.pathname].push(JSON.parse(body));
   res.setHeader('Content-Type', 'application/json'); res.end(JSON.stringify({ ok: true, output: 'Test: erfolgreich', rendered: [] })); return;
  }
  if (url.pathname === '/api/config/raw') { res.setHeader('Content-Type', 'text/plain'); res.end('listen = "127.0.0.1:9000"\n'); return; }
  res.setHeader('Content-Type', 'application/json'); res.end(JSON.stringify(api(url))); return;
 }
 const key = current === 'brew' ? 'brew' + url.pathname : (url.pathname === '/login' ? 'tbs-connect-login' : current === 'tbs-connect-login' && url.pathname === '/dashboard' ? 'tbs-connect' : current);
 let html = pages[key]; if (!html) { res.writeHead(404); res.end('Not found'); return; }
 html = html.replace(/(<script id="netcore-service-init">[\s\S]*?<\/script>)/, '$1<script>window.__netcoreEarlyTheme=document.documentElement.dataset.ncTheme;</script>');
 res.setHeader('Content-Type', 'text/html; charset=utf-8'); res.end(html);
});
const sockets = new Set();
server.on('connection', socket => { sockets.add(socket); socket.on('close', () => sockets.delete(socket)); });
await new Promise(resolve => server.listen(0, '127.0.0.1', resolve));
const base = `http://127.0.0.1:${server.address().port}`;
await mkdir(output, { recursive: true });
const browser = await playwright.chromium.launch({ headless: true, executablePath: process.env.CHROMIUM_EXECUTABLE_PATH });
let assertions = 0;
const check = (condition, message) => { assert.ok(condition, message); assertions++; };
try {
 const context = await browser.newContext({ viewport: { width: 1600, height: 1000 } });
 // Serve the repository's real Leaflet distribution and a blank map tile entirely locally.
 await context.route('https://unpkg.com/leaflet@1.9.4/dist/**', async route => {
  const filename = new URL(route.request().url()).pathname.split('/').pop();
  if (!['leaflet.js', 'leaflet.css'].includes(filename)) return route.fulfill({ contentType: 'image/png', body: Buffer.from('iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR4nGNgYGBgAAAABQABpfZFQAAAAABJRU5ErkJggg==', 'base64') });
  await route.fulfill({ contentType: filename.endsWith('.js') ? 'text/javascript' : 'text/css', body: await readFile(path.join(root, 'system-backend/alert-service/static/vendor', filename)) });
 });
 await context.route('https://*.tile.openstreetmap.org/**', route => route.fulfill({ contentType: 'image/png', body: Buffer.from('iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR4nGNgYGBgAAAABQABpfZFQAAAAABJRU5ErkJggg==', 'base64') }));
 const page = await context.newPage();
 let errors = [];
 page.on('pageerror', error => errors.push(error.message));
 page.on('dialog', dialog => dialog.accept(dialog.type() === 'prompt' ? 'Test' : undefined));
 const load = async (service, route = '/') => {
  current = service; errors = [];
  await page.goto(base + route);
  await page.locator('.nc-service-header').waitFor();
  check(await page.evaluate(() => document.querySelector('.nc-service-header').getBoundingClientRect().top === 0), service + route + ': header at top');
  await page.locator('.nc-service-logo img').first().waitFor();
  await page.waitForFunction(() => [...document.querySelectorAll('.nc-service-logo img')].every(img => img.complete && img.naturalWidth));
  check(await page.locator('.nc-service-header').count() === 1, service + route + ': one shell');
  check(await page.locator('.nc-service-logo img').count() >= 1, service + route + ': original logo loaded');
  const text = await page.locator('body').innerText();
  check(!text.includes('Designvorschau') && !text.includes('Beispieldaten'), service + route + ': production page');
 };
 const screenshot = async name => { await page.evaluate(() => window.scrollTo(0, 0)); await page.screenshot({ path: path.join(output, name + '.png'), fullPage: true }); };
 const noErrors = label => check(errors.length === 0, label + ': ' + errors.join('; '));
 const mobile = async label => {
  await page.setViewportSize({ width: 390, height: 844 });
  await page.evaluate(() => window.scrollTo(0, 0));
  const overflow = await page.evaluate(() => ({ width: document.documentElement.scrollWidth, viewport: innerWidth, elements: [...document.querySelectorAll('body *')].filter(e => e.getBoundingClientRect().right > innerWidth + 2 && !e.closest('.nc-table-wrap,.tablewrap,.table-wrap,.nc-service-nav')).map(e => ({tag:e.tagName,id:e.id,class:e.className,width:e.getBoundingClientRect().width,right:e.getBoundingClientRect().right})).slice(0,12) }));
  check(overflow.width <= overflow.viewport + 2, label + ': mobile document width ' + JSON.stringify(overflow));
  await screenshot(label + '-mobile');
  await page.setViewportSize({ width: 1600, height: 1000 });
 };
 const contrast = async label => {
  const results = await page.evaluate(() => {
   const rgba = value => {
    const match = value.match(/rgba?\(([^)]+)\)/); if (!match) return [0,0,0,0];
    const parts = match[1].split(/[,\s/]+/).filter(Boolean).map(Number); return [parts[0],parts[1],parts[2],parts[3] ?? 1];
   };
   const composite = (fg,bg) => { const a=fg[3]+bg[3]*(1-fg[3]);return a ? [0,1,2].map(i=>(fg[i]*fg[3]+bg[i]*bg[3]*(1-fg[3]))/a).concat(a) : [0,0,0,0]; };
   const luminance = rgb => rgb.slice(0,3).map(v=>{v/=255;return v<=.04045?v/12.92:((v+.055)/1.055)**2.4}).reduce((v,n,i)=>v+n*[.2126,.7152,.0722][i],0);
   const selectors = ['h1','h2','.nc-service-nav .active','.nc-service-nav a','.nc-service-nav button','label','input:not([type=checkbox])','select','textarea','th','tbody td','.muted','.stat-label','.stat-value','.nc-login-items strong','.nc-login-items span','.log .info','.group-call .source','.group-call .dest','.group-call .frames','.nc-sync-row span','pre','.leaflet-popup-content','.leaflet-popup-content span','.modal.open .field span','dialog[open] h2'];
   const found = [];
   for (const selector of selectors) {
    const element = [...document.querySelectorAll(selector)].find(e=>e.getBoundingClientRect().width>0 && e.getBoundingClientRect().height>0 && getComputedStyle(e).visibility!=='hidden');
    if (!element) continue;
    const style=getComputedStyle(element), chain=[];for(let e=element;e;e=e.parentElement)chain.push(e);
    let background=[255,255,255,1];for(const e of chain.reverse())background=composite(rgba(getComputedStyle(e).backgroundColor),background);
    const foreground=composite(rgba(style.color),background), l1=luminance(foreground),l2=luminance(background);
    const ratio=(Math.max(l1,l2)+.05)/(Math.min(l1,l2)+.05);
    const large=parseFloat(style.fontSize)>=24 || (parseFloat(style.fontSize)>=18.66 && Number(style.fontWeight)>=700);
    found.push({selector,ratio,minimum:large?3:4.5,color:style.color,background:background.slice(0,3)});
   }
   return found;
  });
  check(results.length >= 4, label + ': representative contrast samples');
  for (const item of results) check(item.ratio + .03 >= item.minimum, label + ': ' + JSON.stringify(item));
 };
 const darkMode = async label => {
  const toggle=page.locator('.nc-theme-toggle');
  await toggle.click();
  await page.waitForFunction(() => document.documentElement.dataset.ncTheme === 'dark');
  await page.evaluate(() => Promise.all(document.getAnimations().map(animation => animation.finished.catch(() => {}))));
  check(await toggle.getAttribute('aria-pressed') === 'true', label + ': dark control state');
  check(await page.evaluate(() => localStorage.getItem('netcore-theme')) === 'dark', label + ': canonical preference saved');
  check(await page.evaluate(() => getComputedStyle(document.body).backgroundColor !== 'rgb(243, 246, 251)'), label + ': dark surface');
  await contrast(label + '-dark');
  if (current === 'assets') {
   await page.getByRole('button',{name:'Asset anlegen',exact:true}).click();await contrast(label+'-dark-editor');await screenshot(label+'-dark-editor');await page.locator('#assetDlg').getByRole('button',{name:'Abbrechen'}).click();
  }
  if (current === 'directory') {
   await page.locator('#newBtn').click();await contrast(label+'-dark-editor');await screenshot(label+'-dark-editor');await page.locator('.box-actions').getByRole('button',{name:'Abbrechen'}).click();
  }
  if (current === 'brew' && label === 'brew-map') {
   await page.locator('.leaflet-marker-icon').first().click();await page.locator('.leaflet-popup-content').waitFor();await page.evaluate(() => Promise.all(document.getAnimations().map(animation => animation.finished.catch(() => {}))));await contrast(label+'-dark-popup');
  }
  await screenshot(label+'-dark');
  await mobile(label+'-dark');
  await page.reload();await page.locator('.nc-service-header').waitFor();
  check(await page.evaluate(() => document.documentElement.dataset.ncTheme) === 'dark', label + ': reload retains dark');
  check(await page.evaluate(() => window.__netcoreEarlyTheme) === 'dark', label + ': preference applied before service markup/styles');
  check(await page.locator('.nc-theme-toggle').getAttribute('aria-pressed') === 'true', label + ': reloaded toggle');
  await page.locator('.nc-theme-toggle').click();await page.waitForFunction(() => document.documentElement.dataset.ncTheme === 'light');
  await page.evaluate(() => Promise.all(document.getAnimations().map(animation => animation.finished.catch(() => {}))));
  check(await page.evaluate(() => localStorage.getItem('netcore-theme')) === 'light', label + ': light preference saved');
  check(await page.evaluate(() => getComputedStyle(document.body).backgroundColor) === 'rgb(243, 246, 251)', label + ': light palette restored');
 };
 await load('assets');
 await page.locator('#assetRows').getByText('HRT-TEST').waitFor();
 check((await page.locator('#assetRows').innerText()).includes('Verfügbar'), 'Assets: supported status displayed');
 check(await page.locator('.nc-service-nav a').count() === 7, 'Assets: seven domain sections');
 await page.getByRole('button', { name: 'Asset anlegen', exact: true }).click();
 await page.locator('#assetForm [name=asset_id]').fill('created-test');
 await page.locator('#assetForm [name=inventory_id]').fill('HRT-CREATED');
 await page.locator('#assetForm button.primary').click();
 await page.locator('#assetRows').getByText('HRT-CREATED').waitFor();
 check(requests.some(x => x.method === 'POST' && x.path === '/api/v1/assets' && JSON.parse(x.body).asset_id === 'created-test'), 'Assets: create reaches original endpoint');
 await page.getByRole('button', { name: 'Person anlegen', exact: true }).click();
 check(await page.locator('#personDlg').isVisible(), 'Assets: person editor retained');
 await page.locator('#personDlg').getByRole('button', { name: 'Abbrechen' }).click();
 await page.getByRole('button', { name: 'Wartung planen', exact: true }).click();
 check(await page.locator('#maintDlg').isVisible(), 'Assets: maintenance editor retained');
 await page.locator('#maintDlg').getByRole('button', { name: 'Abbrechen' }).click();
 await screenshot('asset-management'); await mobile('asset-management'); await darkMode('asset-management'); noErrors('Assets');
 await load('directory'); await page.locator('#tbody').getByText('Testfunkgerät').waitFor();
 for (const [tab, expected] of [['basestations', 'Teststation'], ['groups', 'Testgruppe'], ['device_groups', 'Testgruppe'], ['status_messages', 'Teststatus']]) {
  await page.locator(`[data-tab="${tab}"]`).click();
  check((await page.locator('#tbody').innerText()).includes(expected), 'Directory: ' + tab + ' tab');
 }
 await page.locator('[data-tab=devices]').click();
 await page.locator('#newBtn').click(); await page.locator('#f-issi').fill('990003'); await page.locator('#f-name').fill('Neues Testfunkgerät');
 await page.locator('.box-actions').getByRole('button', { name: 'Speichern' }).click();
 await page.locator('#tbody').getByText('Neues Testfunkgerät').waitFor();
 check(requests.some(x => x.method === 'POST' && x.path === '/api/devices'), 'Directory: create retained');
 await screenshot('directory'); await mobile('directory'); await darkMode('directory'); noErrors('Directory');
 await load('sip-switch'); await page.locator('#tbsRows tr td').first().waitFor();
 await page.locator('#number').fill('990001'); await page.getByRole('button', { name: 'Auflösen', exact: true }).click();
 await page.waitForFunction(() => document.getElementById('routeResult').textContent.includes('implicit_issi'));
 check((await page.locator('#cards').innerText()).includes('edge_media'), 'SIP: real media mode');
 const renderResponse = page.waitForResponse(response => response.url().endsWith('/api/v1/actions/render-asterisk') && response.request().method() === 'POST');
 await page.getByRole('button', { name: 'Asterisk neu rendern' }).click();
 await renderResponse;
 check(requests.some(x => x.method === 'POST' && x.path === '/api/v1/actions/render-asterisk'), 'SIP: original render action');
 await screenshot('sip-switch'); await mobile('sip-switch'); await darkMode('sip-switch'); noErrors('SIP Switch');
 await load('tbs-connect-login', '/login');
 check(await page.locator('form input').count() === 2, 'TBS Login: exactly two genuine fields');
 check(!(await page.locator('body').innerText()).includes('OPEN LAB'), 'TBS Login: real session mode');
 await screenshot('tbs-connect-login'); await mobile('tbs-connect-login'); await darkMode('tbs-connect-login');
 await page.locator('[name=username]').fill('test-user'); await page.locator('[name=password]').fill('test-password');
 await page.getByRole('button', { name: 'Anmelden', exact: true }).click();
 await page.waitForURL(base + '/dashboard');
 check(requests.some(x => x.method === 'POST' && x.path === '/login' && x.body.includes('username=test-user')), 'TBS Login: original form POST');
 await load('tbs-connect', '/dashboard');
 await page.locator('#connections').getByText('test-connection').waitFor();
 await page.locator('#sse-status').getByText('Live verbunden').waitFor();
 check((await page.locator('#group-calls').innerText()).includes('GSSI 990100'), 'TBS: live group data');
 check(!(await page.locator('body').innerText()).includes('OPEN LAB'), 'TBS Dashboard: session access');
 await screenshot('tbs-connect'); await mobile('tbs-connect'); await darkMode('tbs-connect'); noErrors('TBS Connect');
 for (const route of brewRoutes) {
  await load('brew', route);
  check(await page.locator('.nc-service-nav a').count() === 10, 'Rust Brew' + route + ': ten routes');
  await page.waitForFunction(() => document.getElementById('whoami')?.textContent === 'test-user');
  check(await page.locator('.nc-service-nav a[aria-current=page]').getAttribute('href') === route, 'Rust Brew' + route + ': selected route');
  if (route === '/') {
   await page.locator('#livecalls').getByText('990001').waitFor();
   await page.locator('#control-stations [aria-label="DGNA Attachment-Modus"]').fill('1');
   await page.locator('[id$="_dgna_issi"]').fill('990001'); await page.locator('[id$="_dgna_gssi"]').fill('990100');
   await page.getByRole('button', { name: 'DGNA', exact: true }).click();
   await page.waitForFunction(() => document.querySelector('[id$="_result"]').textContent.startsWith('OK:'));
   check(requests.some(x => x.method === 'POST' && x.path === '/api/control/TBS-TEST' && JSON.parse(x.body).attachment_mode === 1 && JSON.parse(x.body).attach === true), 'Rust Brew: actual DGNA payload preserved');
  }
  if (route === '/settings') {
   await page.waitForFunction(() => document.getElementById('raw').value.includes('listen'));
   await page.locator('#ext-user').fill('1001'); await page.locator('#ext-name').fill('Testextension');
   const extensionResponse = page.waitForResponse(response => response.url().endsWith('/api/config/sip/extensions/1001') && response.request().method() === 'POST');
   await page.locator('button[onclick="saveExt()"]').click();
   await extensionResponse;
   check(requests.some(x => x.method === 'POST' && x.path === '/api/config/sip/extensions/1001'), 'Rust Brew: extension editor POST preserved');
  }
  const label = 'brew-' + (route === '/' ? 'index' : route.slice(1));
  await screenshot(label); await mobile(label); await darkMode(label); noErrors(label);
 }
 admin = false; await load('brew', '/');
 await page.waitForFunction(() => document.querySelector('[data-nc-admin-link]').hidden);
 check(!(await page.locator('[data-nc-admin-link]').isVisible()), 'Rust Brew: settings navigation follows whoami admin');
 await page.evaluate(() => {localStorage.removeItem('netcore-theme');localStorage.setItem('netcore-service-theme','dark')});
 await page.reload();await page.locator('.nc-service-header').waitFor();
 check(await page.evaluate(() => window.__netcoreEarlyTheme) === 'dark', 'Rust Brew: legacy theme preference preserved');
 await page.evaluate(() => {localStorage.setItem('netcore-theme','blue');localStorage.setItem('netcore-service-theme','dark')});
 await page.reload();await page.locator('.nc-service-header').waitFor();
 check(await page.evaluate(() => window.__netcoreEarlyTheme) === 'light', 'Rust Brew: explicit base blue preference wins over legacy dark');
 console.log(`${assertions} auxiliary WebUI checks passed across5services/15views. Screenshots: ${output}`);
} finally {
 await browser.close();
 sockets.forEach(socket => socket.destroy());
 await new Promise(resolve => server.close(resolve));
}
