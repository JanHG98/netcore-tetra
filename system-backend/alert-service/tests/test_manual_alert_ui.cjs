const assert=require('node:assert/strict');
const fs=require('node:fs');
const path=require('node:path');
const vm=require('node:vm');
const {test}=require('node:test');

class Element {
  constructor(tag='div'){this.tagName=tag;this.children=[];this.attributes={};this.value='';this.style={};}
  set textContent(v){this.text=String(v);this.children=[];}
  get textContent(){return (this.text||'')+this.children.map(c=>c.textContent).join('');}
  append(...c){this.children.push(...c);}
  replaceChildren(...c){this.text='';this.children=c;}
  setAttribute(k,v){this.attributes[k]=v;}
  addEventListener(){}
  scrollIntoView(){this.scrolled=true;}
  showModal(){this.open=true;}
  close(){this.open=false;}
  click(){this.onclick?.();}
  reset(){this.resetCalled=true;}
}
const initial=()=>({alerts:[],devices:[],deliveries:[],errors:{},now:'2026-09-27T12:00:00Z',last_cycle:1000,delivery_enabled:true,router_ready:true,device_overview:[]});
const warning=(id,extra={})=>({id,source:'manual',title:id,active:true,eligible:true,removed:false,sent_at:'2026-09-27T12:00:00Z',starts_at:'2026-09-27T12:00:00Z',expires_at:'2026-09-28T12:00:00Z',description:'Testmeldung',geometry:{type:'Circle',coordinates:[13,52],radius_m:1000},...extra});
const settle=()=>new Promise(resolve=>setImmediate(resolve));
function ui(){
  const nodes=new Map(), requests=[];
  const get=id=>{if(!nodes.has(id))nodes.set(id,new Element());return nodes.get(id);};
  get('alert-form').elements=Object.fromEntries(['latitude','longitude','radius_m','expires_at'].map(n=>[n,new Element('input')]));
  const layer=()=>({layers:[],setView(){return this;},on(){return this;},removeLayer(){},addTo(parent){parent.layers?.push(this);return this;},clearLayers(){this.layers=[];},getLayers(){return this.layers;},getBounds(){return {isValid:()=>true};},bindPopup(){return this;},fitBounds(){},bindTooltip(){return this;}});
  const response=(data,status=200)=>({ok:status<400,status,json:async()=>data});
  const context=vm.createContext({document:{getElementById:get,createElement:t=>new Element(t)},L:{map:layer,tileLayer:layer,featureGroup:layer,layerGroup:layer,circle:layer,geoJSON:layer},sessionStorage:{getItem:()=>''},console,Date,Map,setInterval:()=>{},FormData:class{*[Symbol.iterator](){yield* Object.entries({title:'Neu',description:'Text',latitude:'52',longitude:'13',radius_m:'1000',expires_at:'2026-09-28T12:00:00Z'});}},fetch:(url,options)=>new Promise(resolve=>requests.push({url,options,reply:(data,status)=>resolve(response(data,status))}))});
  vm.runInContext(fs.readFileSync(path.join(__dirname,'../static/app.js'),'utf8')+'\nglobalThis.testSet=value=>{snapshot=value;render();};globalThis.testRefresh=refresh;',context);
  return {get,requests,context,set:extra=>context.testSet({...initial(),...extra})};
}
const buttons=element=>element.children.flatMap(c=>[...(c.tagName==='button'?[c]:[]),...buttons(c)]);

test('sent own warning stays in its stable list across provider refreshes; deletion remains available',()=>{
  const view=ui();
  const own=warning('manual:one',{title:'<img src=x onerror=alert(1)>'});
  const nina=Array.from({length:100},(_,i)=>warning('nina:'+i,{source:'nina'}));
  view.set({alerts:[...nina,own],deliveries:[{alert_id:own.id,issi:5102,state:'accepted'}]});
  const card=view.get('manual-list').children[0];
  assert.equal(view.get('manual-list').children.length,1);
  assert.match(card.textContent,/Aktiv.*Von TBS angenommen.*Löschen/);
  assert.equal(card.children[1].textContent,own.title);
  assert.equal(card.children[1].children.length,0);
  assert.doesNotMatch(view.get('alert-list').textContent,/manual:one|<img/);
  view.set({alerts:[...nina.reverse(),own]});
  assert.match(view.get('manual-list').textContent,/Löschen/);
  buttons(view.get('manual-list')).find(b=>b.textContent==='Löschen').click();
  assert.equal(view.get('delete-dialog').open,true);
  assert.equal(view.get('delete-title').textContent,own.title);
});

test('search and filters include expired and deleted warnings without offering deletion twice',()=>{
  const view=ui();
  view.set({alerts:[warning('old',{active:false}),warning('gone',{active:false,removed:true}),warning('new',{sent_at:'2026-09-27T13:00:00Z'})]});
  assert.match(view.get('manual-list').children[0].textContent,/new/);
  view.get('manual-filter-ended').click();
  assert.equal(view.get('manual-list').children.length,2);
  assert.equal(buttons(view.get('manual-list')).filter(b=>b.textContent==='Löschen').length,1);
  view.get('manual-search').value='gone';view.get('manual-search').oninput();
  assert.match(view.get('manual-list').textContent,/Gelöscht · Versand gestoppt/);
  assert.equal(buttons(view.get('manual-list')).filter(b=>b.textContent==='Löschen').length,0);
});

test('delete during an in-flight poll cannot bring an active warning back',async()=>{
  const view=ui(), own=warning('manual:one');
  view.set({alerts:[own]});
  buttons(view.get('manual-list')).find(b=>b.textContent==='Löschen').click();
  const deleting=view.get('confirm-delete').onclick();
  assert.match(view.requests[1].url,/alerts\/manual%3Aone$/);
  view.requests[1].reply({deleted:true});await settle();
  assert.match(view.get('manual-list').textContent,/Gelöscht/);
  view.requests[0].reply({...initial(),alerts:[own]});await settle();
  assert.match(view.get('manual-list').textContent,/Gelöscht/,'stale poll ignored');
  assert.equal(view.requests[2].url,'/api/v1/status');
  view.requests[2].reply({...initial(),alerts:[{...own,active:false,removed:true}]});await deleting;
  assert.equal(view.get('delete-dialog').open,false);
  assert.match(view.get('manual-result').textContent,/Weitere Zustellungen werden gestoppt/);
});

test('failed delete keeps the warning and displays the server error in the dialog',async()=>{
  const view=ui();view.requests[0].reply(initial());await settle();
  view.set({alerts:[warning('manual:one')]});
  buttons(view.get('manual-list')).find(b=>b.textContent==='Löschen').click();
  const deleting=view.get('confirm-delete').onclick();
  view.requests[1].reply({error:'Dienst nicht erreichbar'},503);await deleting;
  assert.equal(view.get('delete-dialog').open,true);
  assert.equal(view.get('delete-error').textContent,'Dienst nicht erreichbar');
  assert.match(view.get('manual-list').textContent,/Aktiv/);
  assert.equal(view.get('confirm-delete').disabled,false);
});

test('create during an in-flight poll resets filters and keeps the created warning visible',async()=>{
  const view=ui();view.set({});
  view.get('manual-filter-ended').click();view.get('manual-search').value='old filter';
  const creating=view.get('alert-form').onsubmit({preventDefault(){}});
  const own=warning('manual:new',{title:'Neu'});view.requests[1].reply(own,201);await settle();
  assert.match(view.get('manual-list').textContent,/Neu/);
  assert.equal(view.get('manual-search').value,'');
  assert.equal(view.get('manual-alerts').scrolled,true);
  view.requests[0].reply(initial());await settle();
  assert.match(view.get('manual-list').textContent,/Neu/);
  view.requests[2].reply({...initial(),alerts:[own]});await creating;
  assert.match(view.get('manual-result').textContent,/jederzeit löschen/);
});
