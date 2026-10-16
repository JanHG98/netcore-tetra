const {chromium} = require('playwright');
const {spawn} = require('node:child_process');
const {once} = require('node:events');
const assert = require('node:assert/strict');
const path = require('node:path');

(async()=>{
  const fixture=spawn('python3',[path.join(__dirname,'serve_ui.py')],{stdio:['ignore','pipe','inherit']});
  let browser;
  try {
    // A failed fixture must fail CI, even if its stdout never emitted data.
    const [data]=await Promise.race([
      once(fixture.stdout,'data'),
      once(fixture,'exit').then(([code,signal])=>{throw new Error(`UI fixture exited before startup (${code ?? signal})`);}),
    ]);
    const url=data.toString().trim();
    const executablePath=process.env.CHROMIUM_EXECUTABLE_PATH || process.env.PLAYWRIGHT_CHROMIUM_EXECUTABLE;
    browser=await chromium.launch({headless:true,...(executablePath?{executablePath}:{})});
    const page=await browser.newPage({viewport:{width:1440,height:1100}});
    const errors=[];page.on('pageerror',e=>errors.push(e.message));
    const mutations=[];page.on('request',request=>{if(request.method()==='POST')mutations.push({path:new URL(request.url()).pathname,body:request.postDataJSON()});});
    const profileRemove=name=>page.locator(`[data-remove-profile="${name}"]`);
    await page.goto(url);
    await page.locator('#nodes').getByText('test-gateway',{exact:true}).waitFor();
    assert.equal(await page.locator('#host-count').textContent(),'4');
    assert.equal(await page.locator('#drift-count').textContent(),'5');
    const visibleNodes=()=>page.locator('#nodes tr').evaluateAll(rows=>rows.map(row=>
      row.cells[0].querySelector('strong').textContent + row.querySelector('[data-remove-node]').dataset.removeNode));
    await page.locator('#filter-kind').selectOption('tbs');
    assert.deepEqual(await visibleNodes(),['tbstest-tbs']);
    await page.locator('#filter-kind').selectOption('services');
    assert.equal((await visibleNodes()).some(text=>text.includes('test-tbs')),false);
    await page.locator('#filter-service').selectOption('packet-core');
    assert.deepEqual(await visibleNodes(),['packet-coretest-backend-fails']);
    await page.locator('#filter-service').selectOption('');
    await page.locator('#filter-kind').selectOption('all');
    await page.locator('#filter-online').selectOption('offline');
    assert.deepEqual(await visibleNodes(),['media-switchtest-offline']);
    await page.locator('#filter-online').selectOption('online');
    assert.equal((await visibleNodes()).length,4);
    await page.locator('#filter').fill('zz-test');
    assert.deepEqual(await visibleNodes(),['call-controlzz-test-backend']);
    await page.evaluate(()=>refresh());
    assert.equal(await page.locator('#filter').inputValue(),'zz-test');
    assert.deepEqual(await visibleNodes(),['call-controlzz-test-backend']);
    await page.locator('#filter').fill('');
    await page.locator('#filter-online').selectOption('all');
    await page.locator('#scan').click();
    await page.getByText('Suchlauf gestartet. Die Übersicht aktualisiert sich automatisch.').waitFor();
    // A fresh controller can create its first site template without a local file.
    await page.waitForFunction(()=>!document.querySelector('#template-new').disabled);
    assert.equal((await (await page.request.get(url+'/api/v1/status')).json()).has_template,false);
    await page.locator('#template-new').click();
    await page.locator('#template-rf').getByText(/418 MHz.*408 MHz/).waitFor();
    await page.locator('#template-secondary-carrier').fill('720');
    await page.locator('#template-preview-button').click();
    await page.locator('#template-error').getByText(/müssen verschieden/).waitFor();
    await page.locator('#template-cancel').click();
    assert.equal((await (await page.request.get(url+'/api/v1/status')).json()).has_template,false,'Cancel must not save a template');
    await page.locator('#template-new').click();
    await page.locator('#template-rf').getByText(/418 MHz.*408 MHz/).waitFor();
    await page.locator('#template-toml summary').click();
    const createdTemplate=await page.locator('#template-preview').textContent();
    assert.match(createdTemplate,/config_version = "0.6"/);
    assert.match(createdTemplate,/tx_center_freq = 418012500/);
    await page.screenshot({path:'/tmp/netcore-deployment-template-desktop.png',fullPage:true});
    await page.setViewportSize({width:390,height:844});
    assert.ok(await page.locator('#template-dialog').evaluate(el=>el.getBoundingClientRect().right<=innerWidth));
    assert.equal(await page.evaluate(()=>document.documentElement.scrollWidth>innerWidth),false);
    await page.screenshot({path:'/tmp/netcore-deployment-template-mobile.png',fullPage:true});
    await page.setViewportSize({width:1440,height:1100});
    await page.locator('#template-save').click();
    await page.locator('#template-dialog').waitFor({state:'hidden'});
    await page.getByText('Standort-Template erstellt. Du kannst jetzt ein TBS-Profil anlegen.',{exact:true}).waitFor();
    assert.equal(await page.locator('#template-new').isDisabled(),true);
    await page.reload();
    await page.locator('#nodes').getByText('test-gateway',{exact:true}).waitFor();
    assert.equal((await (await page.request.get(url+'/api/v1/status')).json()).has_template,true);
    assert.equal(mutations.filter(request=>request.path==='/api/v1/template/create').length,1);
    assert.equal(mutations.filter(request=>request.path==='/api/v1/template').length,0);
    await page.locator('#profile-form [name=name]').fill('TBS-02');
    await page.locator('#profile-form [name=issi]').fill('4010002');
    await page.locator('#profile-form button').click();
    await page.locator('#profile-list').getByText('TBS-02 · Bootstrap ↓').waitFor();
    const createdConfig=await (await page.request.get(url+'/api/v1/profiles/TBS-02/config')).text();
    assert.match(createdConfig,/tx_freq = 418000000/);
    assert.match(createdConfig,/rx_freq = 408000000/);
    assert.match(createdConfig,/node_id = "TBS-02"/);
    assert.equal(await page.locator('#image-form [name=ref]').count(),0);
    assert.equal(await page.locator('#image-ref').count(),0);
    assert.match(await page.locator('.image-version-note').textContent(),/main.*automatisch/);
    assert.equal(await page.locator('#deploy-ref').inputValue(),'fixture-deployment-ref');
    assert.equal(await page.locator('#settings-ref').inputValue(),'fixture-deployment-ref');
    await page.locator('#target-service').selectOption('node-gateway');
    await page.locator('#deploy-form button[type=submit]').click();
    await page.locator('#plan').waitFor({state:'visible'});
    assert.match(await page.locator('#plan-text').textContent(),/test-gateway/);
    await page.locator('#execute').click();
    await page.locator('#job-list').getByText('Erfolgreich',{exact:true}).waitFor({timeout:15000});
    assert.match(await page.locator('#log').textContent(),/simulated installer/);
    // One reviewed main revision, one user submission, real child HTTP jobs.
    // A failed host must not block an independent backend or the TBS target.
    const bulkRuns=()=>mutations.filter(request=>request.path==='/api/v1/updates/run');
    await page.locator('#bulk-plan').click();
    await page.locator('#bulk-preview').waitFor({state:'visible'});
    const planned=mutations.find(request=>request.path==='/api/v1/updates/plan');
    assert.deepEqual(planned.body,{});
    assert.equal(await page.locator('#bulk-commit').textContent(),'b'.repeat(40));
    assert.match(await page.locator('#bulk-plan-summary').textContent(),/Dienste.*Hosts/);
    assert.match(await page.locator('#bulk-targets').textContent(),/test-backend-fails/);
    assert.match(await page.locator('#bulk-targets').textContent(),/zz-test-backend/);
    await page.locator('#bulk-cancel').click();
    await page.locator('#bulk-preview').waitFor({state:'hidden'});
    assert.equal(bulkRuns().length,0,'Cancelling the preview must not deploy');
    await page.locator('#filter-kind').selectOption('tbs');
    await page.locator('#bulk-plan').click();
    await page.locator('#bulk-preview').waitFor({state:'visible'});
    assert.match(await page.locator('#bulk-targets').textContent(),/packet-core/,'View filters must not silently narrow the deployment');
    await page.locator('#bulk-execute').click();
    await page.waitForFunction(()=>document.querySelector('#bulk-status-body')?.textContent.includes('Läuft'),null,{timeout:15000});
    assert.equal(bulkRuns().length,1);
    assert.equal(bulkRuns()[0].body.confirm_restart,true);
    assert.ok(bulkRuns()[0].body.plan_id);
    const beforeBulkRefresh=mutations.length;
    await page.evaluate(()=>refresh());
    assert.equal(mutations.length,beforeBulkRefresh,'Polling must not queue further deployments');
    await page.waitForFunction(()=>document.querySelector('#bulk-status')?.textContent.includes('Teilerfolg'),null,{timeout:30000});
    const batches=await (await page.request.get(url+'/api/v1/updates')).json();
    assert.equal(batches.length,1);
    const batch=batches[0];
    assert.equal(batch.status,'partial');
    assert.equal(batch.commit,'b'.repeat(40));
    assert.equal(batch.jobs.find(job=>job.request.node_id==='test-backend-fails').status,'failed');
    for(const node of ['test-gateway','zz-test-backend','test-tbs']){
      const job=batch.jobs.find(job=>job.request.node_id===node);
      assert.ok(job,`Missing child job for ${node}`);
      assert.equal(job.status,'succeeded',`${node} must proceed despite the independent failure`);
      assert.equal(job.request.commit,batch.commit,'Every target must use the single reviewed main revision');
    }
    assert.ok(batch.skipped.some(target=>target.node_id==='test-offline'));
    const compact=await page.locator('#bulk-status-body').textContent();
    assert.match(compact,/test-backend-fails/);
    assert.match(compact,/Fehlgeschlagen/);
    assert.match(compact,/zz-test-backend/);
    assert.match(compact,/Erfolgreich/);
    assert.equal(bulkRuns().length,1,'Partial failure must not retry the whole batch');
    assert.equal(batch.jobs.some(job=>Object.hasOwn(job,'log')),false,'Compact batch polling must omit full logs');
    const failedChild=batch.jobs.find(job=>job.request.node_id==='test-backend-fails');
    const logRequest=page.waitForRequest(request=>request.method()==='GET' && new URL(request.url()).pathname==='/api/v1/jobs/'+failedChild.id);
    await page.locator(`[data-bulk-job="${failedChild.id}"]`).click();
    await logRequest;
    await page.waitForFunction(()=>document.querySelector('#log').textContent.includes('simulated backend failure'));
    assert.match(await page.locator('#log-title').textContent(),/test-backend-fails/);
    // Render a lost remote reply distinctly from a confirmed installer failure.
    // This route changes only public status metadata, never deployment requests.
    const uncertainBatch=structuredClone(batch);
    uncertainBatch.jobs.find(job=>job.request.node_id==='test-backend-fails').result={
      remote_uncertain:true,error:'Remote-Auftrag nach verlorener Antwort unklar'};
    uncertainBatch.counts.uncertain=1;
    const updatesStatusRoute=/\/api\/v1\/updates(?:\/[^/]+)?$/;
    await page.route(updatesStatusRoute,route=>{
      if(route.request().method()!=='GET')return route.continue();
      const detail=new URL(route.request().url()).pathname!== '/api/v1/updates';
      return route.fulfill({status:200,contentType:'application/json',body:JSON.stringify(detail?uncertainBatch:[uncertainBatch])});
    });
    await page.evaluate(()=>refresh());
    assert.match(await page.locator('#bulk-status-body').textContent(),/Ungeklärt/);
    assert.equal(bulkRuns().length,1,'Uncertain remote status must never trigger an automatic retry POST');
    await page.unroute(updatesStatusRoute);
    await page.evaluate(()=>refresh());
    await page.locator('#filter-kind').selectOption('all');
    // Removing an entry changes the Discovery inventory, not installed roles.
    // A cancelled confirmation cannot submit; tombstones survive page reload.
    const peerRemovals=()=>mutations.filter(request=>request.path==='/api/v1/peers/remove');
    page.once('dialog',dialog=>dialog.dismiss());
    await page.locator('[data-remove-node="test-offline"]').click();
    assert.equal(peerRemovals().length,0);
    assert.ok((await visibleNodes()).some(text=>text.includes('test-offline')));
    page.once('dialog',dialog=>dialog.accept());
    await page.locator('[data-remove-node="test-offline"]').click();
    await page.waitForFunction(()=>!document.querySelector('[data-remove-node="test-offline"]'));
    assert.equal(peerRemovals().length,1);
    await page.evaluate(()=>refresh());
    await page.reload();
    await page.locator('#nodes').getByText('test-gateway',{exact:true}).waitFor();
    assert.equal((await visibleNodes()).some(text=>text.includes('test-offline')),false);
    const forgotten=await (await page.request.get(url+'/api/v1/status')).json();
    assert.ok(forgotten.forgotten.some(node=>node.node_id==='test-offline'));
    await page.locator('#removed-nodes summary').click();
    assert.equal(await page.locator('[data-restore-node="test-offline"]').count(),1);
    page.once('dialog',dialog=>dialog.dismiss());
    await page.locator('[data-restore-node="test-offline"]').click();
    assert.equal(mutations.filter(request=>request.path==='/api/v1/peers/restore').length,0);
    page.once('dialog',dialog=>dialog.accept());
    await page.locator('[data-restore-node="test-offline"]').click();
    await page.waitForFunction(()=>!document.querySelector('[data-restore-node="test-offline"]'));
    assert.equal((await visibleNodes()).some(text=>text.includes('test-offline')),false,'An offline restore must await a fresh manifest');
    // A reachable peer reappears only through its actual agent manifest.
    page.once('dialog',dialog=>dialog.accept());
    await page.locator('[data-remove-node="zz-test-backend"]').click();
    await page.waitForFunction(()=>!document.querySelector('[data-remove-node="zz-test-backend"]'));
    await page.reload();
    await page.locator('#removed-nodes summary').click();
    page.once('dialog',dialog=>dialog.accept());
    await page.locator('[data-restore-node="zz-test-backend"]').click();
    await page.locator('#nodes').getByText('zz-test-backend',{exact:true}).waitFor({timeout:15000});
    assert.equal(mutations.filter(request=>request.path==='/api/v1/peers/restore').length,2);
    // Reload intentionally clears form selection. Restore it explicitly before
    // continuing the existing image-login/private-credential tests below.
    await page.locator('#target-profile').selectOption('TBS-02');
    await page.locator('#image-profile').selectOption('TBS-02');
    assert.equal(await page.locator('#image-controller').inputValue(),'http://10.0.1.50:8320');
    const imageBuildCount=()=>mutations.filter(request=>request.path==='/api/v1/images/build').length;
    const sshKey='ssh-ed25519 '+Buffer.concat([Buffer.from([0,0,0,11]),Buffer.from('ssh-ed25519'),Buffer.from([0,0,0,32]),Buffer.alloc(32,1)]).toString('base64')+' browser-fixture';
    assert.equal(await page.locator('#image-desktop').inputValue(),'kde','new image UI should offer KDE by default');
    assert.deepEqual(await page.locator('#image-desktop option').evaluateAll(options=>options.map(o=>o.value)),['kde','xfce','gnome','pios','none']);
    assert.equal(await page.locator('#image-vpn-credentials').isVisible(),false);
    assert.equal(await page.locator('#image-vpn-username').isDisabled(),true);
    await page.locator('#image-form [name=ssh_key]').fill(sshKey);
    await page.locator('#image-build').click();
    assert.equal(imageBuildCount(),0,'KDE requires a login password even when an SSH key was supplied');
    assert.equal(await page.locator('#image-password').evaluate(input=>input.required&&input.validity.valueMissing),true);
    assert.match(await page.locator('#image-password-note').textContent(),/KDE/);
    for(const [desktop,label] of [['xfce','XFCE'],['gnome','GNOME'],['pios','Raspberry Pi OS Desktop']]){
      await page.locator('#image-desktop').selectOption(desktop);
      assert.match(await page.locator('#image-password-note').textContent(),new RegExp(label));
      await page.locator('#image-build').click();
      assert.equal(imageBuildCount(),0,`${label} also needs a graphical login password`);
      assert.equal(await page.locator('#image-password').evaluate(input=>input.required&&input.validity.valueMissing),true);
    }
    await page.locator('#image-desktop').selectOption('kde');
    await page.locator('#image-form [name=password]').fill('fixture-only-password');
    await page.locator('details.image-options').filter({has:page.locator('#image-vpn')}).locator('summary').click();
    await page.locator('#image-vpn').setInputFiles({name:'incomplete.ovpn',mimeType:'text/plain',buffer:Buffer.from('client\ndev tun\nremote vpn.fixture.invalid 1194\n<auth-user-pass>\nold-user\n</auth-user-pass>\n')});
    await page.waitForFunction(()=>document.querySelector('#image-vpn-username').required);
    await page.locator('#image-build').click();
    assert.equal(imageBuildCount(),0,'An inline username without password also needs the VPN fields');
    const privateVpn='client\ndev tun\nremote vpn.fixture.invalid 1194\nauth-user-pass\n<ca>\nFIXTURECERT\n</ca>\n';
    await page.locator('#image-vpn').setInputFiles({name:'station.ovpn',mimeType:'text/plain',buffer:Buffer.from(privateVpn)});
    await page.waitForFunction(()=>document.querySelector('#image-vpn-username').required);
    assert.equal(await page.locator('#image-vpn-credentials').isVisible(),true);
    await page.locator('#image-build').click();
    assert.equal(imageBuildCount(),0,'VPN auth-user-pass needs credentials before submission');
    await page.locator('#image-vpn-username').fill('fixture-vpn-user');
    await page.locator('#image-vpn-password').fill('fixture-vpn-private');
    await page.locator('#image-build').click();
    await page.locator('#image-artifacts').getByText('Download bereit',{exact:true}).waitFor({timeout:15000});
    // An artifact can be published before the job/log snapshot finishes. Wait
    // for the real selected job's completion instead of asserting that race.
    await page.waitForFunction(()=>
      document.querySelector('#image-job-list .job.selected .badge')?.textContent==='Erfolgreich' &&
      document.querySelector('#image-log').textContent.includes('simulated image builder'),
      null,{timeout:15000});
    assert.match(await page.locator('#image-log').textContent(),/simulated image builder/);
    assert.match(await page.locator('#image-job-list').textContent(),/bbbbbbbb/);
    assert.match(await page.locator('#image-artifacts').textContent(),/bbbbbbbb/);
    assert.equal(mutations.filter(request=>request.path==='/api/v1/template').length,0,'Image build must work without importing a template');
    const imageSubmission=mutations.find(request=>request.path==='/api/v1/images/build');
    assert.equal(imageSubmission.body.profile,'TBS-02');
    assert.equal(imageSubmission.body.desktop,'kde');
    assert.equal(imageSubmission.body.vpn_config,privateVpn);
    assert.equal(imageSubmission.body.vpn_username,'fixture-vpn-user');
    assert.equal(imageSubmission.body.vpn_password,'fixture-vpn-private');
    assert.equal(imageSubmission.body.sip_fallback_config,'');
    assert.equal(Object.hasOwn(imageSubmission.body,'ref'),false,'Pi image UI must not submit a branch, tag or commit');
    assert.equal(await page.locator('#image-form [name=password]').inputValue(),'');
    assert.equal(await page.locator('#image-vpn-username').inputValue(),'');
    assert.equal(await page.locator('#image-vpn-password').inputValue(),'');
    assert.equal(await page.locator('#image-vpn-credentials').isVisible(),false);
    const [download]=await Promise.all([page.waitForEvent('download'),page.locator('#image-artifacts').getByText('Image ↓',{exact:true}).click()]);
    assert.equal(download.suggestedFilename(),'fixture.img.xz');
    const readable=await download.createReadStream();const chunks=[];for await(const chunk of readable)chunks.push(chunk);
    assert.equal(Buffer.concat(chunks).toString(),'fixture image transport only');
    assert.match(await page.locator('#image-artifacts').textContent(),/KDE Plasma/);
    assert.match(await page.locator('#image-artifacts').textContent(),/Einrichtung erforderlich/);
    // The Lite image accepts SSH-key-only access. Private SIP TOML reaches the
    // offline image request, but public job metadata and the rendered UI omit it.
    await page.locator('#image-desktop').selectOption('none');
    assert.equal(await page.locator('#image-password').evaluate(input=>input.required),false);
    assert.match(await page.locator('#image-password-note').textContent(),/SSH-Public-Key/);
    const privateSip='[service]\nnode_id="TBS-02"\n[central]\nhost="sip-core.fixture.invalid"\nusername="fixture-central-user"\npassword="fixture-central-private"\n[fallback_pbx]\nhost="pbx.fixture.invalid"\nusername="fixture-pbx-user"\nauth_username="fixture-pbx-auth"\npassword="fixture-pbx-private"\n';
    await page.locator('#image-sip').setInputFiles({name:'station-sip.toml',mimeType:'text/plain',buffer:Buffer.from(privateSip)});
    assert.equal(await page.locator('#image-form [name=password]').inputValue(),'');
    await page.locator('#image-build').click();
    await page.waitForFunction(()=>document.querySelectorAll('#image-artifacts .image-card').length===2,{},{timeout:15000});
    // Terminal status also means the private request has been scrubbed before
    // reading public API metadata and asserting credential cleanup below.
    await page.waitForFunction(()=>
      document.querySelector('#image-job-list .job.selected .badge')?.textContent==='Erfolgreich',
      null,{timeout:15000});
    const imageSubmissions=mutations.filter(request=>request.path==='/api/v1/images/build');
    assert.equal(imageSubmissions.length,2);
    assert.equal(imageSubmissions[1].body.desktop,'none');
    assert.equal(imageSubmissions[1].body.password,'');
    assert.equal(imageSubmissions[1].body.ssh_key,sshKey);
    assert.equal(imageSubmissions[1].body.sip_fallback_config,privateSip);
    assert.equal(Object.hasOwn(imageSubmissions[1].body,'ref'),false);
    assert.equal(await page.locator('#image-sip').evaluate(input=>input.files.length),0,'successful import must clear the private file selection');
    const imageState=await (await page.request.get(url+'/api/v1/images')).json();
    assert.equal(imageState.artifacts.length,2);
    const desktopJob=imageState.jobs.find(job=>job.request.desktop==='kde');
    const liteJob=imageState.jobs.find(job=>job.request.desktop==='none');
    assert.ok(desktopJob&&liteJob,'both desktop modes must survive the real API roundtrip');
    assert.equal(desktopJob.request.sip_setup,'setup_required');
    assert.equal(liteJob.request.sip_setup,'configured');
    const publicJson=JSON.stringify(imageState);
    for(const privateValue of ['fixture-only-password','fixture-central-private','fixture-pbx-private','fixture-central-user','fixture-pbx-auth',privateSip,'sip_fallback_config','fixture-vpn-user','fixture-vpn-private',privateVpn,'vpn_password','vpn_username']){
      assert.equal(publicJson.includes(privateValue),false,'public image jobs must not expose private credentials or configuration');
      assert.equal((await page.locator('body').textContent()).includes(privateValue),false,'rendered job/artifact/log must not contain private input');
    }
    assert.match(await page.locator('#image-artifacts').textContent(),/Ohne Desktop \(Lite\)/);
    assert.match(await page.locator('#image-artifacts').textContent(),/SIP eingerichtet/);
    // Cancellation, busy/error handling, pending protection and actual successful
    // removal all use the saved-profile UI; successful requests reach the real API.
    await page.locator('#profile-form [name=name]').fill('TBS-03');
    await page.locator('#profile-form [name=issi]').fill('4010003');
    await page.locator('#profile-form button').click();
    await page.locator('#profile-list').getByText('TBS-03 · Bootstrap ↓').waitFor();
    await page.locator('#target-profile').selectOption('TBS-03');
    assert.equal(await page.locator('#image-profile').inputValue(),'TBS-03');
    const removeCount=()=>mutations.filter(request=>request.path==='/api/v1/profiles/remove').length;
    let confirmation='';
    page.once('dialog',async dialog=>{confirmation=dialog.message();await dialog.dismiss();});
    await profileRemove('TBS-03').click();
    assert.match(confirmation,/gespeicherte TBS-Profil „TBS-03“/);
    assert.match(confirmation,/installierte Basisstationen, Images und Buildprotokolle bleiben erhalten/);
    assert.equal(removeCount(),0,'Cancelled confirmation must not send a deletion request');
    assert.equal(await page.locator('.saved-profile').count(),2);
    const conflict='TBS-Profil wird von einem laufenden Auftrag verwendet.';
    await page.route('**/api/v1/profiles/remove',route=>route.fulfill({status:409,contentType:'application/json',body:JSON.stringify({error:conflict})}));
    page.once('dialog',dialog=>dialog.accept());
    await profileRemove('TBS-03').click();
    await page.getByText(conflict,{exact:true}).waitFor();
    assert.equal(await page.locator('#notice.error').isVisible(),true);
    assert.equal(await page.locator('.saved-profile').count(),2,'Failed deletion must retain the profile list');
    assert.equal(await page.locator('#target-profile').inputValue(),'TBS-03');
    assert.equal(await page.locator('#image-profile').inputValue(),'TBS-03');
    assert.equal(await profileRemove('TBS-03').isEnabled(),true,'Failed request must restore the delete button');
    await page.unroute('**/api/v1/profiles/remove');
    let releaseRemoval;
    const pendingRemoval=new Promise(resolve=>{releaseRemoval=resolve;});
    let removalArrived;
    const awaitingRemoval=new Promise(resolve=>{removalArrived=resolve;});
    await page.route('**/api/v1/profiles/remove',async route=>{removalArrived();await pendingRemoval;await route.continue();});
    page.once('dialog',dialog=>dialog.accept());
    await profileRemove('TBS-03').click();
    await page.getByText('Wird gelöscht …',{exact:true}).waitFor();
    await awaitingRemoval;
    assert.equal(await page.locator('[data-remove-profile]:disabled').count(),2);
    const pendingCount=removeCount();
    const refreshMutationCount=mutations.length;
    await page.evaluate(()=>refresh());
    assert.equal(mutations.length,refreshMutationCount,'Refreshing the UI must not create mutation or Git-check requests');
    await page.locator('[data-remove-profile="TBS-03"]').dispatchEvent('click');
    assert.equal(removeCount(),pendingCount,'Repeated pending click must not submit twice');
    assert.equal(await page.locator('[data-remove-profile]:disabled').count(),2,'Polling must retain pending-button protection');
    releaseRemoval();
    await page.getByText('TBS-Profil „TBS-03“ gelöscht.',{exact:true}).waitFor();
    await page.unroute('**/api/v1/profiles/remove');
    assert.equal(await page.locator('.saved-profile').count(),1);
    assert.equal(await page.locator('#image-profile').inputValue(),'','Deleting the selected profile must not silently select another image profile');
    assert.equal(await page.locator('#target-profile').inputValue(),'');
    assert.equal(await page.locator('#image-hostname').inputValue(),'');
    assert.equal(await page.locator('#image-build').isDisabled(),true);
    await page.waitForFunction(()=>!document.querySelector('[data-remove-profile="TBS-02"]').disabled);
    assert.equal(await page.locator('#image-artifacts .image-card').count(),2,'Profile removal must preserve existing images');
    assert.match(await page.locator('#job-list').textContent(),/Erfolgreich/);
    for(const [name,applied] of [['TBS-04',true],['TBS-05',null]]){
      await page.locator('#profile-form [name=name]').fill(name);
      await page.locator('#profile-form [name=issi]').fill(name==='TBS-04'?'4010004':'4010005');
      await page.locator('#profile-form button').click();
      await page.locator('#profile-list').getByText(`${name} · Bootstrap ↓`,{exact:true}).waitFor();
      await page.locator('#target-profile').selectOption(name);
      const before=removeCount();
      const persistenceWarning='Profiländerung wurde gespeichert, Persistenzbestätigung fehlgeschlagen.';
      await page.route('**/api/v1/profiles/remove',async route=>{
        // Commit the actual server-side removal before simulating a response whose
        // final directory-fsync failed. GET must therefore see the removed profile.
        const response=await route.fetch();
        assert.equal(response.status(),200);
        await route.fulfill({response,status:503,contentType:'application/json',body:JSON.stringify({error:persistenceWarning,profile_change_applied:applied,durability_uncertain:true})});
      });
      page.once('dialog',dialog=>dialog.accept());
      await profileRemove(name).click();
      await page.waitForFunction(()=>document.querySelector('#notice.error')?.textContent.includes('Persistenzbestätigung fehlgeschlagen'));
      await page.waitForFunction(()=>!document.querySelector('[data-remove-profile="TBS-02"]').disabled);
      await page.unroute('**/api/v1/profiles/remove');
      assert.equal(removeCount(),before+1,'Persistence uncertainty must not resubmit the deletion');
      assert.equal(await page.locator('.saved-profile').count(),1);
      assert.equal(await page.locator('#target-profile').inputValue(),'');
      assert.equal(await page.locator('#image-profile').inputValue(),'');
      assert.equal(await page.locator('#image-hostname').inputValue(),'');
      assert.equal(await page.locator('#image-build').isDisabled(),true);
      const current=await (await page.request.get(url+'/api/v1/profiles')).json();
      assert.deepEqual(current.map(profile=>profile.name),['TBS-02'],'UI must reconcile with the actual persisted profile state after a 503');
      if(applied===true)assert.match(await page.locator('#notice').textContent(),/gelöscht, aber die dauerhafte Speicherung konnte nicht bestätigt werden/);
      assert.equal(await page.locator('#image-artifacts .image-card').count(),2);
    }
    // Import remains available after the complete create/profile/image workflow.
    await page.locator('#template').setInputFiles({name:'site.toml',mimeType:'text/plain',buffer:Buffer.from(createdTemplate)});
    await page.getByText('Standort-Template gespeichert.',{exact:true}).waitFor();
    assert.equal(mutations.filter(request=>request.path==='/api/v1/template').length,1);
    assert.equal(await page.evaluate(()=>document.documentElement.scrollWidth>innerWidth),false);
    await page.screenshot({path:'/tmp/netcore-deployment-desktop.png',fullPage:true});
    await page.setViewportSize({width:390,height:844});
    await page.screenshot({path:'/tmp/netcore-deployment-mobile.png',fullPage:true});
    assert.equal(await page.evaluate(()=>document.documentElement.scrollWidth>innerWidth),false);
    assert.deepEqual(errors,[]);
    await page.locator('#target-profile').selectOption('TBS-02');
    await page.locator('#image-profile').selectOption('TBS-02');
    page.once('dialog',dialog=>dialog.accept());
    await profileRemove('TBS-02').click();
    await page.getByText('TBS-Profil „TBS-02“ gelöscht.',{exact:true}).waitFor();
    await page.getByText('Noch keine TBS-Profile gespeichert.',{exact:true}).waitFor();
    assert.equal(await page.locator('#profile-list a').getAttribute('href'),'#profile-form');
    assert.equal(await page.locator('#target-profile').inputValue(),'');
    assert.equal(await page.locator('#image-profile').inputValue(),'');
    assert.equal(await page.locator('#image-profile option').count(),1);
    assert.equal(await page.locator('#image-build').isDisabled(),true);
    assert.equal(await page.locator('#image-artifacts .image-card').count(),2,'Removing the last profile must not remove previously built images');
    await page.screenshot({path:'/tmp/netcore-deployment-empty-profiles-mobile.png',fullPage:true});
    for(let remaining=2;remaining>0;remaining--){
      page.once('dialog',dialog=>dialog.accept());
      await page.locator('#image-artifacts').getByText('Löschen',{exact:true}).first().click();
      await page.waitForFunction(count=>document.querySelectorAll('#image-artifacts .image-card').length===count,remaining-1);
    }
    await page.getByText('Image gelöscht. Das Buildprotokoll bleibt erhalten.').waitFor();
    assert.equal(await page.locator('#image-artifacts .image-card').count(),0);
    assert.deepEqual(errors,[]);
    console.log('PASS: real WebUI, Discovery type/role/online/text filters, bulk preview/cancel/live/partial continuation, one commit and no retry, persistent inventory removal/restore, new site template without import, invalid RF/cancel/persistence/preview, TBS wizard and image build followed by existing import, remote job, all desktop choices/login gating, automatic-main KDE/Lite submission, private SIP/VPN credentials roundtrip/download/cleanup, profile cancel/error/pending/deletion/503 reconciliation/empty state, preserved images, desktop and mobile');
  } finally {if(browser)await browser.close();fixture.kill('SIGTERM');}
})().catch(e=>{console.error(e);process.exitCode=1;});


