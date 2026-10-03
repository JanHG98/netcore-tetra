#!/usr/bin/env node
// Meaningful shell checks: functional node preservation, mobile tables, authentic access labels.
import assert from 'node:assert/strict';
import path from 'node:path';
import { readFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { createRequire } from 'node:module';
const require = createRequire(import.meta.url);
let playwright;
try { playwright = require('playwright'); }
catch (error) {
  if (!process.env.CODEX_PRIMARY_RUNTIME_NODE_MODULES) throw error;
  playwright = require(path.join(process.env.CODEX_PRIMARY_RUNTIME_NODE_MODULES, 'playwright'));
}
const assets = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../assets');
const css = await readFile(path.join(assets, 'service-design.css'), 'utf8');
const js = await readFile(path.join(assets, 'service-design.js'), 'utf8');
const logo = (await readFile(path.join(assets, 'netcore-logo.data-uri'), 'utf8')).trim();
const original = await readFile(path.resolve(assets, '../../../../crates/tetra-entities/src/net_dashboard/ui/netcore-logo.png'));
assert.deepEqual(Buffer.from(logo.slice('data:image/png;base64,'.length), 'base64'), original);
const shell = (content, access = 'open-lab') => `<!doctype html><html data-netcore-ui="service"><head><style>:root{--bg:#000;--text:#fff}body{background:var(--bg);color:var(--text)}.layout{display:grid;grid-template-columns:240px 1fr}.page{display:none}.page.active{display:block}</style><style>${css}</style><script type="application/json" id="netcore-service-config">${JSON.stringify({name:'Subscriber Core',access,logo})}</script></head><body>${content}<script>${js}</script></body></html>`;
const browser = await playwright.chromium.launch({ headless: true, executablePath: process.env.CHROMIUM_EXECUTABLE_PATH });
let checks = 0;
try {
  const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
  await page.setContent(shell(`<div class="layout"><aside><h1>Subscriber Core</h1><div id="gateway">Gateway verbunden</div><nav id="nav"><button id="overviewButton" class="active" data-page="overview">Übersicht</button><button id="profileButton" data-page="profiles">Teilnehmer</button></nav></aside><main><section id="overview" class="page active"><div class="cards"><div class="card"><div class="value">12</div><span class="muted">Profile</span></div></div></section><section id="profiles" class="page"><div class="panel"><h2>Teilnehmer</h2><table><thead><tr><th>ISSI</th><th>Name</th><th>Aktion</th></tr></thead><tbody><tr><td>100001</td><td>Testgerät</td><td><button id="save" onclick="window.saved=(window.saved||0)+1">Bearbeiten</button></td></tr></tbody></table></div></section></main></div><script>window.navNode=document.getElementById('nav');window.saveNode=document.getElementById('save');document.querySelectorAll('nav button').forEach(button=>button.addEventListener('click',()=>{document.querySelectorAll('.page').forEach(p=>p.classList.toggle('active',p.id===button.dataset.page));}));</script>`));
  await page.waitForSelector('.nc-service-header');
  assert.equal(await page.locator('.nc-service-header').count(), 1); checks++;
  assert.equal(await page.evaluate(() => document.getElementById('nav') === window.navNode && document.getElementById('save') === window.saveNode), true); checks++;
  assert.equal(await page.locator('.nc-service-header #nav').count(), 1); checks++;
  assert.equal(await page.locator('#gateway').textContent(), 'Gateway verbunden'); checks++;
  await page.click('#profileButton');
  assert.equal(await page.locator('#profiles').isVisible(), true); checks++;
  await page.click('#save');
  assert.equal(await page.evaluate(() => window.saved), 1); checks++;
  assert.equal(await page.evaluate(() => getComputedStyle(document.body).backgroundColor), 'rgb(243, 246, 251)'); checks++;
  assert.equal(await page.locator('.nc-service-access').textContent(), 'OPEN LAB'); checks++;
  assert.equal(await page.locator('.nc-service-logo img').count(), 2); checks++;
  assert.equal(await page.locator('.nc-service-logo img').first().evaluate(img => img.naturalWidth), 1024); checks++;
  await page.click('.nc-theme-toggle');
  assert.equal(await page.getAttribute('html', 'data-nc-theme'), 'dark'); checks++;
  assert.equal(await page.locator('.nc-theme-toggle').getAttribute('aria-pressed'), 'true'); checks++;
  await page.click('.nc-theme-toggle');
  assert.equal(await page.getAttribute('html', 'data-nc-theme'), 'light'); checks++;
  await page.setViewportSize({width:390,height:844});
  assert.equal(await page.evaluate(() => document.documentElement.scrollWidth <= 391), true); checks++;
  await page.setContent(shell(`<main><section class="panel"><h2>Aktive Aufnahmen</h2><table style="min-width:900px"><thead><tr><th>Recording-ID</th></tr></thead><tbody><tr><td>sample-recording</td></tr></tbody></table></section><section class="panel"><h2>Aufbewahrung</h2><button id="hold" onclick="window.held=true">Hold</button></section></main>`, 'access-key'));
  await page.waitForSelector('.nc-service-header');
  assert.equal(await page.locator('.nc-service-nav a').count(), 2); checks++;
  assert.equal(await page.locator('.nc-service-access').textContent(), 'Zugangsschlüssel'); checks++;
  assert.equal(await page.locator('.nc-service-header').getByText('OPEN LAB').count(), 0); checks++;
  assert.equal(await page.locator('.nc-table-wrap').count(), 1); checks++;
  assert.equal(await page.evaluate(() => document.documentElement.scrollWidth <= 391), true); checks++;
  const ids = await page.locator('[id]').evaluateAll(nodes => nodes.map(n => n.id));
  assert.equal(new Set(ids).size, ids.length); checks++;
  await page.click('.nc-service-nav a:last-child');
  await page.click('#hold');
  assert.equal(await page.evaluate(() => window.held), true); checks++;
  // Re-running enhancement cannot create a second shell or duplicate IDs.
  await page.addScriptTag({content:js});
  assert.equal(await page.locator('.nc-service-header').count(), 1); checks++;
  console.log(`${checks} shared service shell checks passed`);
} finally { await browser.close(); }
