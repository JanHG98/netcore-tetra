const {chromium} = require('playwright');
const {spawn} = require('node:child_process');
const assert = require('node:assert/strict');
const {mkdtemp, mkdir, readFile, rm} = require('node:fs/promises');
const os = require('node:os');
const path = require('node:path');
const readline = require('node:readline');

const root = path.resolve(__dirname, '..');
function fixture(script, args = []) {
  const child = spawn('python3', [path.join(root, script), ...args], {cwd:root, stdio:['ignore','pipe','inherit']});
  const url = new Promise((resolve, reject) => {
    const timer = setTimeout(() => reject(new Error('Fixture startup timed out')), 20000);
    child.once('exit', code => {clearTimeout(timer);reject(new Error('Fixture exited: ' + code));});
    readline.createInterface({input:child.stdout}).on('line', line => {
      const match = line.match(/^(?:FIXTURE )?(http:\/\/127\.0\.0\.1:\d+)$/);
      if (match) {clearTimeout(timer);resolve(match[1]);}
    });
  });
  return {child,url};
}

(async () => {
 const sip = fixture('tools/test_sip_station_management.py', ['--serve']);
 const deploy = fixture('system-backend/deployment-core/tests/serve_ui.py');
 const temporary = await mkdtemp(path.join(os.tmpdir(), 'netcore-sip-ui-'));
 const output = path.join(root, 'target/service-ui-preview/sip-stations');
 let browser;
 try {
  const [sipUrl,deployUrl] = await Promise.all([sip.url,deploy.url]);
  browser = await chromium.launch({headless:true,executablePath:process.env.CHROMIUM_EXECUTABLE_PATH});
  const context = await browser.newContext({viewport:{width:1440,height:1050},acceptDownloads:true});
  const page = await context.newPage(), errors=[];
  page.on('pageerror', error => errors.push(error.message));
  page.on('dialog', dialog => dialog.accept());
  await page.goto(sipUrl);
  await page.locator('#station-new').click();
  await page.locator('#station-node').fill('TBS-SIP-UI');
  await page.locator('#station-central-host').fill('10.0.1.125');
  await page.locator('#station-pbx-user').fill('dedicated-ui-pbx');
  await page.locator('#station-pbx-auth').fill('dedicated-ui-auth');
  await page.locator('#station-pbx-password').fill('browser-private-secret');
  await page.locator('#station-save').click();
  await page.locator('#station-dialog').waitFor({state:'hidden'});
  const row = () => page.locator('#tbsRows tr').filter({has:page.getByText('TBS-SIP-UI',{exact:true})});
  await row().getByText('TOML bereit',{exact:true}).waitFor();
  assert.match(await page.locator('#station-message').textContent(), /Asterisk neu geladen/);
  await row().getByRole('button',{name:'Bearbeiten',exact:true}).click();
  assert.equal(await page.locator('#station-password').inputValue(), '');
  assert.equal(await page.locator('#station-pbx-password').inputValue(), '');
  assert.match(await page.locator('#station-pbx-password').getAttribute('placeholder'), /vorhanden/);
  await page.locator('#station-aliases').fill('UI-Alias');
  await page.locator('#station-save').click();
  await page.locator('#station-dialog').waitFor({state:'hidden'});
  let listed = await (await page.request.get(sipUrl+'/api/v1/tbs')).json();
  assert.deepEqual(listed.find(v=>v.node_id==='TBS-SIP-UI').aliases,['UI-Alias']);
  assert.ok(!JSON.stringify(listed).includes('browser-private-secret'));
  // An open editor must not overwrite a newer station revision.
  await row().getByRole('button',{name:'Bearbeiten',exact:true}).click();
  const current = listed.find(v=>v.node_id==='TBS-SIP-UI');
  assert.equal((await page.request.put(sipUrl+'/api/v1/tbs/TBS-SIP-UI',{data:{revision:current.revision,aliases:['Newer-Alias']}})).status(),200);
  await page.locator('#station-aliases').fill('Would-overwrite');
  await page.locator('#station-save').click();
  await page.locator('#station-error').getByText(/inzwischen/).waitFor();
  await page.locator('#station-cancel').click();
  await page.evaluate(()=>refresh());
  const downloadEvent = page.waitForEvent('download');
  await row().getByRole('button',{name:'SIP-TOML ↓',exact:true}).click();
  const download = await downloadEvent;
  assert.equal(download.suggestedFilename(),'TBS-SIP-UI-sip-fallback.toml');
  const filename=path.join(temporary,download.suggestedFilename());
  await download.saveAs(filename);
  const toml=await readFile(filename,'utf8');
  assert.ok(toml.includes('browser-private-secret'));
  assert.ok(toml.includes('dedicated-ui-pbx'));
  assert.ok(!toml.includes('shared-trunk-secret'));
  await mkdir(output,{recursive:true});
  await page.screenshot({path:path.join(output,'stations-light.png'),fullPage:true});
  await page.locator('.nc-theme-toggle').click();
  await row().getByRole('button',{name:'Bearbeiten',exact:true}).click();
  await page.screenshot({path:path.join(output,'editor-dark.png'),fullPage:true});
  await page.setViewportSize({width:390,height:844});
  assert.ok(await page.evaluate(()=>document.documentElement.scrollWidth<=innerWidth+2),'No mobile page overflow');
  assert.ok(await page.locator('#station-dialog').evaluate(el=>el.getBoundingClientRect().right<=innerWidth),'Editor fits mobile viewport');
  await page.screenshot({path:path.join(output,'editor-mobile.png'),fullPage:true});
  await page.locator('#station-cancel').click();
  await page.setViewportSize({width:1440,height:1050});
  // Import the actual downloaded file through the unchanged Deploy image form,
  // real controller/worker API and their real image/SIP validators.
  const deployment = await context.newPage();
  deployment.on('pageerror',error=>errors.push(error.message));
  await deployment.goto(deployUrl+'/#images');
  await deployment.locator('#template').setInputFiles({name:'site.toml',mimeType:'text/plain',buffer:Buffer.from('[net_info]\nmcc=901\nmnc=1510\n[cell_info]\nlocation_area=1\ncolour_code=1\n[phy_io]\nbackend="SoapySdr"\n')});
  await deployment.getByText('Standort-Template gespeichert.',{exact:true}).waitFor();
  await deployment.locator('#profile-form [name=name]').fill('TBS-SIP-UI');
  await deployment.locator('#profile-form [name=issi]').fill('4010007');
  await deployment.locator('#profile-form button').click();
  await deployment.locator('#profile-list').getByText('TBS-SIP-UI · Bootstrap ↓').waitFor();
  await deployment.locator('#image-profile').selectOption('TBS-SIP-UI');
  await deployment.locator('#image-password').fill('ci-private-os-password');
  await deployment.locator('details').filter({has:deployment.locator('#image-sip')}).locator('summary').click();
  await deployment.locator('#image-sip').setInputFiles(filename);
  const submission=deployment.waitForResponse(r=>r.url().endsWith('/api/v1/images/build')&&r.request().method()==='POST');
  await deployment.locator('#image-build').click();
  const response=await submission;
  assert.ok(response.ok(),await response.text());
  const job=await response.json();
  assert.equal(job.request.sip_setup,'configured');
  await deployment.locator('#image-artifacts').getByText('SIP eingerichtet',{exact:true}).waitFor({timeout:15000});
  const images=await (await deployment.request.get(deployUrl+'/api/v1/images')).json();
  assert.equal(images.artifacts.find(a=>a.profile.name==='TBS-SIP-UI').sip_setup,'configured');
  assert.ok(!JSON.stringify(images).includes('browser-private-secret'));
  await row().getByRole('button',{name:'Löschen',exact:true}).click();
  await row().waitFor({state:'detached'});
  await page.reload();
  await page.locator('#station-new').waitFor();
  listed=await (await page.request.get(sipUrl+'/api/v1/tbs')).json();
  assert.ok(!listed.some(v=>v.node_id==='TBS-SIP-UI'));
  assert.deepEqual(errors,[]);
  console.log('PASS: real SIP HTTP CRUD, stale-form guard, private download, light/dark/mobile UI and downloaded TOML imported via real Deploy controller/worker validation. Image build and Asterisk CLI are fixtures.');
 } finally {
  if(browser)await browser.close();
  sip.child.kill('SIGTERM');deploy.child.kill('SIGTERM');
  await rm(temporary,{recursive:true,force:true});
 }
})().catch(error=>{console.error(error);process.exitCode=1;});
