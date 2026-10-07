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
    await page.locator('#target-service').selectOption('node-gateway');
    await page.locator('#deploy-form button[type=submit]').click();
    await page.locator('#plan').waitFor({state:'visible'});
    assert.match(await page.locator('#plan-text').textContent(),/test-gateway/);
    await page.locator('#execute').click();
    await page.locator('#job-list').getByText('Erfolgreich',{exact:true}).waitFor({timeout:15000});
    assert.match(await page.locator('#log').textContent(),/simulated installer/);
    assert.equal(await page.locator('#image-controller').inputValue(),'http://10.0.1.50:8320');
    await page.locator('#image-form [name=password]').fill('fixture-only-password');
    await page.locator('#image-build').click();
    await page.locator('#image-artifacts').getByText('Download bereit',{exact:true}).waitFor({timeout:15000});
    assert.match(await page.locator('#image-log').textContent(),/simulated image builder/);
    assert.equal(await page.locator('#image-form [name=password]').inputValue(),'');
    const [download]=await Promise.all([page.waitForEvent('download'),page.locator('#image-artifacts').getByText('Image ↓',{exact:true}).click()]);
    assert.equal(download.suggestedFilename(),'fixture.img.xz');
    const readable=await download.createReadStream();const chunks=[];for await(const chunk of readable)chunks.push(chunk);
    assert.equal(Buffer.concat(chunks).toString(),'fixture image transport only');
    assert.equal(await page.evaluate(()=>document.documentElement.scrollWidth>innerWidth),false);
    await page.screenshot({path:'/tmp/netcore-deployment-desktop.png',fullPage:true});
    await page.setViewportSize({width:390,height:844});
    await page.screenshot({path:'/tmp/netcore-deployment-mobile.png',fullPage:true});
    assert.equal(await page.evaluate(()=>document.documentElement.scrollWidth>innerWidth),false);
    assert.deepEqual(errors,[]);
    page.once('dialog',dialog=>dialog.accept());
    await page.locator('#image-artifacts').getByText('Löschen',{exact:true}).click();
    await page.getByText('Image gelöscht. Das Buildprotokoll bleibt erhalten.').waitFor();
    assert.equal(await page.locator('#image-artifacts .image-card').count(),0);
    console.log('PASS: real WebUI, discovery, TBS wizard, remote job, image submission/download/deletion, desktop and mobile');
  } finally {if(browser)await browser.close();fixture.kill('SIGTERM');}
})().catch(e=>{console.error(e);process.exitCode=1;});
