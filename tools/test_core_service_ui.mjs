#!/usr/bin/env node
// Isolated browser checks against API shapes from the core services' state/protocol types.
// Fixture data is never included in a production service.
import assert from 'node:assert/strict';
import http from 'node:http';
import {mkdir} from 'node:fs/promises';
import path from 'node:path';
import {fileURLToPath} from 'node:url';
import {createRequire} from 'node:module';
import {execFileSync} from 'node:child_process';
const require=createRequire(import.meta.url);
let chromium;try{({chromium}=require('playwright'))}catch(e){if(!process.env.CODEX_PRIMARY_RUNTIME_NODE_MODULES)throw e;({chromium}=require(path.join(process.env.CODEX_PRIMARY_RUNTIME_NODE_MODULES,'playwright')))}
const root=path.resolve(path.dirname(fileURLToPath(import.meta.url)),'..'),output=path.join(root,'target/service-ui-preview/core');await mkdir(output,{recursive:true});
const services=['node-gateway','subscriber-core','group-core','mobility-core','call-control','media-switch','sds-router','packet-core','ip-gateway'];
const names=['Node Gateway','Subscriber Core','Group Core','Mobility Core','Call Control','Media Switch','SDS Router','Packet Core','IP Gateway'];
const shared=path.join(root,'system-backend/shared/web-ui');
// Compile the actual dependency-free renderer, including its early theme restore.
const rendererSource=path.join(output,'render-core.rs'),renderer=path.join(output,'render-core');
await import('node:fs/promises').then(({writeFile})=>writeFile(rendererSource,`#[path=${JSON.stringify(path.join(shared,'service-design.rs'))}] mod service_design; fn main(){let a:Vec<String>=std::env::args().collect();let h=std::fs::read_to_string(&a[1]).unwrap();print!("{}",service_design::render(&h,&a[2],"open-lab"));}`));
execFileSync(process.env.RUSTC||'rustc',['--edition=2024',rendererSource,'-o',renderer]);
const time='2026-10-03T00:00:00Z';
const node={node_id:'tbs-test-a',station_name:'Test-TBS A',connected:true,stale:false,last_seen:time,mcc:901,mnc:1510,location_area:1,colour_code:1,main_carrier:33440,secondary_carrier:null,stack_version:'0.4',group_policy_capable:true,call_control_capable:true,call_restore_capable:true,media_bridge:true,media_frame_count:25,sds_capable:true,packet_data_capable:true,multi_pdch_capable:true,gateway_running:true,interface_name:'ntc-pd0',gateway_address:'10.0.0.1',active_contexts:1,active_bearers:1,bearer_capacity:4,packets_from_mobile:4,packets_to_mobile:3,bytes_from_mobile:80,bytes_to_mobile:60};
const gatewayNode={...node,identity:{...node},peer:'127.0.0.1:5000',last_message_kind:'telemetry',message_count:35,telemetry_count:12,control_ack_count:2,capabilities:{managed_calls:true,media_bridge:true}};
const profile={issi:5102,display_name:'Testleitung <script>unsafe</script>',home_mcc:901,home_mnc:1510,organization:'UI-Test',device_label:'Testgerät',device_tei:1001,enabled:true,registration_allowed:true,call_priority:4,emergency_allowed:true,sds_allowed:true,packet_data_allowed:true,default_groups:[15201],notes:'Nur Testdaten'};
const group={gssi:15201,name:'Testgruppe',description:'UI-Test',enabled:true,attach_allowed:true,dgna_allowed:true,call_allowed:true,sds_allowed:true,emergency_allowed:false,call_priority:4,class_of_usage:4,area_nodes:[],notes:''};
const observed={issi:5102,serving_node:node.node_id,registered:true,known_profile:true,authorized:true,groups:[15201],last_rssi_dbfs:-43.2,last_seen:time,energy_saving_mode:0};
const sync={node_id:node.node_id,applied_revision:42,desired_revision:42,phase:'synchronized',message:'Test policy applied'};
const call={logical_call_id:'nc-test-1',operation_id:'op-test-1',kind:'group',phase:'active',managed:true,source:'test',source_issi:5102,gssi:15201,calling_issi:null,called_issi:null,simplex:true,priority:4,emergency:false,floor_holder:5102,floor_queue:[5103,5104],legs:{[node.node_id]:{node_id:node.node_id,local_call_id:61,operation_id:'op-leg-1',phase:'active',timeslot:1,carrier_num:0,usage:4,floor_holder:5102,queued_issi:null,restored:false,created_at:time,updated_at:time,message:''}},created_at:time,updated_at:time};
const context={id:'context-test-1',issi:5102,nsapi:1,node_id:node.node_id,anchor_node_id:node.node_id,ipv4:'10.0.0.2',primary_nsapi:null,snei:1001,mtu:480,priority:4,state:'ready',available:true,usage_active:true,source:'shadow',created_at:time,updated_at:time,last_activity_at:time,packets_up:4,bytes_up:80,packets_down:3,bytes_down:60,dropped_packets:0,queued_packets:0,queued_bytes:0,carrier_num:0,logical_ts:1,air_ts:1,last_error:null,revision:42};
const message={id:'message-test-1',created_at:time,source_issi:5102,dest_issi:5103,is_group:false,sds_type:4,protocol_id:130,priority:4,state:'queued',text_preview:'Browser-Test',payload_hex:'54657374',expires_at:'2026-10-03T00:05:00Z',delivered_legs:0,total_legs:1,application_pending:0,last_error:null};
const statusKeys='connected_nodes known_nodes stale_nodes available_services monitored_services degraded_services unavailable_services backend_clients total_node_messages total_media_frames total_commands subscribers_total subscribers_authorized subscribers_blocked observed_registered nodes_connected nodes_synced database_revision groups_total groups_enabled memberships_total observed_affiliations dgna_pending subscribers_known transfers_active transfers_completed transfers_failed calls_active call_legs_active calls_managed participants_registered pending_commands restores_pending sessions_active streams_active pending_frames frames_received frames_routed frames_sent frames_dropped duplicate_frames frames_injected messages_total queued offline in_flight delivered dead_letter duplicate_messages contexts_total contexts_ready contexts_standby contexts_suspended bearers_active actions_pending reassemblies_active npdu_outbox queued_packets queued_bytes contexts flows packets_uplink packets_downlink packets_dropped captures_active dns_queries test_requests'.split(' ');
const status={...Object.fromEntries(statusKeys.map(k=>[k,0])),node_gateway_connected:true,call_control_connected:true,packet_core_connected:true,mode:'shadow',access_mode:'allow_known',authoritative:false,tun_open:false,tun_name:'ntc-tun0',kernel_last_error:null,connected_nodes:1,known_nodes:1,available_services:1,monitored_services:1,nodes_connected:1,nodes_synced:1,database_revision:42,subscribers_total:1,subscribers_authorized:1,observed_registered:1,groups_total:1,groups_enabled:1,memberships_total:1,observed_affiliations:1,calls_active:1,call_legs_active:1,calls_managed:1,participants_registered:1,sessions_active:1,streams_active:1,contexts_total:1,contexts_ready:1,contexts:1,messages_total:1,queued:1,subscribers_known:1};
function data(service,p){
 if(p==='/api/v1/status')return status;if(p==='/api/v1/config')return {security:{mode:'open_lab'},test_fixture:true};
 if(p==='/api/v1/core-services')return {services:[{service:'Subscriber Core',level:'available',critical_for_edge:true,fallback_mode:'local-policy',checked_at:time,last_success_at:time,message:''}]};
 if(p==='/api/v1/nodes')return service==='node-gateway'?[gatewayNode]:[node,{...node,node_id:'tbs-test-b',station_name:'Test-TBS B'}];
 if(p==='/api/v1/subscribers')return service==='subscriber-core'?[profile]:service==='mobility-core'?[observed]:[{issi:5102,node_id:node.node_id}];
 if(p==='/api/v1/observed')return [observed];if(p==='/api/v1/syncs')return [sync];
 if(p==='/api/v1/groups')return service==='sds-router'?[{gssi:15201,nodes:[node.node_id]}]:[group];
 if(p==='/api/v1/memberships')return [{issi:5102,gssi:15201,allowed:true,auto_attach:true,locked:false,notes:''}];
 if(p==='/api/v1/affiliations'||p==='/api/v1/participants')return [{...observed,node_id:node.node_id}];
 if(p==='/api/v1/calls')return [call];if(p==='/api/v1/sessions')return [{...call,frames_received:25,frames_routed:25,frames_dropped:0}];
 if(p==='/api/v1/streams')return [{session_id:call.logical_call_id,node_id:node.node_id,logical_ts:1,local_call_id:61,carrier_num:0,phase:'active',muted:true,rx_frames:25,tx_frames:25,dropped_frames:0,last_sequence:25}];
 if(p==='/api/v1/buffers')return [{session_id:call.logical_call_id,target_node_id:node.node_id,target_logical_ts:1,queued_frames:1,oldest_due_in_ms:25}];
 if(p==='/api/v1/messages')return [message];if(p==='/api/v1/messages/'+message.id)return {...message,delivery_legs:[],application_legs:[],trace:[{timestamp:time,kind:'queued',detail:'Browser-Test'}]};
 if(p==='/api/v1/contexts')return [context];if(p==='/api/v1/kernel/plan')return {revision:42,routes:[],nat_rules:[],firewall_rules:[],blocked_addresses:[]};
 if(p==='/api/v1/events')return [{seq:1,sequence:1,timestamp:time,kind:'test_event',detail:{}}];return [];
}
let activeService=services[0];const writes=[],documents=new Map();
for(let i=0;i<services.length;i++)documents.set(services[i],execFileSync(renderer,[path.join(root,'system-backend',services[i],'web-ui/index.html'),names[i]],{encoding:'utf8',maxBuffer:4*1024*1024}));
const server=http.createServer(async(req,res)=>{try{const url=new URL(req.url,'http://localhost');if(url.pathname==='/'){activeService=url.searchParams.get('service')||activeService;res.setHeader('Content-Type','text/html');return res.end(documents.get(activeService))}if(req.method!=='GET'){const chunks=[];for await(const c of req)chunks.push(c);const text=Buffer.concat(chunks).toString();writes.push({service:activeService,path:url.pathname,method:req.method,body:text?JSON.parse(text):null});res.setHeader('Content-Type','application/json');return res.end(JSON.stringify({queued:1,ok:true}))}res.setHeader('Content-Type','application/json');res.end(JSON.stringify(data(activeService,url.pathname)))}catch(e){res.statusCode=500;res.end(JSON.stringify({error:e.message}))}});
await new Promise(resolve=>server.listen(0,'127.0.0.1',resolve));
const base='http://127.0.0.1:'+server.address().port,browser=await chromium.launch({headless:true,...(process.env.CHROMIUM_EXECUTABLE_PATH?{executablePath:process.env.CHROMIUM_EXECUTABLE_PATH}:{}),args:['--no-sandbox']});
let checks=0;const errors=[];const check=(v,m)=>{assert.ok(v,m);checks++};

// Check actual rendered colors, including translucent ancestor backgrounds.
// Normal text, selected rows, muted labels and editable control values need 4.5:1.
async function readable(page,description){
 const result=await page.evaluate(()=>{
  const rgba=value=>{const n=value.match(/[\d.]+/g)?.map(Number)||[];return n.length>=3?[n[0],n[1],n[2],n[3]??1]:[0,0,0,0]};
  const over=(fg,bg)=>{const a=fg[3]+bg[3]*(1-fg[3]);return a?[0,1,2].map(i=>(fg[i]*fg[3]+bg[i]*bg[3]*(1-fg[3]))/a).concat(a):[0,0,0,0]};
  const bgFor=el=>{const stack=[];for(let n=el;n;n=n.parentElement)stack.push(rgba(getComputedStyle(n).backgroundColor));return stack.reverse().reduce((bg,fg)=>over(fg,bg),[255,255,255,1])};
  const lum=rgb=>rgb.slice(0,3).map(v=>v/255).map(v=>v<=.04045?v/12.92:((v+.055)/1.055)**2.4).reduce((a,v,i)=>a+v*[.2126,.7152,.0722][i],0);
  const failures=[];let inspected=0;
  const regions='.nc-service-header *, .panel *, .cards *, .lab, .warn, .core-note, .form *, dialog[open] *';
  for(const el of document.querySelectorAll(regions)){
   const style=getComputedStyle(el),rect=el.getBoundingClientRect();
   if(!el.getClientRects().length||rect.width===0||rect.height===0||style.visibility!=='visible'||el.disabled||el.matches('script,style,img,canvas,svg,input[type=checkbox],input[type=radio],input[type=hidden]'))continue;
   const text=[...el.childNodes].filter(n=>n.nodeType===Node.TEXT_NODE).map(n=>n.textContent).join('').trim();
   if(!text&&!el.matches('input,select,textarea'))continue;
   const bg=bgFor(el),fg=over(rgba(style.color),bg),a=lum(fg),b=lum(bg),ratio=(Math.max(a,b)+.05)/(Math.min(a,b)+.05);
   const large=parseFloat(style.fontSize)>=24||(parseFloat(style.fontSize)>=18.66&&Number(style.fontWeight)>=700);inspected++;
   if(ratio<(large?3:4.5)-.02)failures.push({tag:el.tagName,id:el.id,text:(text||el.value||el.placeholder||'').slice(0,70),ratio:Number(ratio.toFixed(2)),color:style.color,background:backgroundColor(el)});
   function backgroundColor(el){return bgFor(el).slice(0,3).map(Math.round).join(',')}
  }
  return {inspected,failures};
 });
 check(result.inspected>0,description+' contains readable content');
 check(result.failures.length===0,description+' contrast failures: '+JSON.stringify(result.failures));
}
async function theme(page,mode){const current=await page.evaluate(()=>document.documentElement.dataset.ncTheme);if(current!==mode)await page.locator('.nc-theme-toggle').click();await page.waitForFunction(mode=>document.documentElement.dataset.ncTheme===mode,mode);check(await page.locator('.nc-theme-toggle').getAttribute('aria-pressed')===(mode==='dark'?'true':'false'),'Theme button reports '+mode);check(await page.evaluate(()=>localStorage.getItem('netcore-theme'))===mode,'Canonical preference stores '+mode)}
async function inspectDialogs(page,service){
 if(service==='ip-gateway'){
  for(const kind of ['routes','firewall','nat','dns','captures']){await page.evaluate(kind=>corePolicy(kind),kind);await readable(page,service+' dark '+kind+' form');await page.evaluate(()=>document.getElementById('corePolicyDialog').close())}
 }else{
  const ids=await page.locator('dialog').evaluateAll(ds=>ds.map(d=>d.id));
  for(const id of ids){await page.evaluate(id=>document.getElementById(id).showModal(),id);await readable(page,service+' dark dialog '+id);await page.evaluate(id=>document.getElementById(id).close(),id)}
 }
}
async function assertWrite(page,action,p,validate){const before=writes.length;if(await page.locator('dialog[open]').count())await readable(page,activeService+' populated dark form before submit');await action();await page.waitForFunction(()=>!document.querySelector('dialog[open]'),null,{timeout:1000}).catch(()=>{});await new Promise(r=>setTimeout(r,100));const w=writes.slice(before).find(x=>x.path===p);check(!!w,'Expected API write '+p);if(validate){validate(w.body);checks++}}
try{
 // Reduced motion removes intermediate transition colors from contrast measurements.
 const page=await browser.newPage({viewport:{width:1600,height:1000},reducedMotion:'reduce'});page.on('pageerror',e=>errors.push({service:activeService,error:e.message}));page.on('dialog',d=>d.accept(d.type()==='prompt'?'5102':undefined));
 await page.addInitScript(()=>{const observer=new MutationObserver(()=>{if(document.body){window.__coreFirstBodyTheme=document.documentElement.dataset.ncTheme;observer.disconnect()}});observer.observe(document,{childList:true,subtree:true})});
 for(const service of services){
  await page.goto(base+'/?service='+service);await page.waitForSelector('.nc-service-header');await page.waitForFunction(()=>document.querySelectorAll('#cards .card').length>0);
  check(!(await page.locator('body').innerText()).includes('UI-Fehler:'),service+' protocol shape renders');check((await page.locator('body').innerText()).includes('OPEN LAB'),service+' access mode visible');check(await page.locator('.nc-service-logo-wordmark img').count()===1,service+' logo');
  const tabs=page.locator('.nc-service-nav button'),count=await tabs.count();check(count>=4,service+' navigation');for(let i=0;i<count;i++){await tabs.nth(i).click();check(await page.locator('.panel:visible').count()>0,service+' tab '+i+' has content')}await tabs.first().click();await theme(page,'dark');for(let i=0;i<count;i++){await tabs.nth(i).click();await readable(page,service+' dark tab '+i)}await tabs.first().click();
  if(service==='node-gateway'){await page.locator('#nodes [data-core-action="node-select"]').click();check((await page.locator('#coreNodeDetail').innerText()).includes('Mediaframes'),'Gateway media frame units');await assertWrite(page,()=>page.locator('#coreNodeDetail [data-core-action="node-ping"]').click(),'/api/v1/nodes/'+node.node_id+'/ping')}
  if(service==='subscriber-core'){
   await page.locator('#subscriberRows [data-core-action="subscriber-select"]').click();check((await page.locator('#coreSubscriberDetail').innerText()).includes('<script>unsafe</script>'),'Untrusted profile text stays text');check(await page.locator('#coreSubscriberDetail script').count()===0,'Profile cannot inject script');
   await page.getByRole('button',{name:'Teilnehmer anlegen',exact:true}).click();await page.locator('#form [name="issi"]').fill('6001');await page.locator('#form [name="display_name"]').fill('Neues Testprofil');await assertWrite(page,()=>page.locator('#form button[type="submit"]').click(),'/api/v1/subscribers',p=>assert.equal(p.issi,6001));await page.locator('#coreSubscriberDetail [data-core-action="subscriber-edit"]').click();check(await page.locator('#form [name="issi"]').isDisabled(),'Existing ISSI immutable');await readable(page,'Subscriber dark edit form');await page.locator('#form button[type="button"]').click();
  }
  if(service==='group-core'){
   await page.locator('#groupRows [data-core-action="group-select"]').click();check((await page.locator('#coreGroupDetail').innerText()).includes('Konfigurierte Mitgliedschaften'),'Policy distinct from registration');await page.getByRole('button',{name:'Gruppe anlegen',exact:true}).click();await page.locator('#groupForm [name="gssi"]').fill('15501');await assertWrite(page,()=>page.locator('#groupForm button[type="submit"]').click(),'/api/v1/groups',p=>assert.equal(p.gssi,15501));await page.getByRole('button',{name:'Mitgliedschaft hinzufügen',exact:true}).click();await page.locator('#membershipForm [name="issi"]').fill('6001');await page.locator('#membershipForm [name="gssi"]').fill('15201');await assertWrite(page,()=>page.locator('#membershipForm button[type="submit"]').click(),'/api/v1/memberships',p=>assert.equal(p.allowed,true));
  }
  if(service==='mobility-core'){await page.locator('#issi').fill('5102');await page.locator('#source').selectOption(node.node_id);await page.locator('#target').selectOption('tbs-test-b');await assertWrite(page,()=>page.getByRole('button',{name:'Transfer starten',exact:true}).click(),'/api/v1/transfers',p=>assert.equal(p.target_node,'tbs-test-b'))}
  if(service==='call-control'){
   await page.locator('#callRows [data-core-action="call-select"]').click();check((await page.locator('#coreCallDetail').innerText()).includes('Warteschlange (2)'),'Floor queue ordered ISSIs');await page.locator('#coreCallDetail [data-core-action="call-floor"]').click();await page.locator('#coreFloorForm [name="source_issi"]').fill('5103');await assertWrite(page,()=>page.locator('#coreFloorForm button[type="submit"]').click(),'/api/v1/calls/'+call.logical_call_id+'/floor',p=>{assert.equal(p.source_issi,5103);assert.equal(p.force,false)});await tabs.nth(1).click();await page.locator('#ggssi').fill('15201');await page.locator('#gsource').fill('5102');await assertWrite(page,()=>page.locator('[data-core-view="start"]').first().getByRole('button',{name:'Starten',exact:true}).click(),'/api/v1/calls/group',p=>assert.equal(p.gssi,15201));await tabs.first().click();
  }
  if(service==='media-switch'){await page.locator('#sessionRows [data-core-action="media-select"]').click();check((await page.locator('#coreMediaDetail').innerText()).includes('RX-Frames'),'Media session actual counters');await assertWrite(page,()=>page.locator('#streamRows button').click(),'/api/v1/sessions/'+call.logical_call_id+'/mute',p=>assert.equal(p.muted,false))}
  if(service==='sds-router'){
   await page.locator('#messageRows button').first().click();await page.waitForFunction(()=>document.querySelector('#coreMessageDetail').textContent.includes('Message-ID'));check((await page.locator('#coreMessageDetail').innerText()).includes(message.id),'SDS complete details');await page.getByRole('button',{name:'Nachricht senden',exact:true}).click();await page.locator('#messageForm [name="source_issi"]').fill('5102');await page.locator('#messageForm [name="dest_issi"]').fill('5103');await page.locator('#messageForm [name="text"]').fill('Browser-Test SDS');await assertWrite(page,()=>page.locator('#messageForm button[type="submit"]').click(),'/api/v1/messages',p=>{assert.equal(p.text,'Browser-Test SDS');assert.equal(p.sds_type,4)});
  }
  if(service==='packet-core'){
   await tabs.nth(1).click();await page.locator('#contextRows [data-core-action="context-select"]').click();check((await page.locator('#coreContextDetail').innerText()).includes('Auch im Shadow-Modus'),'Shadow manual controls stay functional');await assertWrite(page,()=>page.locator('#coreContextDetail [data-core-action="context-wake"]').click(),'/api/v1/contexts/'+context.id+'/wake');await page.getByRole('button',{name:'N-PDU Downlink',exact:true}).click();await page.locator('#downlinkForm [name="issi"]').fill('5102');await page.locator('#downlinkForm [name="nsapi"]').fill('1');await page.locator('#downlinkForm [name="payload_hex"]').fill('45000014');await assertWrite(page,()=>page.locator('#downlinkForm button[type="submit"]').click(),'/api/v1/downlink',p=>assert.equal(p.payload_hex,'45000014'));
  }
  if(service==='ip-gateway'){
   await tabs.nth(2).click();for(const [label,p] of [['Route hinzufügen','routes'],['Regel hinzufügen','firewall'],['NAT-Regel hinzufügen','nat'],['A-Record hinzufügen','dns']]){await page.getByRole('button',{name:label,exact:true}).click();check(await page.locator('#corePolicyDialog').isVisible(),'IP '+p+' styled form');await assertWrite(page,()=>page.locator('#corePolicyForm button[type="submit"]').click(),'/api/v1/'+p,payload=>assert.equal(typeof payload.name,'string'))}await tabs.nth(3).click();await page.getByRole('button',{name:'Capture starten',exact:true}).click();await assertWrite(page,()=>page.locator('#corePolicyForm button[type="submit"]').click(),'/api/v1/captures',p=>assert.equal(p.direction,'both'));await tabs.first().click();
  }

  await readable(page,service+' selected detail and rows in dark');
  await inspectDialogs(page,service);
  await page.evaluate(()=>scrollTo(0,0));await page.mouse.move(1590,980);await page.waitForTimeout(100);
  await page.screenshot({path:path.join(output,service+'-dark.png'),fullPage:true});
  await page.setViewportSize({width:390,height:844});check(await page.evaluate(()=>document.documentElement.scrollWidth<=innerWidth+1),service+' dark mobile viewport does not overflow');await readable(page,service+' dark mobile');await page.evaluate(()=>scrollTo(0,0));await page.waitForTimeout(100);await page.screenshot({path:path.join(output,service+'-mobile-dark.png'),fullPage:true});
  await page.setViewportSize({width:1600,height:1000});await page.reload();await page.waitForSelector('.nc-service-header');await page.waitForFunction(()=>document.querySelectorAll('#cards .card').length>0);
  check(await page.evaluate(()=>document.documentElement.dataset.ncTheme==='dark'),service+' dark theme persists through reload');check(await page.evaluate(()=>window.__coreFirstBodyTheme==='dark'),service+' saved dark theme is restored before body renders');await readable(page,service+' restored dark theme');
  await theme(page,'light');check(await page.evaluate(()=>document.documentElement.dataset.ncTheme==='light'),service+' returns to light');
  // Keep the original light screenshots alongside the new dark variants.
  if(service==='packet-core')await page.locator('.nc-service-nav button').nth(1).click();
  if(service==='sds-router'){await page.locator('#messageRows button').first().click();await page.waitForFunction(()=>document.querySelector('#coreMessageDetail').textContent.includes('Message-ID'))}
  await page.evaluate(()=>scrollTo(0,0));await page.mouse.move(1590,980);await page.waitForTimeout(100);await page.screenshot({path:path.join(output,service+'.png'),fullPage:true});
  await page.setViewportSize({width:390,height:844});check(await page.evaluate(()=>document.documentElement.scrollWidth<=innerWidth+1),service+' light mobile viewport does not overflow');await page.evaluate(()=>scrollTo(0,0));await page.waitForTimeout(100);await page.screenshot({path:path.join(output,service+'-mobile.png'),fullPage:true});await page.setViewportSize({width:1600,height:1000});

 }
 check(errors.length===0,'Browser runtime errors: '+JSON.stringify(errors));console.log(JSON.stringify({services:services.length,checks,writes:writes.length,errors,screenshots:path.relative(root,output)},null,2));
}finally{await browser.close();await new Promise(resolve=>server.close(resolve))}
