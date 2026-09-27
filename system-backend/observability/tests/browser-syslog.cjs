const {chromium} = require('playwright');
const assert = require('node:assert/strict');
(async () => {
  const browser = await chromium.launch({headless: true, args: ['--no-sandbox'],
    ...(process.env.PLAYWRIGHT_CHROMIUM_EXECUTABLE ? {executablePath: process.env.PLAYWRIGHT_CHROMIUM_EXECUTABLE} : {})});
  try {
    const page = await browser.newPage({viewport: {width: 1440, height: 1000}});
    const errors = [];
    page.on('pageerror', error => errors.push(String(error)));
    await page.goto(process.env.OBSERVABILITY_TEST_URL || 'http://127.0.0.1:18210');
    await page.locator('nav button[data-page="targets"]').click();
    await page.waitForFunction(() => document.querySelector('#discoveryStatus').textContent.includes('Discovery deaktiviert'));
    assert.match(await page.locator('#targetsTable').innerText(), /10\.0\.1\.179:8080/);
    await page.locator('nav button[data-page="logs"]').click();
    await page.waitForFunction(() => document.querySelector('#syslogStatus').textContent.includes('share test offline'));
    assert.match(await page.locator('#syslogStatus').innerText(), /Unarchiviert verworfen: 2 Segmente/);
    await page.locator('#logContains').fill('native-preview-test');
    await page.locator('#logs button').filter({hasText: 'Suchen'}).click();
    await page.waitForFunction(() => document.querySelector('#logsTable').textContent.includes('native-preview-test'));
    assert.equal(await page.locator('#logsTable tr').count(), 1);
    if (process.env.OBSERVABILITY_SCREENSHOT) {
      await page.screenshot({path: process.env.OBSERVABILITY_SCREENSHOT, fullPage: true});
    }
    assert.deepEqual(errors, []);
    console.log('Native UI: discovery status, archive warning, loss counter and log search OK');
  } finally {
    await browser.close();
  }
})().catch(error => {console.error(error); process.exit(1)});
