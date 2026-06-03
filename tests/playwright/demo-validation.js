const { chromium } = require('playwright');

(async () => {
  const errors = [];
  const browser = await chromium.launch();
  const page = await browser.newPage();

  page.on('console', msg => {
    if (msg.type() === 'error') {
      errors.push(msg.text());
    }
  });
  page.on('pageerror', err => {
    errors.push(err.message);
  });

  await page.goto('http://localhost:3021/', { waitUntil: 'networkidle', timeout: 60000 });
  await page.waitForTimeout(10000);

  await browser.close();

  if (errors.length > 0) {
    console.error(`Demo validation failed: ${errors.length} console error(s) found:`);
    [...new Set(errors)].forEach(e => console.error('  -', e.substring(0, 400)));
    process.exit(1);
  }

  console.log('Demo validation passed: 0 console errors');
})();
