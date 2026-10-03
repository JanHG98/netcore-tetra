#!/usr/bin/env node
/** Run: RUSTC=rustc node tools/test_workflow_service_ui.mjs
 * Browser checks exercise actual page scripts with API-shaped fixtures.
 * No radio, SDS router, warning provider or deployed service is contacted.
 */
import assert from 'node:assert/strict';
import { readFileSync, writeFileSync, mkdtempSync, rmSync, mkdirSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { resolve, join, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';
import { execFileSync } from 'node:child_process';
import { createRequire } from 'node:module';
import http from 'node:http';
import { test, before, after } from 'node:test';
const require=createRequire(import.meta.url);
let playwright;
try { playwright=require('playwright'); } catch { playwright=require(join(process.env.CODEX_PRIMARY_RUNTIME_NODE_MODULES,'playwright')); }
const root=resolve(dirname(fileURLToPath(import.meta.url)),'..');
const content=p=>readFileSync(join(root,p),'utf8');
const tmp=mkdtempSync(join(tmpdir(),'netcore-workflow-ui-'));
let browser,server,origin;
let screenshotIndex=0;
const now=new Date().toISOString();
const future=new Date(Date.now()+42000).toISOString();
const alarm={alarm_id:'alarm-42',token:'AL-0042',title:'Uplink prüfen',description:'Verbindung gestört',state:'open',severity:'critical',priority:10,occurrences:3,source:{service:'node-gateway'},subject:{id:'TBS-03'},next_escalation_at:future,escalation_level:0,history:[{action:'created',actor:'test',timestamp:now,detail:{note:'Link gestört'}}]};
const taskData=(id,state,extra={})=>({task_id:'task-'+id,token:'TK-00'+id,title:'Auftrag '+id,task_type:'check',description:'Funkgerät prüfen',priority:5,state,owner:'test',assigned_issi:1001,created_by:'test',updated_at:now,timeline:[{action:'created',actor:'test',timestamp:now}],notifications:[],...extra});
const tasks=[taskData(81,'open',{notifications:[{status:'queued',destination:1001,is_group:false,reason:'created'}]}),taskData(82,'accepted'),taskData(83,'in_progress'),taskData(84,'blocked',{priority:12}),taskData(85,'completed'),taskData(86,'cancelled'),taskData(87,'expired')];
const templates=[{id:'check',name:'Fahrzeugcheck',fields:[{id:'vehicle',label:'Fahrzeug',required:true}]}];
const ownWarning={id:'manual:one',source:'manual',provider:'NetCore',title:'Zufahrt freihalten',description:'Bitte die Zufahrt für Einsatzfahrzeuge freihalten.',active:true,eligible:true,removed:false,sent_at:now,starts_at:now,expires_at:new Date(Date.now()+7200000).toISOString(),severity:'Moderate',geometry:{type:'Circle',coordinates:[9.732,52.3759],radius_m:2000}};
const alertStatus={now,last_cycle:Date.now()/1000,delivery_enabled:true,router_ready:true,alerts:[ownWarning],devices:[{issi:1001,node_id:'TBS-01',latitude:52.3759,longitude:9.732,updated_at:Date.now()/1000}],deliveries:[{alert_id:'manual:one',issi:1001,state:'accepted',updated_at:now}],errors:{},device_overview:[{issi:1001,node_id:'TBS-01',available:true,latitude:52.3759,longitude:9.732,gps_age_seconds:5,area_status:'affected',matching_alerts:[{id:'manual:one',title:ownWarning.title,eligible:true,delivery_state:'accepted'}]},{issi:1002,node_id:'TBS-01',available:false,reason:'GPS veraltet',gps_age_seconds:1000,area_status:'unknown',matching_alerts:[]}]};
const csp="default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'; img-src 'self' data: https://tile.openstreetmap.org; connect-src 'self'; frame-ancestors 'none'; base-uri 'none'; form-action 'self'";
before(async()=>{
 const wrapper=join(tmp,'render.rs'),binary=join(tmp,'render');
 writeFileSync(wrapper,`#[path=${JSON.stringify(join(root,'bins/netcore-control-room/src/webui.rs'))}] mod webui; fn main(){print!("{}", webui::index_html("/tbs", "/operator", std::env::args().any(|a|a=="--auth")));}`);
 execFileSync(process.env.RUSTC||'rustc',['--edition=2024',wrapper,'-o',binary]);
 const pages={ '/control':execFileSync(binary,[],{encoding:'utf8'}),'/protected':execFileSync(binary,['--auth'],{encoding:'utf8'}),'/alarm':content('system-backend/alarm-workflow/web-ui/index.html'),'/task':content('system-backend/task-workflow/web-ui/index.html'),'/alert':content('system-backend/alert-service/static/index.html') };
 server=http.createServer((req,res)=>{
  const pathname=new URL(req.url,'http://localhost').pathname;
  if(pages[pathname]) {if(pathname==='/alert')res.setHeader('Content-Security-Policy',csp);res.setHeader('Content-Type','text/html');res.end(pages[pathname]);return;}
  const files={'/app.js':'app.js','/style.css':'style.css','/service-design.js':'service-design.js','/vendor/leaflet.js':'vendor/leaflet.js','/vendor/leaflet.css':'vendor/leaflet.css'};
  if(files[pathname]) {res.setHeader('Content-Type',pathname.endsWith('.js')?'text/javascript':'text/css');res.end(content('system-backend/alert-service/static/'+files[pathname]));return;}
  res.writeHead(404);res.end();
 });
 await new Promise(resolve=>server.listen(0,'127.0.0.1',resolve));origin='http://127.0.0.1:'+server.address().port;
 const browserExecutable=process.env.CHROMIUM_EXECUTABLE_PATH||process.env.NETCORE_BROWSER_EXECUTABLE;
 browser=await playwright.chromium.launch({headless:true,...(browserExecutable?{executablePath:browserExecutable}:{})});
});
after(async()=>{await browser?.close();await new Promise(resolve=>server?server.close(resolve):resolve());rmSync(tmp,{recursive:true,force:true});});
async function pageFor(service,options={}){
 const page=await browser.newPage({viewport:{width:1600,height:1000}}),errors=[],requests=[];
 page.on('pageerror',error=>errors.push(error.message));
 // Capture CSP failures while ignoring intentionally unavailable map tiles.
 page.on('console',msg=>{if(msg.type()==='error'&&/Content Security Policy|Refused to execute/.test(msg.text()))errors.push(msg.text());});
 await page.route('https://tile.openstreetmap.org/**',route=>route.abort());
 await page.route('**/api/**',async route=>{
  const req=route.request(),u=new URL(req.url()),path=u.pathname;
  if(req.method()!=='GET'){requests.push({path,method:req.method(),body:req.postDataJSON(),headers:req.headers()});}
  let data={};
  if(service==='control'||service==='protected'){
   if(path==='/api/v1/control-room/overview')data={operations:{services_healthy:1,services_total:2,last_poll_finished_at:now},legacy:{nodes_connected:1,subscribers_online:2,active_calls_total:1,nodes:[{node_id:'TBS-01',station_name:'Nord',site:'Campus',connected:true,subscribers_online:2,subscribers_total:3,active_calls_total:1}]},federated:{preferred_counts:{},domains:{}}};
   else if(path==='/api/v1/services')data={services:[{name:'node-gateway',display_name:'Node Gateway',kind:'core',status:'healthy',critical:true,enabled:true,base_url:'http://node-gateway:8120',webui_url:'http://node-gateway:8120/',live:true,ready:true,latency_ms:4},{name:'sds-router',display_name:'SDS Router',kind:'core',status:'degraded',enabled:true,base_url:'http://sds-router:8150',webui_url:'http://sds-router:8150/',live:true,ready:false,latency_ms:20}]};
   else if(path==='/api/v1/incidents')data={incidents:[{id:'i-open',title:'Uplink',description:'Prüfen',status:'open',severity:'warning',created_at:now},{id:'i-done',title:'Behoben',status:'resolved',severity:'info',created_at:now}]};
   else if(path==='/api/v1/shift-log')data={entries:[]};else if(path==='/api/emergencies')data={emergencies:[]};else if(path==='/api/calls')data={calls:[]};else if(path.startsWith('/api/nodes/'))data={command_id:'command-1'};
  }else if(service==='alarm'){
   if(path==='/api/v1/status')data={mqtt_connected:true,sds_router_healthy:true,rules:8};
   else if(path==='/api/v1/alarms')data=req.method()==='GET'?[alarm,{...alarm,alarm_id:'alarm-resolved',token:'AL-0043',title:'Behoben',state:'resolved',next_escalation_at:null}]:{...alarm,token:'AL-0044'};
   else if(path==='/api/v1/events')data=[{event_id:'event-1',event_type:'alarm.created',timestamp:now,subject:{id:'TBS-03'},source:{service:'node-gateway'},severity:'critical'}];
   else if(path==='/api/v1/escalation-profiles')data=[{id:'critical',name:'Kritisch'}];
  }else if(service==='task'){
   if(path==='/api/v1/status')data={mqtt_connected:true,sds_router_healthy:true,templates:1};
   else if(path==='/api/v1/tasks')data=req.method()==='GET'?tasks:{...tasks[0],token:'TK-0099'};
   else if(path==='/api/v1/templates')data=templates;
  }else if(service==='alert'){
   if(!options.authorized&&req.headers().authorization!=='Bearer fixture-key') {await route.fulfill({status:401,contentType:'application/json',body:JSON.stringify({error:'unauthorized'})});return;}
   if(path==='/api/v1/status')data=alertStatus;
   else if(path==='/api/v1/alerts'&&req.method()==='POST')data={...ownWarning,id:'manual:new',title:req.postDataJSON().title};
  }
  await route.fulfill({status:200,contentType:'application/json',body:JSON.stringify(data)});
 });
 if(options.authorized)await page.addInitScript(()=>sessionStorage.setItem('netcore-alert-token','fixture-key'));
 await page.goto(origin+'/'+service);await page.waitForSelector('.nc-service-header');
 const capture=async name=>{if(process.env.NETCORE_WORKFLOW_SCREENSHOT_DIR){mkdirSync(process.env.NETCORE_WORKFLOW_SCREENSHOT_DIR,{recursive:true});await page.screenshot({path:join(process.env.NETCORE_WORKFLOW_SCREENSHOT_DIR,service+'-'+name+'.png'),fullPage:true});}};
 return {page,errors,requests,capture,close:async()=>{assert.deepEqual(errors,[],'Browser errors');await capture(String(screenshotIndex++));await page.close();}};
}

test('Control Room renders live readiness and excludes resolved incident actions',async()=>{
 const v=await pageFor('control');await v.page.waitForSelector('#services tr');
 assert.match(await v.page.locator('#services').innerText(),/Bereit/);assert.match(await v.page.locator('#services').innerText(),/Nicht bereit/);
 assert.equal(await v.page.locator('#kpis .kpi').count(),4);assert.equal(await v.page.locator('#supplementary-kpis .kpi').count(),8);
 const done=v.page.locator('#incidents tr').filter({hasText:'Behoben'});assert.equal(await done.locator('button').count(),0);
 assert.equal(await v.page.locator('.nc-service-access').innerText(),'OPEN LAB');
 await v.page.locator('#poll').click();assert.equal(v.requests[0].path,'/api/v1/services/poll');await v.close();
});
test('Control Room submits the typed DGNA shortcut with real numeric fields',async()=>{
 const v=await pageFor('control');await v.page.locator('[name=node_id]').fill('TBS-01');await v.page.locator('[name=action]').selectOption('dgna-attach');await v.page.locator('[name=issi]').fill('1001');await v.page.locator('[name=gssi]').fill('2001');await v.page.locator('#command-form button[type=submit]').click();await v.page.waitForFunction(()=>document.querySelector('#command-result').textContent.includes('Angenommen'));
 assert.equal(v.requests[0].path,'/api/nodes/TBS-01/commands/dgna');assert.deepEqual(v.requests[0].body,{operator_id:'jan',issi:1001,gssi:2001,attach:true});await v.close();
});
test('protected Control Room declares the existing HTTP Basic mode',async()=>{
 const v=await pageFor('protected');assert.equal(await v.page.locator('.nc-service-access').innerText(),'HTTP-Basic');assert.equal(await v.page.getByText('OPEN LAB',{exact:true}).count(),0);await v.close();
});
test('Alarm open state can resolve and has a future escalation countdown',async()=>{
 const v=await pageFor('alarm');await v.page.waitForSelector('.alarm');assert.equal(await v.page.locator('.alarm').count(),1);assert.doesNotMatch(await v.page.locator('.deadline').innerText(),/vor/);await v.page.locator('.alarm h3').click();assert.match(await v.page.locator('#detail-title').innerText(),/AL-0042/);
 await v.page.locator('#detail-body').getByRole('button',{name:'Lösen',exact:true}).click();await v.page.locator('#mn').fill('Link stabil');await v.page.locator('#modalBody').getByRole('button',{name:'Lösen',exact:true}).click();await v.page.waitForFunction(()=>!document.querySelector('#modal').classList.contains('open'));
 assert.equal(v.requests[0].path,'/api/v1/alarms/alarm-42/resolve');assert.equal(v.requests[0].body.note,'Link stabil');await v.close();
});
test('Alarm filters retain closed states and manual creation uses the selected profile',async()=>{
 const v=await pageFor('alarm');await v.page.waitForSelector('.alarm');await v.page.getByRole('button',{name:'Gelöst',exact:true}).click();assert.match(await v.page.locator('#alarms').innerText(),/Behoben/);assert.equal(await v.page.locator('#alarms').getByRole('button',{name:'Lösen',exact:true}).count(),0);
 await v.page.getByRole('button',{name:'＋ Alarm',exact:true}).click();await v.page.locator('#ct').fill('Testalarm');await v.page.locator('#cd').fill('Link prüfen');await v.page.locator('#cs').selectOption('critical');await v.page.locator('#cp').fill('10');await v.page.getByRole('button',{name:'Alarm auslösen',exact:true}).click();await v.page.waitForFunction(()=>!document.querySelector('#modal').classList.contains('open'));
 assert.equal(v.requests[0].path,'/api/v1/alarms');assert.equal(v.requests[0].body.title,'Testalarm');assert.equal(v.requests[0].body.escalation_profile,'critical');await v.close();
});
test('Task five lanes retain accepted, completed, cancelled and expired orders',async()=>{
 const v=await pageFor('task');await v.page.waitForSelector('.task');assert.equal(await v.page.locator('.lane').count(),5);assert.match(await v.page.locator('[data-lane=assigned]').innerText(),/Angenommen/);await v.page.getByRole('button',{name:'Alle',exact:true}).click();assert.equal(await v.page.locator('.task').count(),7);assert.match(await v.page.locator('[data-lane=completed]').innerText(),/Abgebrochen/);assert.match(await v.page.locator('[data-lane=completed]').innerText(),/Abgelaufen/);await v.close();
});
test('Task notification and action semantics do not imply handset reception',async()=>{
 const v=await pageFor('task');await v.page.waitForSelector('.task');await v.page.locator('.task').first().click();assert.match(await v.page.locator('#detail-body').innerText(),/Keine Empfangsbestätigung/);assert.match(await v.page.locator('#detail-body').innerText(),/Beim SDS-Router eingereiht/);
 await v.page.locator('#detail-body').getByRole('button',{name:'SDS senden',exact:true}).click();await v.page.waitForTimeout(50);assert.equal(v.requests[0].path,'/api/v1/tasks/task-81/notify');assert.equal(v.requests[0].body.actor,'webui-openlab');await v.close();
});
test('Task creation collects dynamic template fields and numeric recipient IDs',async()=>{
 const v=await pageFor('task');await v.page.waitForSelector('.task');await v.page.getByRole('button',{name:'＋ Neuer Auftrag',exact:true}).click();await v.page.locator('#title').fill('Fahrzeugcheck');await v.page.locator('#issi').fill('1001');await v.page.locator('#pri').fill('11');await v.page.locator('[data-f=vehicle]').fill('HLF-20');await v.page.getByRole('button',{name:'Anlegen',exact:true}).click();await v.page.waitForFunction(()=>!document.querySelector('#modal').classList.contains('open'));
 assert.equal(v.requests[0].path,'/api/v1/tasks');assert.equal(v.requests[0].body.priority,11);assert.equal(v.requests[0].body.assigned_issi,1001);assert.deepEqual(v.requests[0].body.form_data,{vehicle:'HLF-20'});await v.close();
});
test('Warnzentrale loads the shell under actual CSP and hides private workspace before key entry',async()=>{
 const v=await pageFor('alert');await v.page.waitForFunction(()=>document.querySelector('#connection').textContent==='Nicht verbunden');
 assert.equal(await v.page.locator('#token').getAttribute('type'),'password');assert.equal(await v.page.locator('#warning-workspace').isVisible(),false);assert.equal(await v.page.locator('#device-checks').isVisible(),false);assert.equal(await v.page.locator('#logout').isVisible(),false);assert.equal(await v.page.locator('.nc-service-access').innerText(),'Zugangsschlüssel');
 await v.capture('login');await v.page.locator('#token').fill('fixture-key');await v.page.locator('#login-form button').click();await v.page.waitForFunction(()=>document.querySelector('#login').hidden);
 assert.equal(await v.page.locator('#token').inputValue(),'');assert.equal(await v.page.evaluate(()=>sessionStorage.getItem('netcore-alert-token')),'fixture-key');assert.equal(await v.page.locator('#warning-workspace').isVisible(),true);assert.equal(await v.page.locator('#map').evaluate(n=>n.clientWidth>0&&n.clientHeight>0),true);await v.close();
});
test('Warnzentrale excluded GPS rows and accepted history retain honest states',async()=>{
 const v=await pageFor('alert',{authorized:true});await v.page.waitForSelector('#device-check-list tr');await v.page.waitForFunction(()=>document.querySelector('#login').hidden);
 assert.match(await v.page.locator('#device-check-list').innerText(),/Wartet auf Gerätedaten/);assert.match(await v.page.locator('#device-check-list').innerText(),/Keine Empfangsbestätigung/);assert.equal(await v.page.locator('#manual-list').getByRole('button',{name:'Eigene Meldung „Zufahrt freihalten“ löschen',exact:true}).count(),1);await v.close();
});
test('Warnzentrale own warning form keeps real coordinates, radius, severity and auth request',async()=>{
 const v=await pageFor('alert',{authorized:true});await v.page.waitForFunction(()=>document.querySelector('#login').hidden);await v.page.locator('#new-alert').click();await v.page.locator('[name=title]').fill('Einsatzstelle');await v.page.locator('[name=description]').fill('Zufahrt freihalten');await v.page.locator('[name=latitude]').fill('52.3759');await v.page.locator('[name=longitude]').fill('9.732');await v.page.locator('[name=radius_m]').fill('2000');await v.page.locator('#save-alert').click();await v.page.waitForFunction(()=>document.querySelector('#composer').hidden);
 assert.equal(v.requests[0].path,'/api/v1/alerts');assert.equal(v.requests[0].body.latitude,52.3759);assert.equal(v.requests[0].body.longitude,9.732);assert.equal(v.requests[0].body.radius_m,2000);assert.equal(v.requests[0].body.severity,'Moderate');assert.equal(v.requests[0].headers.authorization,'Bearer fixture-key');await v.close();
});


test('workflow desktop themes and narrow screens keep forms, tables and cards reachable',async()=>{
 for(const service of ['control','alarm','task','alert']){
  const v=await pageFor(service,{authorized:service==='alert'});
  await v.page.locator('.nc-theme-toggle').click();assert.equal(await v.page.locator('html').getAttribute('data-nc-theme'),'dark');
  await v.page.locator('.nc-theme-toggle').click();await v.page.setViewportSize({width:420,height:900});
  assert.ok(await v.page.evaluate(()=>document.documentElement.scrollWidth<=window.innerWidth+1),'Page width is contained for '+service);
  if(service==='alarm'||service==='task'){await v.page.locator(service==='alarm'?'.alarm':'.task').first().click();assert.equal(await v.page.locator('#detail-title').isVisible(),true);}
  await v.close();
 }
});
