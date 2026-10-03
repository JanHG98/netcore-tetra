#!/usr/bin/env node
// NETCORE-KOMMENTAR – Was: Prüft die echten Dashboard-Templates mit einem isolierten HTTP-/WebSocket-Testserver.
// NETCORE-KOMMENTAR – Warum: Navigation, Telemetrie und anonyme Zugriffssperren brauchen Browserprüfungen; alle Beispieldaten bleiben ausschließlich hier.
// Run: node tools/test_dashboard_ui.mjs
// Requires Playwright and its Chromium browser; CHROMIUM_EXECUTABLE_PATH may select an existing browser.
import assert from 'node:assert/strict';
import http from 'node:http';
import { readFile, mkdir } from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { createRequire } from 'node:module';

const require = createRequire(import.meta.url);
let playwright;
try { playwright = require('playwright'); }
catch (error) {
  if (!process.env.CODEX_PRIMARY_RUNTIME_NODE_MODULES) throw error;
  playwright = require(path.join(process.env.CODEX_PRIMARY_RUNTIME_NODE_MODULES, 'playwright'));
}
const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const ui = path.join(root, 'crates/tetra-entities/src/net_dashboard/ui');
const output = path.join(root, 'target/dashboard-preview');
const dashboardPages = ['home', 'stations', 'calls', 'lastheard', 'rf', 'health', 'log', 'sdslog', 'packetdata',
  'maps', 'neighbors', 'asterisk', 'audio', 'recordings', 'config', 'telegram', 'wifi', 'system', 'services',
  'help', 'dapnet', 'echolink', 'meshcom', 'geoalarm'];
const labPages = ['dapnet', 'echolink', 'meshcom', 'geoalarm'];
const now = '2026-10-02T12:00:00Z';
// These wire shapes follow net_dashboard/state.rs and the respective handlers in server.rs.
const txVisual = { type: 'tx_visual', sample_rate: 600000, center_freq_hz: 390125000, rms_dbfs: -19.3,
  peak_dbfs: -12.2, spectrum_db_tenths: Array.from({ length: 512 }, (_, i) => Math.round((-85 + 66 * Math.exp(-(((i - 256) / 17) ** 2))) * 10)),
  constellation_iq: Array.from({ length: 96 }, (_, i) => [Math.round(Math.cos(i * Math.PI / 4) * 13000), Math.round(Math.sin(i * Math.PI / 4) * 13000)]).flat() };
const txQuality = { type: 'tx_quality', evm_pct: 1.8, papr_db: 3.7, carrier_leakage_db: -52,
  occupied_bandwidth_hz: 22300, dc_offset_i: 0, dc_offset_q: 0, iq_amplitude_imbalance_db: 0.1, iq_phase_imbalance_deg: 0.2 };
const sdrHealth = { type: 'sdr_health', temperature_c: 44.6, tx_gains: [['PAD', 10]], rx_gains: [['LNA', 18]] };
const snapshot = { type: 'snapshot', brew_online: true, brew_version: 2, emergencies: [],
  ms: [{ issi: 990001, groups: [990100], selected_group: 990100, rssi_dbfs: -34.2, last_seen_secs_ago: 3, energy_saving_mode: 0 },
    { issi: 990002, groups: [990100], selected_group: null, rssi_dbfs: null, last_seen_secs_ago: 15, energy_saving_mode: 1 }],
  calls: [{ call_id: 71, call_type: 'group', gssi: 990100, caller_issi: 990001, active_speaker: 990001,
    called_issi: 0, started_secs_ago: 12, simplex: true, ts: 5, carrier_num: 102, priority: 0 }],
  last_heard: [{ ts: '12:00:00', issi: 990001, activity: 'call_group', dest: 990100, source: 'local', call_id: 71, duration_secs: 12 }],
  log: [{ ts: '12:00:00', level: 'INFO', msg: 'UI test connected' }, { ts: '12:00:01', level: 'WARN', msg: 'UI test warning' }],
  last_tx_visual: txVisual, last_tx_quality: txQuality, last_sdr_health: sdrHealth,
  health: { overall: 'ok', uptime_secs: 3600, last_action: null, domains: [
    { domain: 'service', level: 'ok', detail: 'main loop alive' }, { domain: 'backhaul', level: 'ok', detail: 'connected' },
    { domain: 'radios', level: 'ok', detail: '2 registered' }, { domain: 'congestion', level: 'ok', detail: 'channels available' }] } };
const fixtureApi = {
  '/api/devices': { '990001': { name: 'UI Test Radio', visible: true }, '990002': { name: 'UI Test Backup', visible: true } },
  '/api/groups': { '990100': { name: 'UI Test Group' } },
  '/api/system': { hostname: 'ui-test-station', stack_version: 'test', os: 'Test Linux', config_path: '/test/config.toml',
    cpu_model: 'Test CPU', cpu_cores: 4, cpu_pct: 23, cpu_temp_c: 45, ram_total_mb: 4096, ram_used_mb: 1024,
    uptime_secs: 3600, sdr_name: 'Test SDR', soapy_info: 'Fixture hardware' },
  '/api/btsinfo': { tx_freq_hz: 390125000, rx_freq_hz: 380125000, shift_hz: -10000000, mcc: 901, mnc: 99,
    main_carrier: 101, hangtime_secs: 3, whitelist_restricted: false, whitelist_count: 0, neighbor_count: 1,
    neighbors: [{ cell_identifier_ca: 2, main_carrier_number: 103, main_carrier_number_extension: 0,
      cell_reselection_types_supported: 0, neighbor_cell_synchronized: false, cell_load_ca: 0,
      mcc: 901, mnc: 99, location_area: 2, tdma_frame_offset: null }] },
  '/api/dualcarrier': { enabled: true, secondary_carrier: 102, active: true, running_active: true, main_carrier: 101 },
  '/api/edge-fallback': { enabled: true, mode: 'online', gateway_connected: true, reason: 'central service plane healthy',
    service_matrix_fresh: true, service_matrix_received_at: now, services: [{ service: 'node-gateway', name: 'Node Gateway',
      level: 'available', message: 'Node Gateway WebSocket connected', critical_for_edge: true, fallback_mode: 'local_edge_autonomy' }] },
  '/api/brew/status': { servers: [{ entity: 'brew', configured: true, connected: true, endpoint: 'fixture.invalid:31000',
    feature_sds_enabled: true, feature_rssi_export: false, local_issi_allowlist: [], local_issi_blocklist: [] }] },
  '/api/sds-log': [{ ts: '2026-10-02 12:00:00', direction: 'rx', source_issi: 990001, dest_issi: 990100,
    is_group: true, protocol_id: 130, text: 'UI fixture message' }],
  '/api/packet-data': { packet_data: { gateway: {}, contexts: [], bearers: [] } },
  '/api/asterisk/status': { config: { enabled: false }, runtime: { enabled: false, active_dialogs: 0 } },
  '/api/snom-notify': {}, '/api/dapnet': { enabled: false, runtime: {} }, '/api/dapnet-log': [],
  '/api/echolink': { enabled: false, runtime: {} }, '/api/echolink/directory': { entries: [] },
  '/api/meshcom': { enabled: false, runtime: {} }, '/api/meshcom-nodes': [], '/api/meshcom-messages': [],
  '/api/geoalarm': { enabled: false, runtime: {}, events: [] },
  '/api/audio/status': { state: 'idle', ffmpeg_available: true, position_ms: 0, duration_ms: 0 },
  '/api/audio/sources': { sources: [{ id: 'local', name: 'Fixture media', path: '/test/media', source_type: 'local', available: true }] },
  '/api/audio/browse': { entries: [] },
  '/api/recordings/status': { available: true, active: true, mode: 'all_local_calls', recording_count: 1,
    free_space_bytes: 1073741824, used_bytes: 0, directory: '/test/recordings', active_sessions: 0, archive_enabled: false },
  '/api/recordings': [{ schema_version: 1, id: 'ui-test-recording', title: 'UI Fixture Recording', origin: 'call',
    call_id: 71, source_issi: 990001, destination_id: 990100, destination_type: 'group', started_at: now,
    ended_at: '2026-10-02T12:00:01Z', duration_ms: 1000, audio_bytes: 16000, relative_audio_path: 'ui-test-recording.wav',
    recovered_after_unclean_shutdown: false, segments: [{ source_issi: 990001, timeslot: 5, carrier_num: 102, start_ms: 0, end_ms: 1000 }] }],
  '/api/whitelist': { whitelist: [] }, '/api/wx': {}, '/api/telegram': { enabled: false, chat_ids: [] },
  '/api/configs': [], '/api/live-sds': [], '/api/system/brightness': { available: false },
  '/api/wifi/status': { ok: true, status: { device_present: true, radio_enabled: true, connected_ssid: 'UI Test Network',
    signal: 78, ip_address: '192.0.2.2' } },
  '/api/wifi/saved': { ok: true, profiles: [] }, '/api/wifi/scan': { ok: true, networks: [] },
};
fixtureApi['/api/edge-fallback'].services = ['node-gateway', 'subscriber-core', 'group-core', 'mobility-core',
  'call-control', 'media-switch', 'sds-router', 'packet-core', 'ip-gateway', 'security-core', 'kmf', 'transit',
  'application-gateway', 'media-library', 'recorder', 'observability', 'control-room'].map(service => ({
    service, level: 'available', message: 'Fixture service reachable', checked_at: now,
    critical_for_edge: ['node-gateway', 'subscriber-core', 'group-core', 'mobility-core', 'call-control', 'media-switch', 'sds-router'].includes(service),
  }));
const publicFixture = { available: true, registered_ms: 2, active_calls: 1, group_calls: 1, individual_calls: 0,
  rf_active: true, brew_online: true, stack_version: 'test', uptime_secs: 3600, cpu_temp_c: 45,
  cpu_load_pct: null, health_status: 'ok', captured_at_unix_ms: 1790942400000 };
// A silent mono PCM WAV allows the actual <audio> decoder to load the recording without an external file.
const testWav = Buffer.alloc(16044);
testWav.write('RIFF', 0); testWav.writeUInt32LE(testWav.length - 8, 4); testWav.write('WAVEfmt ', 8);
testWav.writeUInt32LE(16, 16); testWav.writeUInt16LE(1, 20); testWav.writeUInt16LE(1, 22);
testWav.writeUInt32LE(8000, 24); testWav.writeUInt32LE(16000, 28); testWav.writeUInt16LE(2, 32);
testWav.writeUInt16LE(16, 34); testWav.write('data', 36); testWav.writeUInt32LE(16000, 40);
let scenario = { authenticated: true, auth_required: true, public_overview: true, wifi: true };
let requests = [];

// Was: Bedient nur die realen Template-/Assetdateien und kontrollierte lokale Testantworten.
// Warum: Die Suite greift weder auf einen echten Funkknoten noch auf externe Dienste zu.
const server = http.createServer(async (req, res) => {
  const url = new URL(req.url, 'http://localhost');
  const record = { method: req.method, path: url.pathname };
  requests.push(record);
  res.setHeader('Cache-Control', 'no-store');
  if (url.pathname.startsWith('/api/')) {
    res.setHeader('Content-Type', 'application/json');
    if (req.method === 'POST' && url.pathname === '/api/login') {
      let body = ''; for await (const chunk of req) body += chunk;
      record.body = JSON.parse(body);
      res.writeHead(401); res.end(JSON.stringify({ error: 'test invalid credentials' })); return;
    }
    if (req.method === 'POST' && ['/api/audio/play', '/api/audio/stop'].includes(url.pathname)) {
      if (!scenario.authenticated && scenario.auth_required) { res.writeHead(401); res.end('{}'); return; }
      let body = ''; for await (const chunk of req) body += chunk;
      record.body = body ? JSON.parse(body) : null;
      scenario.audio_request = url.pathname === '/api/audio/play' ? record.body : null;
      res.end(JSON.stringify({ ok: true })); return;
    }
    if (req.method !== 'GET') { res.writeHead(405); res.end('{}'); return; }
    if (url.pathname === '/api/session') {
      if (scenario.session_status) res.writeHead(scenario.session_status);
      res.end(JSON.stringify({ authenticated: scenario.authenticated, auth_required: scenario.auth_required,
        public_overview: scenario.public_overview })); return;
    }
    if (url.pathname === '/api/public') {
      res.writeHead(scenario.public_overview ? 200 : 403);
      res.end(JSON.stringify(scenario.public_overview ? publicFixture : { error: 'public overview disabled' })); return;
    }
    if (!scenario.authenticated && scenario.auth_required) { res.writeHead(401); res.end('{}'); return; }
    if (url.pathname === '/api/recordings/ui-test-recording/audio') {
      res.setHeader('Content-Type', 'audio/wav'); res.setHeader('Content-Length', testWav.length);
      res.end(testWav); return;
    }
    if (url.pathname === '/api/audio/status' && scenario.audio_request) {
      res.end(JSON.stringify({ ...fixtureApi[url.pathname], state: 'playing', file_name: 'UI Fixture Recording',
        target_type: scenario.audio_request.target_type, target_id: scenario.audio_request.target_id,
        priority: scenario.audio_request.priority, call_id: 72, timeslot: 2, duration_ms: 6000,
        position_ms: 600, sent_blocks: 10, total_blocks: 100 })); return;
    }
    if (url.pathname === '/api/wifi/available') { res.end(JSON.stringify({ available: scenario.wifi })); return; }
    if (url.pathname === '/api/config') { res.setHeader('Content-Type', 'text/plain'); res.end('# UI test config\n'); return; }
    if (url.pathname === '/api/edge-fallback' && scenario.edge_isolated) {
      res.end(JSON.stringify({ ...fixtureApi[url.pathname], mode: 'isolated', gateway_connected: false,
        service_matrix_fresh: false, reason: 'Node Gateway unreachable; local edge authority active',
        services: [{ service: 'node-gateway', level: 'unavailable', critical_for_edge: true,
          fallback_mode: 'local_edge_autonomy', message: 'Node Gateway unreachable', checked_at: now }] })); return;
    }
    if (url.pathname in fixtureApi) { res.end(JSON.stringify(fixtureApi[url.pathname])); return; }
    res.writeHead(404); res.end(JSON.stringify({ error: 'Unmocked test endpoint: ' + url.pathname })); return;
  }
  try {
    let file;
    if (url.pathname === '/' || url.pathname === '/index.html') file = 'dashboard.html';
    else if (url.pathname === '/login') file = 'login.html';
    else file = path.basename(url.pathname);
    const types = { '.html': 'text/html', '.css': 'text/css', '.js': 'application/javascript', '.png': 'image/png', '.svg': 'image/svg+xml' };
    res.setHeader('Content-Type', types[path.extname(file)] || 'application/octet-stream');
    res.end(await readFile(path.join(ui, file)));
  } catch { res.writeHead(404); res.end('Not found'); }
});
await new Promise(resolve => server.listen(0, '127.0.0.1', resolve));
const base = `http://127.0.0.1:${server.address().port}`;
await mkdir(output, { recursive: true });
let browser;
let checks = 0;
const failures = [];

async function check(label, fn) {
  try { await fn(); checks++; console.log(`PASS ${label}`); }
  catch (error) { failures.push({ label, error }); console.error(`FAIL ${label}: ${error.message}`); }
}
async function duplicateIds(page) {
  const duplicates = await page.evaluate(() => {
    const counts = new Map(); document.querySelectorAll('[id]').forEach(el => counts.set(el.id, (counts.get(el.id) || 0) + 1));
    return [...counts].filter(([, count]) => count > 1).map(([id]) => id);
  });
  assert.deepEqual(duplicates, [], 'DOM IDs must stay unique when original cards move into new pages');
}
async function noOverflow(page, name) {
  const sizes = await page.evaluate(() => ({ viewport: innerWidth, content: document.documentElement.scrollWidth }));
  assert.ok(sizes.content <= sizes.viewport + 1, `${name}: global horizontal overflow ${sizes.content}px > ${sizes.viewport}px`);
}
async function navigate(page, name) {
  await page.evaluate(name => showPage(name, document.getElementById('nav-' + name)), name);
  await page.waitForTimeout(160);
  assert.equal(await page.locator('.page.active').count(), 1, `${name}: exactly one active page`);
  assert.equal(await page.locator(`#page-${name}.active`).count(), 1, `${name}: requested page is active`);
  assert.equal(await page.locator('.nav-item.active').count(), 1, `${name}: exactly one active page navigation item`);
}
async function open(settings = {}, suffix = '/', viewport = { width: 1440, height: 1100 }, storage = {}) {
  scenario = { authenticated: true, auth_required: true, public_overview: true, wifi: true, ...settings };
  requests = [];
  const context = await browser.newContext({ viewport });
  await context.addInitScript(storage => {
    if (storage.blocked) {
      Object.defineProperty(window, 'localStorage', { configurable: true,
        get() { throw new DOMException('Storage disabled for test', 'SecurityError'); } });
    } else {
      if (storage.theme) localStorage.setItem('netcore-theme', storage.theme);
      if (storage.legacyTheme) localStorage.setItem('fs_theme', storage.legacyTheme);
    }
  }, storage);
  const page = await context.newPage();
  const errors = [];
  page.on('pageerror', error => errors.push(error.message));
  // Do not allow fixture map tiles, callsign registries or any other external request to escape this test.
  await page.route('**/*', route => route.request().url().startsWith(base) ? route.continue() : route.abort());
  await page.routeWebSocket('**/ws', socket => {
    requests.push({ method: 'WS', path: '/ws' });
    socket.onMessage(message => {
      const value = JSON.parse(message);
      if (value.type === 'subscribe') {
        socket.send(JSON.stringify(snapshot));
        socket.send(JSON.stringify(txVisual));
      }
    });
  });
  await page.goto(base + suffix);
  await page.waitForTimeout(450);
  return { context, page, errors };
}
async function darkSurfaces(page, selector = '.page.active') {
  const problems = await page.evaluate(selector => {
    const rgb = value => value.match(/[\d.]+/g)?.map(Number) || [];
    const bright = value => { const values = rgb(value); return values.length >= 3 &&
      (values.length < 4 || values[3] > .95) && Math.max(...values.slice(0, 3)) > 160; };
    const visible = node => node.getBoundingClientRect().width > 0 && node.getBoundingClientRect().height > 0;
    const scope = document.querySelector(selector);
    if (!scope) return ['Missing scope ' + selector];
    return [...scope.querySelectorAll('.card,.stat-card,.hero,.modal,.sheet,.read-pop,input:not([type=checkbox]):not([type=radio]):not([type=range]),select,textarea')]
      .filter(visible).filter(node => bright(getComputedStyle(node).backgroundColor))
      .map(node => node.id || node.className || node.tagName);
  }, selector);
  assert.deepEqual(problems, [], 'Dark surfaces and form fields must not keep light backgrounds');
  assert.equal(await page.locator('html').getAttribute('data-theme'), 'dark');
}
function assertNoPrivilegedRequests() {
  const privileged = requests.filter(r => (r.path.startsWith('/api/') && !['/api/session', '/api/public', '/api/login'].includes(r.path)) || r.method === 'WS');
  assert.deepEqual(privileged, [], 'anonymous mode must neither probe privileged endpoints nor open a WebSocket');
}

try {
  browser = await playwright.chromium.launch({ headless: true,
    ...(process.env.CHROMIUM_EXECUTABLE_PATH ? { executablePath: process.env.CHROMIUM_EXECUTABLE_PATH } : {}),
    args: ['--no-sandbox'] });
  const admin = await open({}, '/?intern=netcore');
  await check('authenticated boot opens a telemetry WebSocket', async () => {
    assert.ok(requests.some(r => r.method === 'WS'));
    await admin.page.waitForFunction(() => document.getElementById('ms-tbody')?.textContent.includes('990001'));
  });
  await check('initial host and cell snapshots each load once', async () => {
    for (const route of ['/api/system', '/api/btsinfo']) {
      assert.equal(requests.filter(request => request.method === 'GET' && request.path === route).length, 1,
        `${route}: duplicated boot requests delay useful telemetry`);
    }
  });
  await check('authenticated controls and original callbacks remain', async () => {
    assert.ok(await admin.page.locator('#logout-btn').isVisible());
    assert.equal(await admin.page.locator('#config-editor').count(), 1);
    assert.equal(await admin.page.locator('#dual-carrier-toggle').count(), 1);
  });
  await check('horizontal main/context navigation works with mouse and keyboard', async () => {
    await admin.page.locator('#nc-main-nav [data-group="radio"]').click();
    assert.equal(await admin.page.locator('#page-stations.active').count(), 1);
    await admin.page.locator('#nav-calls').click();
    assert.equal(await admin.page.locator('#page-calls.active').count(), 1);
    await admin.page.locator('#nav-lastheard').focus();
    await admin.page.locator('#nav-lastheard').press('Enter');
    assert.equal(await admin.page.locator('#page-lastheard.active').count(), 1);
    assert.equal(await admin.page.locator('#nc-main-nav [aria-current="page"]').count(), 1);
    assert.equal(await admin.page.locator('#nc-subnav [aria-current="page"]').count(), 1);
  });
  for (const name of dashboardPages) {
    await check(`desktop view ${name}`, async () => {
      await navigate(admin.page, name); await duplicateIds(admin.page); await noOverflow(admin.page, name);
      await admin.page.screenshot({ path: path.join(output, `${name}.png`) });
      assert.deepEqual(admin.errors, [], `${name}: JavaScript must not throw`);
    });
  }
  await check('RF spectrum/waterfall left and quality metrics right', async () => {
    await navigate(admin.page, 'rf');
    const spectrum = await admin.page.locator('#rf-spectrum').boundingBox();
    const waterfall = await admin.page.locator('#rf-waterfall').boundingBox();
    const metric = await admin.page.locator('#rf-evm').boundingBox();
    assert.ok(spectrum && waterfall && metric, 'RF canvas and values must be visible');
    assert.ok(waterfall.y > spectrum.y, 'waterfall is below spectrum');
    assert.ok(Math.abs(waterfall.x - spectrum.x) < 25, 'waterfall and spectrum share the left column');
    assert.ok(metric.x >= spectrum.x + spectrum.width - 2, 'actual TX quality readouts occupy the right column');
    assert.match(await admin.page.locator('#rf-rms').textContent(), /dBFS/);
  });
  await check('secondary carrier air TS2 maps to logical TS5', async () => {
    const model = await admin.page.evaluate(() => ({ carrier: tsSecondaryCarrierFromActivity(), logical: tsNormalizeLogicalTs(102, 2), air: tsAirTs(5) }));
    assert.deepEqual(model, { carrier: 102, logical: 5, air: 2 });
    await navigate(admin.page, 'home');
    const tiles = admin.page.locator('[data-carrier="102"][data-ts="5"][data-air-ts="2"]');
    assert.ok(await tiles.count() > 0, 'actual secondary carrier tile is rendered');
  });
  await check('SDS filter uses real fetched rows', async () => {
    await navigate(admin.page, 'sdslog');
    assert.match(await admin.page.locator('#sdslog-tbody').textContent(), /UI fixture message/);
    await admin.page.locator('#sdslog-from-filter').fill('999999');
    assert.doesNotMatch(await admin.page.locator('#sdslog-tbody').textContent(), /UI fixture message/);
    await admin.page.locator('#sdslog-from-filter').fill('990001');
    assert.match(await admin.page.locator('#sdslog-tbody').textContent(), /UI fixture message/);
  });
  await check('radio filtering and keyboard detail selection use live registry data', async () => {
    await navigate(admin.page, 'stations');
    await admin.page.locator('#nc-radio-filter').fill('Backup');
    assert.equal(await admin.page.locator('#ms-tbody tr:visible').count(), 1);
    assert.match(await admin.page.locator('#ms-tbody tr:visible').textContent(), /990002/);
    await admin.page.locator('#nc-radio-filter').fill('');
    const radio = admin.page.locator('#ms-tbody tr[data-nc-issi="990001"]');
    await radio.focus(); await radio.press('Enter');
    assert.match(await admin.page.locator('#nc-radio-detail').textContent(), /UI Test Radio/);
    assert.match(await admin.page.locator('#nc-radio-detail').textContent(), /990001/);
    assert.match(await admin.page.locator('#nc-radio-detail').textContent(), /-34\.2 dBFS/);
  });
  await check('neighbors display configuration without fabricated reachability', async () => {
    await navigate(admin.page, 'neighbors');
    assert.match(await admin.page.locator('#nc-neighbor-count').textContent(), /1 Nachbar/);
    assert.match(await admin.page.locator('#nc-neighbor-rows').textContent(), /103/);
    assert.match(await admin.page.locator('#page-neighbors').textContent(), /nicht gemessen/);
  });
  await check('service page renders the full reported service matrix', async () => {
    await navigate(admin.page, 'services');
    assert.equal(await admin.page.locator('#core-services-grid .core-service-card').count(), 17);
    assert.equal(await admin.page.locator('#core-count-status').textContent(), '17 / 17');
  });
  await check('recording preview uses the actual audio player', async () => {
    await navigate(admin.page, 'recordings');
    const recording = admin.page.locator('#rec-tbody tr[data-recording-id="ui-test-recording"]');
    await recording.locator('button[onclick^="playRecording"]').click();
    await admin.page.waitForFunction(() => document.getElementById('rec-player').readyState >= 2);
    assert.ok(await admin.page.locator('#rec-player-box').isVisible());
    assert.ok(requests.some(request => request.path === '/api/recordings/ui-test-recording/audio'));
    assert.match(await admin.page.locator('#rec-player-title').textContent(), /UI Fixture Recording/);
    await admin.page.locator('#rec-player').evaluate(player => player.pause());
  });
  await check('recording resend exposes transmission progress and stop', async () => {
    const recording = admin.page.locator('#rec-tbody tr[data-recording-id="ui-test-recording"]');
    await recording.getByRole('button', { name: 'Senden', exact: true }).click();
    assert.equal(await admin.page.locator('#page-recordings.active').count(), 1, 'choose a target in the recording context');
    await admin.page.locator('#audio-target-type').selectOption('group');
    await admin.page.locator('#audio-target-manual').fill('990100');
    await admin.page.locator('#audio-priority').fill('7');
    await admin.page.locator('#audio-send-card').getByRole('button', { name: 'Jetzt senden' }).click();
    await admin.page.waitForFunction(() => document.getElementById('page-audio').classList.contains('active')
      && document.getElementById('audio-state').textContent === 'SENDET');
    assert.deepEqual(requests.find(request => request.path === '/api/audio/play' && request.method === 'POST')?.body,
      { source_type: 'recording', target_type: 'group', target_id: 990100, priority: 7, recording_id: 'ui-test-recording' });
    assert.ok(await admin.page.locator('#audio-stop').isVisible());
    assert.ok(await admin.page.locator('#audio-stop').isEnabled());
    assert.match(await admin.page.locator('#audio-progress').textContent(), /00:01 \/ 00:06/);
    await admin.page.locator('#audio-stop').click();
    await admin.page.waitForFunction(() => document.getElementById('audio-state').textContent === 'BEREIT');
    assert.ok(requests.some(request => request.path === '/api/audio/stop' && request.method === 'POST'));
    assert.ok(!(await admin.page.locator('#audio-stop').isEnabled()));
    await duplicateIds(admin.page);
  });
  await check('dark theme and larger text persist', async () => {
    const before = await admin.page.evaluate(() => getComputedStyle(document.body).backgroundColor);
    const fontBefore = await admin.page.evaluate(() => parseFloat(getComputedStyle(document.body).fontSize));
    await admin.page.locator('.theme-btn[data-t="dark"]').click();
    await admin.page.evaluate(() => setUiSize('h'));
    assert.equal(await admin.page.evaluate(() => localStorage.getItem('fs_theme')), 'dark');
    assert.equal(await admin.page.evaluate(() => localStorage.getItem('netcore-theme')), 'dark');
    assert.equal(await admin.page.locator('.theme-btn[data-t="dark"]').getAttribute('aria-pressed'), 'true');
    assert.equal(await admin.page.locator('html').getAttribute('data-uisize'), 'h');
    assert.ok(await admin.page.evaluate(() => parseFloat(getComputedStyle(document.body).fontSize)) > fontBefore,
      'the supported high-readability setting must increase actual rendered text');
    assert.notEqual(await admin.page.evaluate(() => getComputedStyle(document.body).backgroundColor), before);
    await navigate(admin.page, 'system');
    await admin.page.screenshot({ path: path.join(output, 'system-dark.png') });
    await admin.page.evaluate(() => setUiSize('m'));
  });
  for (const name of dashboardPages) {
    await check(`desktop dark view ${name}`, async () => {
      await navigate(admin.page, name); await darkSurfaces(admin.page); await noOverflow(admin.page, name);
      await admin.page.screenshot({ path: path.join(output, `${name}-dark.png`) });
      assert.deepEqual(admin.errors, []);
    });
  }
  await check('RF spectrum and waterfall recolour immediately after a theme click', async () => {
    await navigate(admin.page, 'rf');
    await admin.page.locator('.theme-btn[data-t="light"]').click();
    await admin.page.waitForFunction(() => document.getElementById('rf-spectrum').getContext('2d').getImageData(0,0,1,1).data[0] > 200);
    await admin.page.locator('.theme-btn[data-t="dark"]').click();
    await admin.page.waitForFunction(() => ['rf-spectrum','rf-waterfall'].every(id => {
      const pixel = document.getElementById(id).getContext('2d').getImageData(0,0,1,1).data;
      return pixel[0] < 50 && pixel[1] < 60 && pixel[3] === 255;
    }));
  });
  await check('all station dialogs and readability settings inherit dark surfaces', async () => {
    const overlays = await admin.page.locator('.modal-overlay,.sheet-overlay').evaluateAll(nodes => nodes.map(node => node.id));
    for (const id of overlays) {
      await admin.page.evaluate(id => document.getElementById(id).classList.add('open'), id);
      await darkSurfaces(admin.page, '#' + id);
      await admin.page.evaluate(id => document.getElementById(id).classList.remove('open'), id);
    }
    await admin.page.locator('#read-btn').click();
    assert.ok(await admin.page.locator('#read-pop').isVisible());
    await darkSurfaces(admin.page, '.eye-wrap');
    await admin.page.evaluate(() => closeReadPop());
  });
  await check('Blue remains available and restores from the shared preference', async () => {
    await admin.page.locator('.theme-btn[data-t="blue"]').click();
    assert.equal(await admin.page.evaluate(() => localStorage.getItem('netcore-theme')), 'blue');
    await admin.page.reload();
    await admin.page.waitForFunction(() => typeof showPage === 'function');
    assert.equal(await admin.page.locator('html').getAttribute('data-theme'), 'blue');
    assert.equal(await admin.page.locator('.theme-btn[data-t="blue"]').getAttribute('aria-pressed'), 'true');
    await admin.page.locator('.theme-btn[data-t="light"]').click();
  });
  await admin.context.close();

  const legacyDark = await open({}, '/?intern=netcore', undefined, { legacyTheme: 'dark' });
  await check('legacy station theme migrates to the shared preference', async () => {
    await darkSurfaces(legacyDark.page);
    assert.equal(await legacyDark.page.evaluate(() => localStorage.getItem('netcore-theme')), 'dark');
    assert.deepEqual(legacyDark.errors, []);
  });
  await legacyDark.context.close();
  const canonicalDark = await open({}, '/?intern=netcore', undefined, { theme: 'dark', legacyTheme: 'light' });
  await check('canonical theme takes precedence over older station preference', async () => {
    await darkSurfaces(canonicalDark.page);
    assert.equal(await canonicalDark.page.locator('.theme-btn[data-t="dark"]').getAttribute('aria-pressed'), 'true');
    assert.deepEqual(canonicalDark.errors, []);
  });
  await canonicalDark.context.close();
  const blockedDashboard = await open({}, '/?intern=netcore', undefined, { blocked: true });
  await check('blocked storage does not prevent station boot or theme switching', async () => {
    await blockedDashboard.page.waitForFunction(() => document.getElementById('ms-tbody')?.textContent.includes('990001'));
    await blockedDashboard.page.locator('.theme-btn[data-t="dark"]').click();
    await darkSurfaces(blockedDashboard.page);
    assert.deepEqual(blockedDashboard.errors, []);
  });
  await blockedDashboard.context.close();

  const normal = await open({ wifi: false });
  await check('laboratory integrations stay hidden by default', async () => {
    for (const name of labPages) {
      assert.ok(!(await normal.page.locator(`#nav-${name}`).isVisible()), `${name} stays hidden`);
      await normal.page.evaluate(name => showPage(name), name);
      assert.equal(await normal.page.locator(`#page-${name}.active`).count(), 0, `${name} cannot bypass the normal navigation`);
    }
  });
  await check('WLAN navigation is hidden without NetworkManager', async () => {
    assert.ok(!(await normal.page.locator('#nav-wifi').isVisible()));
  });
  await check('normal boot has no JavaScript errors', async () => assert.deepEqual(normal.errors, []));
  await normal.context.close();

  const mobile = await open({}, '/?intern=netcore', { width: 390, height: 844 });
  for (const name of dashboardPages) {
    await check(`mobile width 390 ${name}`, async () => {
      await navigate(mobile.page, name); await noOverflow(mobile.page, name);
      assert.deepEqual(mobile.errors, []);
    });
  }
  await navigate(mobile.page, 'rf');
  await mobile.page.screenshot({ path: path.join(output, 'rf-mobile.png') });
  await mobile.page.locator('.theme-btn[data-t="dark"]').click();
  for (const name of dashboardPages) {
    await check(`mobile dark width 390 ${name}`, async () => {
      await navigate(mobile.page, name); await darkSurfaces(mobile.page); await noOverflow(mobile.page, name);
      assert.deepEqual(mobile.errors, []);
    });
  }
  await navigate(mobile.page, 'rf');
  await mobile.page.screenshot({ path: path.join(output, 'rf-mobile-dark.png'), fullPage: true });
  await mobile.page.locator('.theme-btn[data-t="light"]').click();
  await check('ultra readability fits mobile navigation and key workspaces', async () => {
    const fontBefore = await mobile.page.evaluate(() => parseFloat(getComputedStyle(document.body).fontSize));
    await mobile.page.evaluate(() => setUiSize('u'));
    assert.ok(await mobile.page.evaluate(() => parseFloat(getComputedStyle(document.body).fontSize)) > fontBefore);
    for (const name of ['home', 'rf', 'recordings', 'config']) { await navigate(mobile.page, name); await noOverflow(mobile.page, name); }
    await mobile.page.locator('#nc-main-nav [data-group="diagnostics"]').click();
    assert.equal(await mobile.page.locator('#page-services.active').count(), 1, 'main navigation restores the last diagnostics page');
    await mobile.page.locator('#nav-health').click();
    assert.equal(await mobile.page.locator('#page-health.active').count(), 1);
    await mobile.page.evaluate(() => setUiSize('m'));
  });
  await mobile.context.close();

  const anonymous = await open({ authenticated: false });
  await check('anonymous public view isolates all privileged traffic', async () => {
    assert.equal(await anonymous.page.locator('#page-public.active').count(), 1);
    await anonymous.page.waitForTimeout(3150); // Include one actual poll cadence in the access assertion.
    assertNoPrivilegedRequests();
    assert.ok(await anonymous.page.locator('#login-btn').isVisible());
    assert.ok(!(await anonymous.page.locator('#logout-btn').isVisible()));
    await duplicateIds(anonymous.page); await noOverflow(anonymous.page, 'public');
    await anonymous.page.screenshot({ path: path.join(output, 'public.png') });
    assert.deepEqual(anonymous.errors, []);
  });
  await check('anonymous navigation cannot fetch admin pages', async () => {
    await anonymous.page.evaluate(() => showPage('config', document.getElementById('nav-config')));
    assert.equal(await anonymous.page.locator('#page-config.active').count(), 0);
    assertNoPrivilegedRequests();
  });
  await check('anonymous public overview supports dark mode without privileged traffic', async () => {
    await anonymous.page.locator('.theme-btn[data-t="dark"]').click();
    await darkSurfaces(anonymous.page, '#page-public');
    assertNoPrivilegedRequests();
    await anonymous.page.screenshot({ path: path.join(output, 'public-dark.png') });
    assert.deepEqual(anonymous.errors, []);
  });
  await anonymous.context.close();

  const sessionFailure = await open({ authenticated: false, session_status: 503 });
  await check('unavailable session status fails closed', async () => {
    await sessionFailure.page.evaluate(() => showPage('config', document.getElementById('nav-config')));
    assertNoPrivilegedRequests();
    assert.deepEqual(sessionFailure.errors, []);
  });
  await sessionFailure.context.close();

  const privateDashboard = await open({ authenticated: false, public_overview: false });
  await check('anonymous private dashboard redirects to login without privileged traffic', async () => {
    await privateDashboard.page.waitForURL(base + '/login');
    assertNoPrivilegedRequests();
    assert.ok(await privateDashboard.page.locator('#login-form').isVisible());
    assert.deepEqual(privateDashboard.errors, []);
  });
  await privateDashboard.context.close();

  const noAuth = await open({ auth_required: false });
  await check('deployment without configured authentication retains dashboard access', async () => {
    assert.ok(requests.some(r => r.method === 'WS'));
    await navigate(noAuth.page, 'config');
    assert.match(await noAuth.page.locator('#config-editor').inputValue(), /UI test config/);
    assert.deepEqual(noAuth.errors, []);
  });
  await noAuth.context.close();

  const fallback = await open({ edge_isolated: true });
  await check('isolated service plane preserves fallback and security explanations', async () => {
    await navigate(fallback.page, 'services');
    assert.match(await fallback.page.locator('#core-mode-status').textContent(), /LOKALER FALLBACK/);
    assert.ok(await fallback.page.locator('#edge-fallback-banner').isVisible());
    const services = await fallback.page.locator('#core-services-grid').textContent();
    assert.match(services, /keine unsichere Herabstufung/);
    assert.match(services, /kein OTAR/);
    assert.deepEqual(fallback.errors, []);
  });
  await fallback.context.close();

  for (const publicOverview of [true, false]) {
    const login = await open({ authenticated: false, public_overview: publicOverview }, '/login');
    await check(`login public_overview=${publicOverview}`, async () => {
      assert.ok(await login.page.locator('#login-form').isVisible());
      assertNoPrivilegedRequests();
      if (!publicOverview) assert.ok(requests.filter(r => r.path === '/api/public').length <= 1,
        'disabled public overview must not start repeated public polling');
      if (publicOverview) {
        assert.equal(await login.page.locator('#public-devices').textContent(), '2');
        assert.match(await login.page.locator('#public-load').textContent(), /Nicht verfügbar/,
          'a missing CPU measurement must not appear as zero percent');
      } else {
        assert.equal(await login.page.locator('#public-status').getAttribute('data-state'), 'disabled');
      }
      await duplicateIds(login.page); await noOverflow(login.page, 'login');
      await login.page.screenshot({ path: path.join(output, publicOverview ? 'login.png' : 'login-private.png') });
      assert.deepEqual(login.errors, []);
    });
    await check(`login POST and invalid-credential feedback public=${publicOverview}`, async () => {
      await login.page.locator('#username').fill('ui-test');
      await login.page.locator('#password').fill('test-only-invalid-password');
      await login.page.locator('#submit-btn').click();
      await login.page.waitForFunction(() => document.getElementById('err').textContent.length > 0);
      assert.ok(requests.some(r => r.method === 'POST' && r.path === '/api/login' && r.body?.user === 'ui-test'));
      assert.ok(await login.page.locator('#submit-btn').isEnabled());
      assertNoPrivilegedRequests();
    });
    await check(`login dark toggle and persistence public=${publicOverview}`, async () => {
      await login.page.locator('#login-theme-toggle').focus();
      await login.page.locator('#login-theme-toggle').press('Enter');
      await darkSurfaces(login.page, '.login-card');
      assert.equal(await login.page.locator('#login-theme-toggle').getAttribute('aria-pressed'), 'true');
      assert.equal(await login.page.evaluate(() => localStorage.getItem('netcore-theme')), 'dark');
      await login.page.reload();
      await login.page.waitForSelector('#login-form');
      assert.equal(await login.page.locator('html').getAttribute('data-theme'), 'dark');
      await login.page.screenshot({ path: path.join(output, publicOverview ? 'login-dark.png' : 'login-private-dark.png') });
      assert.deepEqual(login.errors, []);
    });
    if (publicOverview) await check('login and authenticated station reuse the same dark/light preference', async () => {
      scenario.authenticated = true;
      await login.page.goto(base + '/?intern=netcore');
      await login.page.waitForFunction(() => document.getElementById('ms-tbody')?.textContent.includes('990001'));
      await darkSurfaces(login.page);
      await login.page.locator('.theme-btn[data-t="light"]').click();
      scenario.authenticated = false; requests = [];
      await login.page.goto(base + '/login');
      await login.page.waitForSelector('#login-theme-toggle');
      assert.equal(await login.page.locator('html').getAttribute('data-theme'), 'light');
      assert.equal(await login.page.locator('#login-theme-toggle').getAttribute('aria-pressed'), 'false');
      assertNoPrivilegedRequests();
      assert.deepEqual(login.errors, []);
    });
    await login.context.close();
  }
  const mobileLogin = await open({ authenticated: false }, '/login', { width: 390, height: 844 });
  await check('mobile login fits and keeps touch inputs readable', async () => {
    await noOverflow(mobileLogin.page, 'mobile login');
    const size = await mobileLogin.page.locator('#username').evaluate(el => parseFloat(getComputedStyle(el).fontSize));
    assert.ok(size >= 16, 'input size avoids mobile keyboard auto-zoom');
    await mobileLogin.page.screenshot({ path: path.join(output, 'login-mobile.png') });
    assert.deepEqual(mobileLogin.errors, []);
  });
  await check('mobile dark login retains readable inputs and complete logo', async () => {
    await mobileLogin.page.locator('#login-theme-toggle').click();
    await darkSurfaces(mobileLogin.page, '.login-card');
    await noOverflow(mobileLogin.page, 'mobile dark login');
    // Let the existing 150 ms button transition finish before capturing its final theme.
    await mobileLogin.page.waitForFunction(() => {
      const color = getComputedStyle(document.getElementById('submit-btn')).backgroundColor.match(/[\d.]+/g)?.map(Number);
      return color && color[0] > 100;
    });
    await mobileLogin.page.screenshot({ path: path.join(output, 'login-mobile-dark.png'), fullPage: true });
  });
  await mobileLogin.context.close();
  const blockedLogin = await open({ authenticated: false }, '/login', undefined, { blocked: true });
  await check('blocked storage keeps login submission and theme toggle usable', async () => {
    await blockedLogin.page.locator('#login-theme-toggle').click();
    await darkSurfaces(blockedLogin.page, '.login-card');
    await blockedLogin.page.locator('#username').fill('ui-test');
    await blockedLogin.page.locator('#password').fill('test-only-invalid-password');
    await blockedLogin.page.locator('#submit-btn').click();
    await blockedLogin.page.waitForFunction(() => document.getElementById('err').textContent.length > 0);
    assert.deepEqual(blockedLogin.errors, []);
  });
  await blockedLogin.context.close();
} finally {
  if (browser) await browser.close();
  await new Promise(resolve => server.close(resolve));
}
console.log(`\n${checks} checks passed; ${failures.length} failed. Screenshots: target/dashboard-preview/`);
if (failures.length) process.exitCode = 1;
