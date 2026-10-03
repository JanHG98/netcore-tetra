#!/usr/bin/env node
// Exercise the deployed embedded HTML with isolated fixtures; no live service is contacted.
// Run: node tools/test_edge_service_ui.mjs (Playwright + Chromium required).
import assert from 'node:assert/strict';
import http from 'node:http';
import { mkdir } from 'node:fs/promises';
import { execFileSync } from 'node:child_process';
import { createRequire } from 'node:module';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const require = createRequire(import.meta.url);
let playwright;
try { playwright = require('playwright'); }
catch (error) {
  if (!process.env.CODEX_PRIMARY_RUNTIME_NODE_MODULES) throw error;
  playwright = require(path.join(process.env.CODEX_PRIMARY_RUNTIME_NODE_MODULES, 'playwright'));
}
const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const output = path.join(root, 'target/service-ui-preview/edge');
await mkdir(output, { recursive: true });
const extractHtml = String.raw`import ast, pathlib, sys
tree = ast.parse(pathlib.Path(sys.argv[1]).read_text())
for node in tree.body:
    if isinstance(node, ast.Assign) and any(isinstance(t, ast.Name) and t.id == 'HTML' for t in node.targets):
        sys.stdout.write(ast.literal_eval(node.value))
        break
else:
    raise RuntimeError('Embedded HTML constant missing')`;
const html = Object.fromEntries(['hardware-gateway', 'rf-monitor'].map(service => [service,
  execFileSync(process.env.PYTHON || 'python3', ['-c', extractHtml,
    path.join(root, 'system-backend', service, 'src', `netcore_${service.replaceAll('-', '_')}.py`)],
  { encoding: 'utf8', maxBuffer: 4 * 1024 * 1024 })]));
const at = new Date().toISOString();
let service = 'hardware-gateway';
let failed = false;
let empty = false;
const requests = [];
const hardwareStatus = { phase: 6, mqtt_host: '127.0.0.1', mqtt_port: 1883,
  devices_online: 2, devices_total: 3, outputs_enabled: false };
const devices = [
  { device_id: 'rack', name: 'Mobiles Rack', online: true, last_seen: at,
    metrics: { temperature_c: 32.5, humidity_percent: 45, supply_voltage_v: 12.3 },
    alarms: [], inputs: { door_open: false }, outputs: {} },
  { device_id: 'warm', name: 'Standort Süd', online: true, last_seen: at,
    metrics: { temperature_c: 42.1 }, alarms: [
      { metric: 'temperature_c', value: 42.1, severity: 'warning', reason: 'warning_above' }] },
  { device_id: 'offline', name: '<img src=x onerror="window.injected=true">', online: false,
    last_seen: at, metrics: { temperature_c: null }, alarms: [] },
];
const alarm = { station_id: 'sued', alarm_key: 'pa_temperature_high', severity: 'warning',
  metric: 'pa_temperature_c', value: 76, limit: 70, unit: '°C', reason: 'high' };
const stations = [
  { station_id: 'nord', name: 'TBS Nord', last_seen: at, captured_at: at, online: true,
    health: 'ok', tx_active: true, metrics: { tx_rms_dbfs: -18.4, tx_peak_dbfs: -8.7,
      evm_pct: 2.1, papr_db: 4.7, sdr_temperature_c: 43, forward_power_w: 5,
      reflected_power_w: .12, vswr: 1.37, pa_temperature_c: 52 },
    spectrum: { sample_rate: 100000, center_freq_hz: 418000000,
      bins_db: Array.from({ length: 128 }, (_, i) => i === 15 ? null : -90 + 70 * Math.exp(-(((i - 64) / 10) ** 2))) },
    alarms: {}, inputs: {}, metadata: {} },
  { station_id: 'ost', name: 'TBS Ost', last_seen: at, captured_at: at, online: true,
    health: 'ok', tx_active: false, metrics: { sdr_temperature_c: 42 },
    spectrum: { bins_db: [null, null] }, alarms: {}, metadata: {}, inputs: {} },
  { station_id: 'sued', name: 'TBS Süd', last_seen: at, captured_at: at, online: true,
    health: 'warning', tx_active: true, metrics: { forward_power_w: 4.8,
      reflected_power_w: .25, vswr: 1.59, sdr_temperature_c: 46, pa_temperature_c: 76 },
    spectrum: null, alarms: { pa_temperature_high: alarm }, inputs: {}, metadata: {} },
];
const rfStatus = { stations_online: 3, stations_total: 3, tx_active: 2, active_alarms: 1,
  health_counts: { ok: 2, warning: 1, critical: 0, offline: 0 } };
const server = http.createServer((request, response) => {
  requests.push({ method: request.method, path: request.url });
  if (request.url === '/') {
    response.setHeader('Content-Type', 'text/html; charset=utf-8');
    response.end(html[service]);
    return;
  }
  if (failed) { response.writeHead(503); response.end('{}'); return; }
  const api = { '/api/v1/devices': empty ? [] : devices,
    '/api/v1/stations': empty ? [] : stations, '/api/v1/alarms': empty ? [] : [alarm],
    '/api/v1/status': service === 'hardware-gateway' ? hardwareStatus : rfStatus };
  if (!(request.url in api)) { response.writeHead(404); response.end('{}'); return; }
  response.setHeader('Content-Type', 'application/json');
  response.end(JSON.stringify(api[request.url]));
});
await new Promise(resolve => server.listen(0, '127.0.0.1', resolve));
let browser;
try {
  browser = await playwright.chromium.launch({
    executablePath: process.env.CHROMIUM_EXECUTABLE_PATH || undefined,
    headless: true, args: ['--no-sandbox'],
  });
  const page = await browser.newPage({ viewport: { width: 1680, height: 1000 } });
  const errors = [];
  page.on('pageerror', error => errors.push(error.message));
  const url = `http://127.0.0.1:${server.address().port}`;
  const screenshot = async (name, mobile = false) => {
    await page.setViewportSize(mobile ? { width: 390, height: 844 } : { width: 1680, height: 1000 });
    await page.evaluate(() => window.scrollTo(0, 0));
    assert.equal(await page.evaluate(() => document.documentElement.scrollWidth <= window.innerWidth), true,
      `${name}: content must remain inside viewport`);
    await page.screenshot({ path: path.join(output, `${name}-${mobile ? 'mobile' : 'desktop'}.png`), fullPage: true });
  };
  await page.goto(url);
  await page.waitForSelector('#d tr:nth-child(3)');
  assert.equal(await page.locator('.nc-service-nav a').count(), 2);
  assert.equal(await page.locator('#d img').count(), 0);
  assert.match(await page.locator('#d tr:nth-child(2)').innerText(), /Online/);
  assert.match(await page.locator('#d tr:nth-child(2)').innerText(), /42,1 °C/);
  assert.match(await page.locator('#d tr:nth-child(3) .hw-metric dd').innerText(), /^—$/);
  assert.match(await page.locator('#s').innerText(), /deaktiviert/);
  assert.equal(await page.evaluate(() => window.injected), undefined);
  assert.equal(await page.locator('.nc-service-access').innerText(), 'OPEN LAB');
  await screenshot('hardware-gateway');
  await screenshot('hardware-gateway', true);
  failed = true;
  await page.evaluate(() => r());
  assert.match(await page.locator('#refresh-state').innerText(), /Daten können veraltet sein/);
  assert.equal(await page.locator('#d tr').count(), 3, 'A failed refresh retains previous device data');
  failed = false;
  empty = true;
  await page.evaluate(() => r());
  assert.match(await page.locator('#d').innerText(), /Noch keine Geräte registriert/);
  empty = false;

  service = 'rf-monitor';
  await page.setViewportSize({ width: 1680, height: 1000 });
  await page.goto(url);
  await page.waitForSelector('#stations tr:nth-child(3)');
  assert.equal(await page.locator('.nc-service-nav a').count(), 4);
  assert.match(await page.locator('#telemetry').innerText(), /-18,4 dBFS/);
  assert.match(await page.locator('#telemetry').innerText(), /5,00 W/);
  assert.match(await page.locator('#spectrum-meta').innerText(), /418,0000 MHz/);
  assert.match(await page.locator('#alarms').innerText(), /pa_temperature_high/);
  assert.match(await page.locator('#alarms').innerText(), /76,0 °C/);
  assert.match(await page.locator('#stations tr:nth-child(2)').innerText(), /Keine externe HF-Messwerte/);
  await screenshot('rf-monitor');
  await screenshot('rf-monitor', true);
  await page.locator('button[data-station="ost"]').click();
  assert.match(await page.locator('#telemetry').innerText(), /Keine Messwerte einer externen HF-Probe verfügbar/);
  assert.equal(await page.locator('#stations tr.selected-row').getAttribute('data-station-id'), 'ost');
  await page.locator('button[data-station="sued"]').click();
  assert.match(await page.locator('#telemetry').innerText(), /76,0 °C/);
  assert.match(await page.locator('#telemetry').innerText(), /1,59/);
  failed = true;
  await page.evaluate(() => refresh());
  assert.match(await page.locator('#refresh-state').innerText(), /Daten können veraltet sein/);
  assert.equal(await page.locator('#stations tr').count(), 3, 'A failed refresh retains previous RF data');
  failed = false;
  empty = true;
  await page.evaluate(() => refresh());
  assert.match(await page.locator('#telemetry').innerText(), /Noch keine Station/);
  assert.match(await page.locator('#stations').innerText(), /Noch keine Basisstationen/);
  assert.match(await page.locator('#alarms').innerText(), /Keine aktiven Alarme/);
  assert.deepEqual(errors, [], 'No browser script errors');
  assert(requests.every(request => request.method === 'GET'), 'Monitoring UI must not issue write requests');
  console.log('PASS Hardware/RF: live-field fixtures, escaped names, null metrics, probe/bin fallbacks, selection, alarms, empty states, stale-data errors, desktop/mobile');
  console.log(`Screenshots: ${path.relative(root, output)}`);
} finally {
  if (browser) await browser.close();
  await new Promise(resolve => server.close(resolve));
}
