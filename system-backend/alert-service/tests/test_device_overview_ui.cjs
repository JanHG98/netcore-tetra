// Run with: node --test system-backend/alert-service/tests/test_device_overview_ui.cjs
// Exercise the real renderer without third-party browser dependencies.
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const vm = require('node:vm');
const {test} = require('node:test');

class Element {
  constructor(tag = 'div') { this.tagName = tag; this.children = []; this.attributes = {}; this.hidden = false; this.value = ''; }
  set textContent(value) { this.text = String(value); this.children = []; }
  get textContent() { return (this.text || '') + this.children.map(child => child.textContent).join(''); }
  append(...children) { this.children.push(...children); }
  replaceChildren(...children) { this.text = ''; this.children = children; }
  setAttribute(name, value) { this.attributes[name] = value; }
  addEventListener() {}
  scrollIntoView() { this.scrolled = true; }
  get style() { return this._style ||= {}; }
}

function ui() {
  const nodes = new Map();
  const get = id => { if(!nodes.has(id)) nodes.set(id, new Element()); return nodes.get(id); };
  get('alert-form').elements = Object.fromEntries(['latitude', 'longitude', 'radius_m', 'expires_at'].map(name => [name, new Element('input')]));
  get('composer').hidden = true;
  const map = {views:[], handlers:{}, setView(position, zoom) { this.views.push({position, zoom}); return this; }, on(name, handler) { this.handlers[name] = handler; return this; }, fitBounds() { return this; }};
  const markers = [];
  const layer = () => ({layers:[], handlers:{}, addTo(parent) { parent.layers?.push(this); return this; }, on(name, handler) { this.handlers[name] = handler; return this; }, clearLayers() { this.layers = []; }, getLayers() { return this.layers; }, getBounds() { return {isValid:() => true}; }, bindPopup(content) { this.popup = content; return this; }, openPopup() { this.popupOpened = true; return this; }});
  const leaflet = {map:() => map, tileLayer:layer, featureGroup:layer, layerGroup:layer, circle:layer, geoJSON:layer, circleMarker(position, options) { const marker = {...layer(), position, options}; markers.push(marker); return marker; }, DomEvent:{stopPropagation(event) { event.propagationStopped = true; }}};
  const context = vm.createContext({document:{getElementById:get, createElement:tag => new Element(tag)}, L:leaflet, sessionStorage:{getItem:() => ''}, console, fetch:() => new Promise(() => {}), setInterval:() => {}, Date, Map});
  const source = fs.readFileSync(path.join(__dirname, '../static/app.js'), 'utf8');
  vm.runInContext(source + '\nglobalThis.setSnapshot = value => { snapshot = value; render(); };', context);
  const set = extra => context.setSnapshot({alerts:[], devices:[], deliveries:[], errors:{}, last_cycle:1000, delivery_enabled:true, router_ready:true, ...extra});
  return {get, set, map, markers};
}

const ready = (issi, area_status, matching_alerts = []) => ({issi, node_id:'TBS-Test', available:true, gps_age_seconds:15, node_age_seconds:3, latitude:52, longitude:13, updated_at:1000, area_status, matching_alerts});
test('all devices remain visible; each matched warning keeps its own delivery result', () => {
  const view = ui();
  const maliciousTitle = '<img src=x onerror=alert(1)>';
  const accepted = {id:'one', title:maliciousTitle, eligible:false, delivery_state:'accepted'};
  const failed = {id:'two', title:'Zweite Warnung', eligible:true, delivery_state:'failed', delivery_error:'TBS nicht erreichbar'};
  const affected = ready(5102, 'affected', [accepted, failed]);
  view.set({delivery_enabled:false, router_ready:false, device_overview:[affected, ready(5103, 'outside'), {issi:5104, available:false, reason:'Keine GPS-Position', area_status:'unknown', matching_alerts:[]}]});
  const rows = view.get('device-check-list').children;
  assert.equal(rows.length, 3);
  assert.match(rows[0].textContent, /Von TBS angenommen/);
  assert.match(rows[0].textContent, /Keine Empfangsbestätigung/);
  assert.match(rows[0].textContent, /Fehlgeschlagen/);
  assert.match(rows[0].textContent, /Keine automatische Wiederholung/);
  assert.doesNotMatch(rows[0].textContent, /Versand pausiert|SDS-Router nicht bereit/);
  assert.equal(rows[0].children[3].children.length, 2);
  const title = rows[0].children[3].children[0].children[0];
  assert.equal(title.textContent, maliciousTitle);
  assert.equal(title.children.length, 0, 'untrusted warning titles remain text');
  assert.match(rows[1].textContent, /Außerhalb.*Kein Versand erforderlich/);
  assert.doesNotMatch(rows[1].textContent, /Fehlgeschlagen|Versand pausiert/);
  view.get('device-filter-affected').onclick();
  assert.equal(view.get('device-check-list').children.length, 1);
  assert.equal(view.get('device-filter-affected').attributes['aria-pressed'], 'true');
  view.get('device-filter-check').onclick();
  assert.equal(view.get('device-check-list').children.length, 2);
});

test('pause, router state and warning eligibility explain only unsent matching warnings', () => {
  const view = ui();
  const data = {device_overview:[ready(5102, 'affected', [{id:'one', title:'Testwarnung', eligible:true, delivery_state:null}])]};
  view.set({...data, delivery_enabled:false});
  assert.match(view.get('device-check-list').textContent, /Versand pausiert/);
  view.set({...data, router_ready:false});
  assert.match(view.get('device-check-list').textContent, /SDS-Router nicht bereit/);
  view.set(data);
  assert.match(view.get('device-check-list').textContent, /Versand ausstehend/);
  data.device_overview[0].matching_alerts[0].eligible = false;
  data.device_overview[0].matching_alerts[0].eligibility_reason = 'NINA-Daten sind zu alt';
  view.set({...data, delivery_enabled:false, router_ready:false});
  assert.match(view.get('device-check-list').textContent, /Versand ausgesetzt.*NINA-Daten sind zu alt/);
  assert.doesNotMatch(view.get('device-check-list').textContent, /Versand pausiert|SDS-Router nicht bereit/);
});

test('valid map actions focus the device without opening the warning editor', () => {
  const view = ui();
  view.set({device_overview:[ready(5102, 'outside'), {issi:5103, available:false, latitude:null, longitude:13, area_status:'unknown', matching_alerts:[]}]});
  const rows = view.get('device-check-list').children;
  rows[0].children[4].children[0].onclick();
  assert.equal(view.map.views.at(-1).zoom, 15);
  assert.equal(view.get('composer').hidden, true);
  assert.equal(view.markers.at(-1).popupOpened, true);
  assert.equal(rows[1].children[4].children[0].textContent, '–');
  assert.equal(view.markers.length, 1);
  assert.equal(view.markers[0].options.bubblingMouseEvents, false);
  const event = {};
  view.markers[0].handlers.click({originalEvent:event});
  assert.equal(event.propagationStopped, true);
});

test('older API combines ready and rejected devices without hiding or duplicating them', () => {
  const view = ui();
  view.set({devices:[{issi:5102, node_id:'TBS-Test', latitude:52, longitude:13, age_seconds:20}], device_diagnostics:[{issi:5102, reason:'Ältere Position'}, {issi:5103, reason:'GPS fehlt'}]});
  assert.equal(view.get('device-check-list').children.length, 2);
  assert.match(view.get('device-check-state').textContent, /bisherige Geräteansicht/);
  assert.doesNotMatch(view.get('device-check-list').textContent, /Ältere Position/);
  view.get('device-check-list').children[0].children[4].children[0].onclick();
  assert.equal(view.markers.at(-1).popupOpened, true, 'legacy map entries use stable identity');
});

test('waiting, failed retrieval, no devices and empty filters have distinct explanations', () => {
  const view = ui();
  view.set({device_overview:[], last_cycle:null});
  assert.match(view.get('device-check-state').textContent, /erste Geräteabgleich/);
  view.set({device_overview:[], errors:{control_room:'Verbindung abgelehnt'}});
  assert.match(view.get('device-check-list').textContent, /konnten nicht geladen werden/);
  view.set({device_overview:[]});
  assert.match(view.get('device-check-list').textContent, /keine angemeldeten Geräte/);
  view.set({device_overview:[ready(5102, 'no_alerts')]});
  assert.match(view.get('device-check-list').textContent, /Keine aktiven Warnungen/);
  view.get('device-filter-check').onclick();
  assert.match(view.get('device-check-list').textContent, /keine Prüfung nötig/);
  assert.equal(view.get('device-check-state').hidden, true);
  view.set({device_overview:[{issi:5103, available:false, area_status:'unknown', matching_alerts:[]}]});
  view.get('device-filter-affected').onclick();
  assert.match(view.get('device-check-list').textContent, /Keine Geräte mit geprüftem Standort im Warngebiet/);
  assert.doesNotMatch(view.get('device-check-list').textContent, /Kein Gerät befindet sich/);
});
