#!/usr/bin/env node
// Exercise the actual service templates and shared shell with isolated HTTP API fixtures.
// The fixtures are test data only; no production service or radio is contacted.
import assert from 'node:assert/strict';
import http from 'node:http';
import { readFile, mkdir } from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { createRequire } from 'node:module';
import { spawnSync } from 'node:child_process';

const require = createRequire(import.meta.url);
let playwright;
try { playwright = require('playwright'); }
catch (error) {
  if (!process.env.CODEX_PRIMARY_RUNTIME_NODE_MODULES) throw error;
  playwright = require(path.join(process.env.CODEX_PRIMARY_RUNTIME_NODE_MODULES, 'playwright'));
}
const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const output = path.join(root, 'target/service-ui-preview/media');
const labels = {
  'security-core': 'Security Core', kmf: 'Key Management Facility', transit: 'Transit',
  'application-gateway': 'Application Gateway', 'media-library': 'Media Library', recorder: 'Recorder',
  observability: 'Observability', 'provisioning-core': 'Provisioning Core', 'iot-gateway': 'IoT Gateway',
};
const now = '2026-10-03T00:00:00Z';
const assetId = '87ca6206-e783-4c71-9356-696a28c571a8';
const keyId = '9d832d88-fc39-4b89-a7bb-90ebf59c6c37';
const asset = { asset_id: assetId, title: 'UI Test Durchsage', source: 'tts', original_filename: 'durchsage.wav',
  kind: 'tts', tags: ['test'], size_bytes: 16044, sha256: 'a'.repeat(64), state: 'ready', approval: 'approved',
  metadata: { format: 'WAV', codec: 'PCM16', duration_ms: 1000, tetra_frame_count: null },
  preview_ready: true, broadcast_ready: false, archived: false, approved_by: 'test', last_error: null };
const profile = { issi: 4010001, display_name: 'UI Test Teilnehmer', authentication_required: true,
  minimum_security_class: 1, preferred_security_class: 3, allowed_nodes: ['tbs-lab-01'], max_failures: 3,
  disabled: false, equipment_disabled: false, notes: 'Testprofil' };
const subscriber = { ...profile, organization: 'UI Test', device_label: 'Test Radio', home_mcc: 901, home_mnc: 99,
  enabled: true, registration_allowed: true, sds_allowed: true, emergency_allowed: false,
  packet_data_allowed: false, call_priority: 0, default_groups: [] };
const group = { gssi: 2000, name: 'UI Test Gruppe', enabled: true, attach_allowed: true, call_allowed: true,
  sds_allowed: true, emergency_allowed: false, dgna_allowed: true, class_of_usage: 4, call_priority: 0, notes: '' };
const membership = { issi: 4010001, gssi: 2000, allowed: true, auto_attach: true, locked: false, notes: 'UI Test' };
const key = { id: keyId, kind: 'CCK', scope: 'network', scope_value: null, label: 'UI Test Key', key_bytes: 32,
  version: 1, state: 'active', fingerprint: 'b'.repeat(64), crypto_period_start: now,
  crypto_period_end: '2026-12-31T00:00:00Z' };
const recording = (id, hold) => ({ id, session_id: 'session-test', started_at: now, emergency: false,
  call_kind: 'group', gssi: 2000, calling_issi: 4010001, called_issi: null, source_nodes: ['tbs-lab-01'],
  frame_count: 1000, duration_ms: 60000, audio_bytes: 35000, lost_tap_frames: 0,
  integrity_status: 'verified', legal_hold: hold, retention_until: '2026-11-03T00:00:00Z' });
const target = { target_id: 'recorder-01', display_name: 'Recorder Test', service: 'recorder',
  base_url: 'http://127.0.0.1:8160', enabled: true, live: true, ready: true, metrics_ok: true,
  response_ms: 18, series_count: 1, last_scrape_at: now, last_error: null };
const connector = { connector_id: 'sds-test', display_name: 'SDS Router Test', kind: 'sds_router', direction: 'outbound',
  endpoint: 'http://127.0.0.1:8150/api/v1/messages', health_endpoint: null, health: 'healthy', circuit_state: 'closed',
  sent_total: 2, failed_total: 0, received_total: 0, required_secrets: [], enabled: true,
  timeout_ms: 5000, rate_limit_per_minute: 60, settings: {} };
const policy = { operating_mode: 'shadow', default_security_class: 1, minimum_security_class: 1,
  authentication_required: true, allow_class1_fallback: true, reject_unknown_subscribers: false,
  disable_after_failures: false };
const fixtures = {
  'security-core': {
    '/api/v1/status': { operating_mode: 'shadow', authoritative: false, profiles: 1, active_auth_contexts: 1,
      active_dck_contexts: 0, open_alarms: 0, node_gateway_connected: true, node_gateway_last_error: null },
    '/api/v1/profiles': [profile], '/api/v1/subscribers': [], '/api/v1/policy': policy,
    '/api/v1/auth-contexts': [{ id: 'context-test-0001', issi: 4010001, node_id: 'tbs-lab-01', requested_security_class: 3,
      negotiated_security_class: 3, state: 'authenticated', attempts: 1, max_attempts: 3,
      challenge_fingerprint: 'c123', response_fingerprint: 'r123' }],
    '/api/v1/dck-contexts': [], '/api/v1/actions': [], '/api/v1/alarms': [], '/api/v1/nodes': [], '/api/v1/audit': [],
  },
  kmf: {
    '/api/v1/status': { operating_mode: 'shadow', authoritative: false, total_keys: 1, active_keys: 1,
      node_transport_profiles: 0, enabled_nodes: 0, otar_jobs: 0, pending_actions: 0, vault_ready: true,
      vault_provider: 'lab_file_vault' },
    '/api/v1/keys': [key], '/api/v1/nodes': [], '/api/v1/otar/jobs': [], '/api/v1/otar/actions': [],
    '/api/v1/audit': [], '/api/v1/backups': [], '/api/v1/policy': { operating_mode: 'shadow', default_key_bytes: 32,
      default_crypto_period_secs: 86400, rotation_lead_secs: 3600, require_dual_approval: true,
      allow_overlapping_crypto_periods: true, auto_retire_predecessor: true },
  },
  transit: {
    '/api/v1/status': { region_id: 'region-a', swmi_id: 'netcore-a', operating_mode: 'shadow', authoritative: false,
      peers_up: 1, peers_total: 1, routes_total: 1, sessions_active: 0, outbound_pending: 0,
      local_deliveries_pending: 0, loop_rejections: 0 },
    '/api/v1/config': {}, '/api/v1/peers': [{ peer_id: 'region-b-primary', region_id: 'region-b', display_name: 'Region B Test',
      endpoint: 'http://127.0.0.1:8200', admin_state: 'enabled', oper_state: 'up', latency_ms: 18,
      protocol_version: 'netcore-transit-v1', capabilities: ['sds'], last_error: null }],
    '/api/v1/routes': [{ route_id: 'route-test', service: 'sds', selector_type: 'default', selector_value: null,
      destination_region: 'region-b', peer_id: 'region-b-primary', preference: 100, metric: 100, enabled: true, failover_group: null }],
    '/api/v1/locations/subscribers': [], '/api/v1/locations/groups': [], '/api/v1/sessions': [],
    '/api/v1/outbound': [], '/api/v1/local-deliveries': [], '/api/v1/events': [],
  },
  'application-gateway': {
    '/api/v1/status': { operating_mode: 'shadow', connectors_healthy: 1, connectors_enabled: 1, circuits_open: 0,
      events_total: 2, events_unrouted: 0, deliveries_queued: 0, deliveries_retry: 0, deliveries_dead_letter: 0,
      tts_jobs_ready: 0, tts_jobs_total: 0, missing_required_secrets: 0 },
    '/api/v1/connectors': [connector], '/api/v1/rules': [], '/api/v1/templates': [], '/api/v1/events': [],
    '/api/v1/deliveries': [], '/api/v1/tts/jobs': [], '/api/v1/audit': [], '/api/v1/backups': [],
  },
  'media-library': {
    '/api/v1/status': { operating_mode: 'authoritative', assets_total: 1, assets_ready: 1, assets_approved: 1, preview_ready: 1,
      broadcast_ready: 0, assets_importing: 0, jobs_queued: 0, jobs_playing: 0, storage_used_bytes: 16044,
      media_switch_connected: true, recorder_connected: true, application_gateway_connected: true },
    '/api/v1/config': { playout_mode: 'basisstation', playout_default_station: 'tbs-lab-01',
      playout_stations: [{ id: 'tbs-lab-01', name: 'TBS Lab 01', base_url: 'http://127.0.0.1:8080', enabled: true }] },
    '/api/v1/assets': [asset], ['/api/v1/assets/' + assetId]: asset,
    ['/api/v1/assets/' + assetId + '/waveform']: { points: [0.1, 0.2, 0.4, 0.2, 0.1] },
    '/api/v1/jobs': [], '/api/v1/events': [], '/api/v1/audit': [], '/api/v1/recorder/recordings': [],
    '/api/v1/tts/status': { provider_available: true, provider_endpoint: 'http://127.0.0.1:5000',
      default_voice: 'de-test', default_speed: 0.95, auto_approve_tts: false, max_text_characters: 2000 },
    '/api/v1/tts/voices': { voices: [{ id: 'de-test', name: 'Deutsch Test', available: true }] },
    '/api/v1/tts/templates': { templates: [] },
  },
  recorder: {
    '/api/v1/status': { active_recordings: 0, completed_recordings: 2, frames_ingested: 2000, frames_lost_before_recorder: 0,
      storage_used_bytes: 70000, storage_free_bytes: 1073741824, media_cursor: 2000, recordings_recovered: 0,
      media_switch_connected: true, storage_available: true },
    '/api/v1/active': [], '/api/v1/recordings': [recording('recording-held', true), recording('recording-free', false)],
    '/api/v1/events': [],
  },
  observability: {
    '/api/v1/status': { targets_up: 1, targets_total: 1, targets_ready: 1, series: 1, logs: 0, traces: 0,
      alerts_firing: 0, alerts_unacknowledged: 0, stack_ready: 1, stack_total: 1 },
    '/api/v1/targets': [target], '/api/v1/stack': [{ component: 'Grafana', ready: true,
      endpoint: 'http://127.0.0.1:3000', response_ms: 18, last_error: null }], '/api/v1/alerts': [],
    '/api/v1/rules': [], '/api/v1/silences': [], '/api/v1/metrics/catalog': [{ name: 'netcore_observability_target_up', series: 1 }],
    '/api/v1/metrics/series': [{ name: 'netcore_observability_target_up', target_id: 'recorder-01',
      labels: { service: 'recorder', target_id: 'recorder-01' }, last_value: 1, last_at: now, samples: [{ value: 1, at: now }] }],
    '/api/v1/config': {}, '/api/v1/logs': [], '/api/v1/traces': [], '/api/v1/audit': [], '/api/v1/diagnostics': [],
    '/api/v1/discovery': { enabled: false, controller_url: 'http://10.0.20.35:8320',
      last_success: null, error: null, using_cache: true },
    '/api/v1/syslog': { receiver: null, archive: null },
  },
  'provisioning-core': {
    '/api/v1/dashboard': { subscribers: [subscriber], groups: [group], memberships: [membership],
      observed: [{ issi: 4010001, registered: true, serving_node: 'tbs-lab-01', last_rssi_dbfs: -32.4 }], failures: [] },
  },
  'iot-gateway': {
    '/api/v1/status': { mqtt_connected: true, sources_healthy: 0, sources_enabled: 0, outbox_pending: 0,
      commands_received: 1, commands_executed: 1, home_assistant_enabled: true, home_assistant_discovery_runs: 1,
      home_assistant_external_entities: 1, homematic_enabled: false, homematic_mode: 'disabled',
      homematic_datapoints_healthy: 0, homematic_datapoints_configured: 0 },
    '/api/v1/sources': [], '/api/v1/topics': {}, '/api/v1/outbox': [], '/api/v1/commands': [], '/api/v1/events': [],
    '/api/v1/policies': [{ enabled: true, effect: 'allow', id: 'allow-openlab-virtual-lights',
      command_types: ['virtual.light.set'], target_types: ['virtual_light'], target_prefixes: ['lab-'] }],
    '/api/v1/virtual-devices': [{ device_type: 'virtual_light', id: 'lab-light-01', state: { on: true, brightness: 42 },
      updated_at: now, command_id: 'test-command' }], '/api/v1/home-assistant': {},
    '/api/v1/home-assistant/entities': [{ entity_id: 'input_boolean.netcore_lab_test', state: 'on',
      observed_at: now, received_at: now, attributes: {} }], '/api/v1/homematic/datapoints': [],
  },
};

const pages = {};
for (const [service, label] of Object.entries(labels)) {
  const input = await readFile(path.join(root, 'system-backend', service, 'web-ui/index.html'), 'utf8');
  const result = spawnSync(process.env.PYTHON || 'python3', ['-c',
    'import sys;sys.path.insert(0,"tools");from embed_service_design import render;print(render(sys.stdin.read(),sys.argv[1],"open-lab"))', label],
    { cwd: root, input, encoding: 'utf8', maxBuffer: 5 * 1024 * 1024 });
  assert.equal(result.status, 0, result.stderr);
  pages[service] = result.stdout;
}
const writes = [];
const unhandled = [];
const wav = Buffer.alloc(44 + 16000);
wav.write('RIFF'); wav.writeUInt32LE(wav.length - 8, 4); wav.write('WAVEfmt ', 8); wav.writeUInt32LE(16, 16);
wav.writeUInt16LE(1, 20); wav.writeUInt16LE(1, 22); wav.writeUInt32LE(8000, 24); wav.writeUInt32LE(16000, 28);
wav.writeUInt16LE(2, 32); wav.writeUInt16LE(16, 34); wav.write('data', 36); wav.writeUInt32LE(16000, 40);
const server = http.createServer(async (req, res) => {
  const url = new URL(req.url, 'http://127.0.0.1');
  const service = url.pathname.split('/')[1] in labels ? url.pathname.split('/')[1]
    : new URL(req.headers.referer || 'http://127.0.0.1/').pathname.split('/')[1];
  if (pages[service] && url.pathname === '/' + service + '/') {
    res.setHeader('Content-Type', 'text/html; charset=utf-8');
    if (['security-core', 'kmf', 'transit', 'observability'].includes(service)) {
      res.setHeader('Content-Security-Policy', "default-src 'self'; img-src 'self' data:; script-src 'unsafe-inline'; style-src 'unsafe-inline'; connect-src 'self'");
    }
    res.end(pages[service]); return;
  }
  if (url.pathname.endsWith('/preview')) { res.setHeader('Content-Type', 'audio/wav'); res.end(wav); return; }
  if (url.pathname === '/favicon.ico') { res.writeHead(204).end(); return; }
  res.setHeader('Content-Type', 'application/json');
  if (req.method !== 'GET') {
    let body = ''; for await (const part of req) body += part;
    writes.push({ service, path: url.pathname, method: req.method, body: body ? JSON.parse(body) : null });
    if (url.pathname === '/api/v1/tts/generate') res.end(JSON.stringify({ asset }));
    else if (url.pathname === '/api/v1/test/command') res.end(JSON.stringify({ status: 'succeeded', message: 'Virtual fixture command' }));
    else res.end(JSON.stringify({ accepted: true }));
    return;
  }
  if (!(url.pathname in (fixtures[service] || {}))) {
    unhandled.push(service + ' ' + url.pathname); res.writeHead(404).end(JSON.stringify({ error: 'Missing test API fixture' })); return;
  }
  res.end(JSON.stringify(fixtures[service][url.pathname]));
});
await new Promise(resolve => server.listen(0, '127.0.0.1', resolve));
const origin = 'http://127.0.0.1:' + server.address().port;
await mkdir(output, { recursive: true });
let browser;
let checks = 0;

// Read the computed foreground against the nearest painted surface. Ignore
// deliberately disabled controls; they remain visually distinct in both modes.
async function assertDarkContrast(page, label) {
  const failures = await page.evaluate(() => {
    const rgb = value => {
      const parts = value.match(/[\d.]+/g)?.map(Number) || [];
      return parts.length >= 3 ? [...parts.slice(0, 3), parts[3] ?? 1] : [0, 0, 0, 0];
    };
    const lum = color => color.slice(0, 3).map(v => v / 255).map(v => v <= 0.04045 ? v / 12.92 : ((v + 0.055) / 1.055) ** 2.4)
      .reduce((sum, v, i) => sum + v * [0.2126, 0.7152, 0.0722][i], 0);
    const effectiveBackground = node => {
      let result = [0, 0, 0, 0];
      for (let p = node; p; p = p.parentElement) {
        const next = rgb(getComputedStyle(p).backgroundColor);
        const alpha = result[3] + next[3] * (1 - result[3]);
        if (alpha) result = [0, 1, 2].map(i => (result[i] * result[3] + next[i] * next[3] * (1 - result[3])) / alpha).concat(alpha);
        if (result[3] >= 0.99) break;
      }
      return result[3] ? result : [16, 26, 45, 1];
    };
    const selector = '.service-summary, .card, .panel, th, td, .notice, pre, code, .pill, label, input:not([type="hidden"]):not(:disabled), select:not(:disabled), textarea:not(:disabled), button:not(:disabled), .matrix-group span, .matrix-device span, .detail-list dt, .detail-list dd';
    return [...document.querySelectorAll(selector)].filter(node => {
      const box = node.getBoundingClientRect();
      return box.width > 0 && box.height > 0 && (node.textContent.trim() || /^(INPUT|SELECT|TEXTAREA)$/.test(node.tagName))
        && !node.closest('[hidden], button:disabled, input:disabled, select:disabled') && getComputedStyle(node).opacity === '1';
    }).map(node => {
      const foreground = rgb(getComputedStyle(node).color), background = effectiveBackground(node);
      const ratio = (Math.max(lum(foreground), lum(background)) + 0.05) / (Math.min(lum(foreground), lum(background)) + 0.05);
      return { selector: node.id || node.className || node.tagName, text: node.textContent.trim().slice(0, 45), ratio: Number(ratio.toFixed(2)) };
    }).filter(item => item.ratio < 4.5);
  });
  assert.deepEqual(failures, [], label + ' dark contrast below 4.5:1'); checks++;
}

try {
  browser = await playwright.chromium.launch({ headless: true,
    executablePath: process.env.CHROMIUM_EXECUTABLE_PATH || undefined, args: ['--no-sandbox'] });
  for (const [service, label] of Object.entries(labels)) {
    const context = await browser.newContext({ viewport: { width: 1600, height: 1000 } });
    const page = await context.newPage();
    const errors = [];
    page.on('pageerror', error => errors.push(error.message));
    page.on('console', msg => { if (msg.type() === 'error') errors.push(msg.text()); });
    page.on('dialog', dialog => dialog.accept());
    await page.goto(origin + '/' + service + '/', { waitUntil: 'networkidle' });
    assert.equal(await page.locator('.nc-service-name').textContent(), label); checks++;
    assert.equal(await page.locator('.nc-service-access').textContent(), 'OPEN LAB'); checks++;
    assert.equal(await page.locator('.nc-service-logo img').first().evaluate(img => img.complete && img.naturalWidth > 0), true); checks++;
    const tabs = page.locator('.nc-service-nav button[data-page], .nc-service-nav button[data-tab]');
    const fallbackTabs = page.locator('.nc-service-nav button');
    const navButtons = await tabs.count() ? tabs : fallbackTabs;
    const count = await navButtons.count();
    for (let i = 0; i < count; i++) {
      await navButtons.nth(i).click();
      assert.equal(await page.locator('.page.active, .tab.active').count(), 1, service + ' tab ' + i); checks++;
    }
    if (count) await navButtons.first().click();
    if (service === 'security-core') {
      await page.locator('.nc-service-nav button').filter({ hasText: /^Security-Profile$|^Profile$/ }).click();
      await page.locator('#profileRows button').filter({ hasText: 'Bearbeiten' }).click();
      assert.equal(await page.locator('#pName').inputValue(), profile.display_name); checks++;
      assert.deepEqual(await page.locator('#pPref option').allTextContents(), ['1', '2', '3']); checks++;
      assert.match(await page.locator('#profileAuthRows').innerText(), /tbs-lab-01/); checks++;
      await page.locator('#pName').fill('UI Test Geändert');
      await page.locator('#profiles button').filter({ hasText: 'Profil speichern' }).click();
      await page.waitForFunction(() => document.getElementById('profileRows').textContent.includes('UI Test Teilnehmer'));
      assert.equal(writes.findLast(w => w.service === service && w.path === '/api/v1/profiles').body.preferred_security_class, 3); checks++;
    }
    if (service === 'kmf') {
      await page.locator('.nc-service-nav button').filter({ hasText: /^Schlüssel$/ }).click();
      await page.locator('#keyRows button').filter({ hasText: key.label }).click();
      assert.match(await page.locator('#keyDetail').innerText(), /32 Bytes/); checks++;
      assert.match(await page.locator('#keyDetail').innerText(), /Crypto Period Ende/); checks++;
      await page.locator('#kBytes').fill('32'); await page.locator('#keys button').filter({ hasText: 'Schlüssel generieren' }).click();
      assert.equal(writes.findLast(w => w.service === service && w.path === '/api/v1/keys').body.key_bytes, 32); checks++;
    }
    if (service === 'transit') {
      assert.match(await page.locator('#overviewPeers').innerText(), /Region B Test/); checks++;
      await page.locator('.nc-service-nav button').filter({ hasText: /^Routing$/ }).click();
      await page.locator('#routeRows button').filter({ hasText: 'Deaktivieren' }).click();
      await page.waitForTimeout(50);
      assert.equal(writes.findLast(w => w.service === service).path, '/api/v1/routes/route-test/disable'); checks++;
    }
    if (service === 'application-gateway') {
      await page.locator('.nc-service-nav button').filter({ hasText: /^Connectoren$/ }).click();
      assert.match(await page.locator('#connectorRows').innerText(), /8150\/api\/v1\/messages/); checks++;
      await page.locator('#connectorRows button').filter({ hasText: /^Edit$/ }).click();
      assert.equal(await page.locator('#cEndpoint').inputValue(), connector.endpoint); checks++;
      await page.locator('#connectorDialog').evaluate(dialog => dialog.close());
    }
    if (service === 'media-library') {
      await page.locator('.nc-service-nav button').filter({ hasText: /^Bibliothek$/ }).click();
      await page.locator('#assetRows button').filter({ hasText: asset.title }).click();
      assert.equal(await page.locator('#libraryDetailTitle').textContent(), asset.title); checks++;
      assert.equal(await page.locator('#libraryPlayer').getAttribute('src'), '/api/v1/assets/' + assetId + '/preview'); checks++;
      await page.locator('#libraryActions button').filter({ hasText: 'Zur Aussendung' }).click();
      assert.equal(await page.locator('#dAsset').inputValue(), assetId); checks++;
      assert.equal(await page.locator('#dSession').isVisible(), false); checks++;
      await page.locator('#dDestination').fill('2000');
      await page.locator('#dSubmit').click(); await page.waitForTimeout(50);
      const dispatch = writes.findLast(w => w.path === '/api/v1/dispatch');
      assert.equal(dispatch.body.station_id, 'tbs-lab-01'); assert.equal(dispatch.body.session_id, null); checks++;
      await page.locator('.nc-service-nav button').filter({ hasText: /^TTS \/ Piper$/ }).click();
      await page.locator('#ttsText').fill('Dies ist ein UI Test.'); await page.locator('#ttsName').fill('UI Test');
      assert.equal(await page.locator('#ttsText').getAttribute('maxlength'), '2000'); checks++;
      assert.equal(await page.locator('#ttsSpeedLabel').textContent(), '95 %'); checks++;
      await page.locator('#ttsGenerate').click();
      await page.waitForFunction(() => document.getElementById('ttsOutput').textContent.includes('TTS gespeichert'));
      const generated = writes.findLast(w => w.path === '/api/v1/tts/generate');
      assert.equal(generated.body.speed, 0.95); assert.equal(generated.body.text, 'Dies ist ein UI Test.'); checks++;
      await page.evaluate(() => scrollTo(0, 0));
      await page.screenshot({ path: path.join(output, 'media-library-tts.png'), fullPage: true });
      await page.locator('.nc-service-nav button').filter({ hasText: /^Bibliothek$/ }).click();
    }
    if (service === 'recorder') {
      const held = page.locator('#recordingRows tr').filter({ hasText: 'recording-held' });
      assert.equal(await held.getByRole('button', { name: 'Löschen', exact: true }).isDisabled(), true); checks++;
      assert.match(await held.innerText(), /Legal Hold aktiv/); checks++;
      assert.equal(await page.locator('#recordingRows tr').filter({ hasText: 'recording-free' }).getByRole('button', { name: 'Löschen', exact: true }).isDisabled(), false); checks++;
      assert.equal(await page.locator('audio').count(), 0); checks++;
      await held.getByRole('button', { name: 'Prüfen', exact: true }).click(); await page.waitForTimeout(50);
      assert.equal(writes.findLast(w => w.service === service).path, '/api/v1/recordings/recording-held/verify'); checks++;
    }
    if (service === 'observability') {
      await page.locator('.nc-service-nav button').filter({ hasText: /^Targets$|^Scrape Targets$/ }).click();
      assert.match(await page.locator('#discoveryStatus').innerText(), /Discovery deaktiviert/); checks++;
      assert.match(await page.locator('#syslogStatus').innerText(), /Syslog-Empfänger noch nicht installiert/); checks++;
      await page.locator('#targetsTable button').filter({ hasText: 'Monitoring aus' }).click(); await page.waitForTimeout(50);
      assert.equal(writes.findLast(w => w.service === service).path, '/api/v1/targets/recorder-01/disable'); checks++;
      assert.equal(await page.locator('#overviewTargets').innerText().then(x => x.includes('up')), true); checks++;
    }
    if (service === 'provisioning-core') {
      assert.match(await page.locator('#deviceRows').innerText(), /-32\.4 dBFS/); checks++;
      await page.locator('.nc-service-nav button[data-tab="matrix"]').click();
      await page.locator('#matrixBody button').filter({ hasText: 'Details' }).click();
      assert.equal(await page.locator('#membershipForm input[name="auto_attach"]').isChecked(), true); checks++;
      await page.locator('#membershipForm textarea[name="notes"]').fill('UI Test Änderung');
      await page.locator('#membershipForm button[type="submit"]').click(); await page.waitForTimeout(50);
      assert.equal(writes.findLast(w => w.service === service).body.notes, 'UI Test Änderung'); checks++;
    }
    if (service === 'iot-gateway') {
      assert.match(await page.locator('#policies').innerText(), /virtual\.light\.set/); checks++;
      assert.match(await page.locator('#external').innerText(), /2026-10-03/); checks++;
      await page.getByRole('button', { name: 'Lab-Licht 42 %', exact: true }).click(); await page.waitForTimeout(50);
      const command = writes.findLast(w => w.service === service).body;
      assert.equal(command.command_type, 'virtual.light.set'); assert.equal(command.target.type, 'virtual_light'); checks++;
    }
    await page.evaluate(() => scrollTo(0, 0));
    assert.equal(Math.round((await page.locator('.nc-service-header').boundingBox()).y), 0); checks++;
    await page.screenshot({ path: path.join(output, service + '.png'), fullPage: true });
    for (const width of [768, 390]) {
      await page.setViewportSize({ width, height: 900 });
      assert.equal(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth + 1), true, service + ' overflow at ' + width); checks++;
    }

    await page.setViewportSize({ width: 1600, height: 1000 });
    await page.locator('.nc-theme-toggle').click();
    assert.equal(await page.evaluate(() => document.documentElement.dataset.ncTheme), 'dark'); checks++;
    assert.equal(await page.evaluate(() => localStorage.getItem('netcore-theme')), 'dark'); checks++;
    await page.reload({ waitUntil: 'networkidle' });
    assert.equal(await page.evaluate(() => document.documentElement.dataset.ncTheme), 'dark'); checks++;
    assert.equal(await page.evaluate(() => getComputedStyle(document.documentElement).colorScheme), 'dark'); checks++;
    assert.equal(await page.locator('.nc-theme-toggle').getAttribute('aria-pressed'), 'true'); checks++;
    await assertDarkContrast(page, service + ' overview');
    const darkTabs = page.locator('.nc-service-nav button[data-page], .nc-service-nav button[data-tab]');
    const darkNav = await darkTabs.count() ? darkTabs : page.locator('.nc-service-nav button');
    for (let i = 0; i < await darkNav.count(); i++) {
      await darkNav.nth(i).click();
      assert.equal(await page.locator('.page.active, .tab.active').count(), 1); checks++;
      await assertDarkContrast(page, service + ' dark tab ' + i);
    }
    if (service === 'security-core') {
      await page.locator('.nc-service-nav button').filter({ hasText: /^Security-Profile$|^Profile$/ }).click();
      await page.locator('#profileRows button').filter({ hasText: 'Bearbeiten' }).click();
      await assertDarkContrast(page, 'Security profile editor');
    } else if (service === 'kmf') {
      await page.locator('.nc-service-nav button').filter({ hasText: /^Schlüssel$/ }).click();
      await page.locator('#keyRows button').filter({ hasText: key.label }).click();
      await assertDarkContrast(page, 'KMF selected metadata');
    } else if (service === 'application-gateway') {
      await page.locator('.nc-service-nav button').filter({ hasText: /^Connectoren$/ }).click();
      await page.locator('#connectorRows button').filter({ hasText: /^Edit$/ }).click();
      await assertDarkContrast(page, 'Application connector dialog');
      await page.screenshot({ path: path.join(output, 'application-gateway-dialog-dark.png'), fullPage: true });
      await page.locator('#connectorDialog').evaluate(dialog => dialog.close());
    } else if (service === 'media-library') {
      await page.locator('.nc-service-nav button').filter({ hasText: /^TTS \/ Piper$/ }).click();
      await page.locator('#ttsText').fill('Dies ist ein UI Test im dunklen Design.');
      await assertDarkContrast(page, 'Media TTS editor');
      await page.evaluate(() => scrollTo(0, 0));
      await page.screenshot({ path: path.join(output, 'media-library-tts-dark.png'), fullPage: true });
      await page.locator('.nc-service-nav button').filter({ hasText: /^Bibliothek$/ }).click();
      await page.locator('#assetRows button').filter({ hasText: asset.title }).click();
      assert.equal(await page.locator('#libraryPlayer').evaluate(node => getComputedStyle(node).colorScheme), 'dark'); checks++;
      await assertDarkContrast(page, 'Media asset, preview and waveform');
    } else if (service === 'observability') {
      await page.locator('.nc-service-nav button').first().click();
    } else if (service === 'provisioning-core') {
      await page.locator('.nc-service-nav button[data-tab="devices"]').click();
      await page.locator('#deviceRows button').filter({ hasText: 'Bearbeiten' }).click();
      await assertDarkContrast(page, 'Provisioning device dialog');
      await page.screenshot({ path: path.join(output, 'provisioning-core-dialog-dark.png'), fullPage: true });
      await page.locator('#deviceDialog').evaluate(dialog => dialog.close());
      await page.locator('.nc-service-nav button[data-tab="matrix"]').click();
      await page.locator('#matrixBody button').filter({ hasText: 'Details' }).click();
      await assertDarkContrast(page, 'Provisioning membership editor and matrix');
    } else if (await darkNav.count()) {
      await darkNav.first().click();
    }
    await page.evaluate(() => scrollTo(0, 0));
    await page.screenshot({ path: path.join(output, service + '-dark.png'), fullPage: true });
    await page.setViewportSize({ width: 390, height: 900 });
    assert.equal(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth + 1), true, service + ' dark mobile overflow'); checks++;
    await assertDarkContrast(page, service + ' dark mobile');
    await page.screenshot({ path: path.join(output, service + '-dark-mobile.png'), fullPage: true });
    await page.locator('.nc-theme-toggle').click();
    assert.equal(await page.evaluate(() => document.documentElement.dataset.ncTheme), 'light'); checks++;
    assert.equal(await page.evaluate(() => localStorage.getItem('netcore-theme')), 'light'); checks++;
    await page.reload({ waitUntil: 'networkidle' });
    assert.equal(await page.evaluate(() => document.documentElement.dataset.ncTheme), 'light'); checks++;
    assert.deepEqual(errors, [], service + ' browser errors'); checks++;
    console.log(service + ': tabs, layout, actions and persisted light/dark themes passed');
    await context.close();
  }
  assert.deepEqual(unhandled, [], 'unhandled fixture API calls');
  console.log('Media/security service browser checks passed: ' + checks + ' checks across nine services.');
} finally {
  await browser?.close(); await new Promise(resolve => server.close(resolve));
}
