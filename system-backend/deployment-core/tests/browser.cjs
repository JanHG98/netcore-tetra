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
    assert.equal(await page.locator('#host-count').textContent(),'1');
    assert.equal(await page.locator('#drift-count').textContent(),'1');
    await page.locator('#scan').click();
    await page.getByText('Suchlauf gestartet. Die Übersicht aktualisiert sich automatisch.').waitFor();
    await page.locator('#template').setInputFiles({name:'site.toml',mimeType:'text/plain',buffer:Buffer.from('[net_info]\nmcc=901\nmnc=1510\n[cell_info]\nlocation_area=1\ncolour_code=1\n[phy_io]\nbackend="SoapySdr"\n')});
    await page.getByText('Standort-Template gespeichert.').waitFor();
    await page.locator('#profile-form [name=name]').fill('TBS-02');
    await page.locator('#profile-form [name=issi]').fill('4010002');
    await page.locator('#profile-form button').click();
    await page.locator('#profile-list').getByText('TBS-02 · Bootstrap ↓').waitFor();
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
    assert.match(await page.locator('#image-log').textContent(),/simulated image builder/);
    assert.match(await page.locator('#image-job-list').textContent(),/bbbbbbbb/);
    assert.match(await page.locator('#image-artifacts').textContent(),/bbbbbbbb/);
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
    console.log('PASS: real WebUI, discovery, TBS wizard, remote job, all desktop choices/login gating, automatic-main KDE/Lite submission, private SIP/VPN credentials roundtrip/download/cleanup, profile cancel/error/pending/deletion/503 reconciliation/empty state, preserved images, desktop and mobile');
  } finally {if(browser)await browser.close();fixture.kill('SIGTERM');}
})().catch(e=>{console.error(e);process.exitCode=1;});


