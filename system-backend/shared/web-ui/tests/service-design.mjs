#!/usr/bin/env node
// Meaningful shell checks: functional node preservation, mobile tables, authentic access labels.
import assert from 'node:assert/strict';
import path from 'node:path';
import { readFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { createRequire } from 'node:module';
import http from 'node:http';
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
const init = await readFile(path.join(assets, 'service-theme-init.js'), 'utf8');
const logo = (await readFile(path.join(assets, 'netcore-logo.data-uri'), 'utf8')).trim();
const original = await readFile(path.resolve(assets, '../../../../crates/tetra-entities/src/net_dashboard/ui/netcore-logo.png'));
assert.deepEqual(Buffer.from(logo.slice('data:image/png;base64,'.length), 'base64'), original);
const shell = (content, access = 'open-lab') => `<!doctype html><html data-netcore-ui="service"><head><script id="netcore-service-init">${init}</script><script>window.earlyTheme=document.documentElement.dataset.ncTheme;</script><style>:root{--bg:#000;--text:#fff}body{background:var(--bg);color:var(--text)}.layout{display:grid;grid-template-columns:240px 1fr}.page{display:none}.page.active{display:block}</style><style>${css}</style><script type="application/json" id="netcore-service-config">${JSON.stringify({name:'Subscriber Core',access,logo})}</script></head><body>${content}<script>${js}</script></body></html>`;
const surfaces = `<main><section class="panel" id="surfaces"><h2>Theme surfaces</h2><label>Input<input id="surface-input" value="Test"></label><select id="surface-select"><option>Test</option></select><textarea id="surface-textarea">Test</textarea><button class="primary" id="surface-primary">Speichern</button><button class="danger" id="surface-danger">Löschen</button><pre id="surface-pre">Diagnose</pre><code id="surface-code">node-1</code><table><thead><tr><th id="surface-th">ISSI</th></tr></thead><tbody><tr><td>100001</td></tr></tbody></table><span class="pill online" id="surface-online">ONLINE</span><span class="pill stale" id="surface-stale">STALE</span><span class="pill offline" id="surface-offline">OFFLINE</span><span class="pill draft" id="surface-draft">DRAFT</span><span class="pill disabled" id="surface-disabled">Monitoring aus</span><span class="nc-status" data-status="warning" id="surface-status">Warnung</span><div class="notice" id="surface-notice">Hinweis</div><div class="leaflet-popup-content-wrapper" id="surface-popup"><div class="leaflet-popup-content">Kartendetails</div></div><div class="leaflet-bar"><a id="surface-zoom">+</a></div><dialog id="surface-dialog"><h2>Bestätigen</h2><input value="Test"></dialog></section><section class="panel"><h2>Zweiter Bereich</h2></section></main>`;
const server = http.createServer((_request, response) => {
  response.writeHead(200, {'Content-Type':'text/html; charset=utf-8'});
  response.end(shell(surfaces));
});
await new Promise(resolve => server.listen(0, '127.0.0.1', resolve));
const base = `http://127.0.0.1:${server.address().port}`;
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

  const context = await browser.newContext({colorScheme:'dark'});
  const themePage = await context.newPage();
  await themePage.goto(base);
  assert.equal(await themePage.evaluate(() => window.earlyTheme), 'light'); checks++;
  assert.equal(await themePage.getAttribute('html', 'data-nc-theme'), 'light'); checks++;
  await themePage.evaluate(() => { window.themeEvents=[];window.addEventListener('netcore-theme-change',e=>window.themeEvents.push(e.detail.theme)); });
  const surfaceSelectors = ['#surfaces','#surface-input','#surface-select','#surface-textarea','#surface-primary','#surface-danger','#surface-pre','#surface-code','#surface-th','#surface-online','#surface-stale','#surface-offline','#surface-draft','#surface-disabled','#surface-status','#surface-notice','#surface-popup','#surface-zoom','#surface-dialog'];
  const checkContrasts = async () => {
    await themePage.evaluate(() => document.querySelector('#surface-dialog').showModal());
    const values = await themePage.evaluate(selectors => {
      const rgb = value => [...value.matchAll(/[\d.]+/g)].slice(0,3).map(m => Number(m[0]));
      const luminance = rgb => rgb.map(c=>c/255).map(c=>c<=.04045?c/12.92:((c+.055)/1.055)**2.4).reduce((a,c,i)=>a+c*[.2126,.7152,.0722][i],0);
      return selectors.map(selector => {
        const style=getComputedStyle(document.querySelector(selector)),fg=luminance(rgb(style.color)),bg=luminance(rgb(style.backgroundColor));
        return {selector,foreground:style.color,background:style.backgroundColor,contrast:(Math.max(fg,bg)+.05)/(Math.min(fg,bg)+.05)};
      });
    },surfaceSelectors);
    for(const value of values){assert.ok(value.contrast>=4.5,`${value.selector}: ${value.foreground} on ${value.background}, contrast ${value.contrast}`);checks++;}
    await themePage.evaluate(() => document.querySelector('#surface-dialog').close());
  };
  await checkContrasts();
  await themePage.click('.nc-theme-toggle');
  assert.equal(await themePage.evaluate(() => localStorage.getItem('netcore-theme')), 'dark'); checks++;
  assert.equal(await themePage.evaluate(() => localStorage.getItem('netcore-service-theme')), 'dark'); checks++;
  assert.deepEqual(await themePage.evaluate(() => window.themeEvents), ['dark']); checks++;
  assert.equal(await themePage.locator('#surface-input').evaluate(node=>getComputedStyle(node).backgroundColor),'rgb(19, 32, 55)');checks++;
  await themePage.waitForFunction(()=>getComputedStyle(document.querySelector('#surface-danger')).backgroundColor==='rgb(72, 35, 47)');
  await checkContrasts();
  await themePage.reload();
  assert.equal(await themePage.evaluate(() => window.earlyTheme), 'dark'); checks++;
  assert.equal(await themePage.getAttribute('html', 'data-nc-theme'), 'dark'); checks++;
  assert.equal(await themePage.locator('.nc-theme-toggle').getAttribute('aria-pressed'),'true');checks++;
  const otherTab = await context.newPage();await otherTab.goto(base);
  await themePage.click('.nc-theme-toggle');
  await otherTab.waitForFunction(()=>document.documentElement.dataset.ncTheme==='light');
  assert.equal(await otherTab.locator('.nc-theme-toggle').getAttribute('aria-pressed'),'false');checks++;
  await themePage.reload();
  assert.equal(await themePage.evaluate(() => window.earlyTheme), 'light');checks++;
  await themePage.evaluate(() => {localStorage.removeItem('netcore-theme');localStorage.setItem('netcore-service-theme','dark');});
  await themePage.reload();
  assert.equal(await themePage.evaluate(() => window.earlyTheme),'dark');checks++;
  await themePage.evaluate(() => localStorage.setItem('netcore-theme','blue'));
  await themePage.reload();
  assert.equal(await themePage.evaluate(() => window.earlyTheme),'light');checks++;
  assert.equal(await themePage.getAttribute('html','data-nc-theme'),'light');checks++;
  const blocked = await browser.newContext();
  await blocked.addInitScript(()=>Object.defineProperty(window,'localStorage',{get(){throw new DOMException('Storage blocked','SecurityError');}}));
  const blockedPage = await blocked.newPage();await blockedPage.goto(base);
  assert.equal(await blockedPage.evaluate(()=>window.earlyTheme),'light');checks++;
  await blockedPage.click('.nc-theme-toggle');
  assert.equal(await blockedPage.getAttribute('html','data-nc-theme'),'dark');checks++;
  await blockedPage.click('.nc-theme-toggle');
  assert.equal(await blockedPage.getAttribute('html','data-nc-theme'),'light');checks++;
  await blockedPage.reload();
  assert.equal(await blockedPage.evaluate(()=>window.earlyTheme),'light');checks++;
  await blocked.close();await context.close();
  console.log(`${checks} shared service shell checks passed`);
} finally { await browser.close();await new Promise(resolve=>server.close(resolve)); }
