const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const { spawn, execFileSync } = require('node:child_process');
const { chromium } = require('playwright');

const ROOT = path.resolve(__dirname, '../..');
const OUTPUT = path.join(ROOT, 'target/rive-page-evidence');
const PORT = Number(process.env.SITE_PORT || 8792);
const URL = `http://127.0.0.1:${PORT}/`;
const manifest = require('../../site/scenes/manifest.json');
const delay = ms => new Promise(resolve => setTimeout(resolve, ms));
const launch = process.env.RIVE_CHROME ? { executablePath: process.env.RIVE_CHROME } : {};
const errors = [], observations = [];
let browser, server;

async function ready(page) {
  await page.waitForFunction(() => document.querySelector('#page-surface')?.dataset.state === 'ready', null, { timeout: 22000 });
}
async function value(page) {
  return Number(await page.locator('[data-control="morph"]').getAttribute('aria-valuenow'));
}
async function screenshot(page, name) {
  await page.screenshot({ path: path.join(OUTPUT, `${name}.png`), fullPage: true });
}
async function pixels(page) {
  return page.locator('#rive-page').evaluate(canvas => {
    const data = canvas.getContext('2d').getImageData(0, 0, canvas.width, canvas.height).data;
    let painted = 0, coral = 0, ink = 0;
    for (let i = 0; i < data.length; i += 4) {
      if (data[i + 3]) painted += 1;
      if (data[i] > 160 && data[i + 1] < 160 && data[i + 2] < 130) coral += 1;
      if (data[i] < 70 && data[i + 1] < 70 && data[i + 2] < 70) ink += 1;
    }
    return { painted, coral, ink, total: canvas.width * canvas.height };
  });
}
async function newPage(options = {}) {
  const page = await browser.newPage(options);
  page.on('pageerror', error => errors.push(error.message));
  page.on('console', message => { if (message.type() === 'error') errors.push(message.text()); });
  await page.goto(URL, { waitUntil: 'load' });
  return page;
}
async function main() {
  fs.mkdirSync(OUTPUT, { recursive: true });
  server = spawn(process.execPath, ['site/serve.js'], { cwd: ROOT, env: { ...process.env, SITE_PORT: String(PORT) }, stdio: 'ignore' });
  for (let attempt = 0; attempt < 100; attempt += 1) {
    try { if ((await fetch(URL)).ok) break; } catch (_) { }
    await delay(100);
  }
  browser = await chromium.launch(launch);
  for (const [name, width, height] of [['desktop', 1280, 1040], ['tablet', 780, 1144], ['mobile', 390, 844]]) {
    const page = await newPage({ viewport: { width, height }, reducedMotion: 'reduce' });
    try {
      await ready(page);
      const rendered = await pixels(page);
      assert.ok(rendered.painted > rendered.total * 0.95, `${name}: page is blank`);
      assert.ok(rendered.coral > 5000, `${name}: sculpture or heading is missing`);
      assert.ok(rendered.ink > 20000, `${name}: stage or text is missing`);
      assert.equal(await page.locator('#page-surface').getAttribute('data-playing'), 'false');
      assert.equal(await page.locator('#page-surface').getAttribute('data-layout'), name);
      const overflow = await page.evaluate(() => document.documentElement.scrollWidth - innerWidth);
      assert.ok(overflow <= 1, `${name}: horizontal overflow`);
      const frozen = await page.locator('#rive-page').screenshot();
      await delay(200);
      assert.deepEqual(await page.locator('#rive-page').screenshot(), frozen, `${name}: reduced motion moved`);
      const pixelsBeforeResize = await page.locator('#rive-page').evaluate(canvas => canvas.toDataURL());
      await page.setViewportSize({ width, height: height + 80 });
      await delay(150);
      assert.equal(await page.locator('#rive-page').evaluate(canvas => canvas.toDataURL()), pixelsBeforeResize, `${name}: resizing advanced paused artwork`);
      await page.setViewportSize({ width, height });
      await delay(150);
      await screenshot(page, name);
      await page.locator('[data-control="weave"]').click();
      await page.waitForFunction(() => document.querySelector('[role="slider"]').getAttribute('aria-valuenow') === '1');
      await delay(100);
      assert.notDeepEqual(await page.locator('#rive-page').screenshot(), frozen, `${name}: preset does not change Rive pixels`);
      await screenshot(page, `${name}-weave`);
      await page.locator('[data-control="stack"]').click();
      await delay(100);
      await screenshot(page, `${name}-stack`);
      const slider = page.locator('[role="slider"]');
      await slider.focus();
      await page.keyboard.press('Home');
      assert.equal(await value(page), 0);
      await page.keyboard.press('End');
      assert.equal(await value(page), 2);
      await page.keyboard.press('ArrowLeft');
      assert.equal(await value(page), 1.99);
      const config = manifest.layouts.find(layout => layout.id === name);
      const rail = config.controls.find(control => control.id === 'morph').rail;
      const bounds = await page.locator('#page-surface').boundingBox();
      const scale = bounds.width / config.width;
      await slider.scrollIntoViewIfNeeded();
      const scrolled = await page.locator('#page-surface').boundingBox();
      const y = scrolled.y + (rail.y + 1.5) * scale;
      await page.mouse.move(scrolled.x + rail.x * scale, y);
      await page.mouse.down();
      await page.mouse.move(scrolled.x + (rail.x + rail.width * 0.375) * scale, y, { steps: 12 });
      await page.mouse.up();
      await delay(150);
      assert.ok(Math.abs(await value(page) - 0.75) < 0.02, `${name}: drag and authored thumb disagree`);
      const pausedFrame = await page.locator('#page-surface').getAttribute('data-frame');
      await delay(200);
      assert.equal(await page.locator('#page-surface').getAttribute('data-frame'), pausedFrame);
      assert.equal(await page.locator('[data-control="source"]').getAttribute('href'), config.source);
      assert.equal(await page.locator('[data-control="download"]').getAttribute('href'), config.artifact);
      assert.equal(await page.locator('[data-control="showcase"]').getAttribute('href'), 'showcase.html');
      assert.equal(await page.locator('[data-control="lab"]').getAttribute('href'), 'lab.html');
      observations.push({ layout: name, viewport: [width, height], ...rendered, blendAfterDrag: await value(page), frozenFrame: pausedFrame });
    } finally {
      await screenshot(page, `${name}-last`);
      await page.close();
    }
  }

  const page = await newPage({ viewport: { width: 1200, height: 940 } });
  try {
    await ready(page);
    await page.waitForFunction(() => Number(document.querySelector('#page-surface').dataset.frame) > 3);
    const orbitBounds = await page.locator('[data-control="weave"]').boundingBox();
    await page.locator('#page-controls').evaluate(element => { element.style.display = 'none'; });
    await page.mouse.click(orbitBounds.x + orbitBounds.width / 2, orbitBounds.y + orbitBounds.height / 2);
    await page.waitForFunction(() => document.querySelector('#page-surface').dataset.morph === '1', null, { timeout: 5000 });
    await screenshot(page, 'native-listener');
    await page.locator('#page-controls').evaluate(element => { element.style.removeProperty('display'); });
    await page.getByRole('button', { name: 'Pause motion' }).click();
    await delay(100);
    const held = await page.locator('#page-surface').getAttribute('data-frame');
    await delay(250);
    assert.equal(await page.locator('#page-surface').getAttribute('data-frame'), held);
    await page.locator('[data-control="replay"]').click();
    await page.waitForFunction(() => document.querySelector('#page-surface').dataset.frame === '0');
    assert.equal(await value(page), 1, 'replay lost selected shape');
    for (const width of [320, 619, 620, 979, 980, 1440]) {
      await page.setViewportSize({ width, height: 940 });
      await ready(page);
      await page.waitForFunction(expected => document.querySelector('#page-surface').dataset.layout === expected, width < 620 ? 'mobile' : width < 980 ? 'tablet' : 'desktop');
      assert.equal(await value(page), 1, 'resize lost shape');
      assert.equal(await page.locator('#page-surface').getAttribute('data-playing'), 'false', 'resize resumed paused motion');
      assert.ok(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth + 1));
    }
    await page.getByRole('button', { name: 'Play motion' }).click();
    await page.emulateMedia({ reducedMotion: 'reduce' });
    await page.waitForFunction(() => document.querySelector('#page-surface').dataset.playing === 'false');
    await page.evaluate(() => dispatchEvent(new PageTransitionEvent('pagehide', { persisted: true })));
    await page.evaluate(() => dispatchEvent(new PageTransitionEvent('pageshow', { persisted: true })));
    assert.equal(await page.locator('#page-surface').getAttribute('data-playing'), 'false', 'bfcache ignored reduced motion');
    observations.push({ nativePresetListener: true, resizeRetainsInput: true, reducedMotionChange: true, replayPreservesInput: true });
  } finally { await screenshot(page, 'lifecycle-last'); await page.close(); }

  const touch = await newPage({ viewport: { width: 390, height: 844 }, isMobile: true, hasTouch: true, deviceScaleFactor: 2, reducedMotion: 'reduce' });
  await ready(touch);
  await touch.locator('[data-control="stack"]').tap();
  assert.equal(await value(touch), 2);
  await screenshot(touch, 'mobile-touch-2x');
  await touch.close();

  for (const blocked of ['**/scenes/mobile.riv', '**/assets/rive.wasm', '**/scenes/manifest.json']) {
    const failed = await browser.newPage({ viewport: { width: 390, height: 844 } });
    await failed.route(blocked, route => route.abort());
    await failed.goto(URL);
    await failed.waitForFunction(() => document.querySelector('#page-surface').dataset.state === 'error', null, { timeout: 22000 });
    const link = failed.locator('#page-fallback a[href="text.html"]');
    assert.ok(await link.isVisible());
    assert.ok((await link.boundingBox()).y < 844, 'failure navigation is below the fold');
    await screenshot(failed, blocked.includes('wasm') ? 'failed-wasm' : blocked.includes('manifest') ? 'failed-manifest' : 'failed-artifact');
    await failed.close();
  }
  const nojs = await browser.newPage({ javaScriptEnabled: false, viewport: { width: 390, height: 844 } });
  await nojs.goto(URL);
  assert.ok(await nojs.locator('#page-fallback a[href="text.html"]').isVisible());
  await nojs.close();
  assert.deepEqual(errors, [], 'normal-load console errors');
}

main().then(() => {
  console.log('Rive page browser contracts passed.');
}).catch(error => {
  console.error(error.stack);
  process.exitCode = 1;
}).finally(async () => {
  fs.mkdirSync(OUTPUT, { recursive: true });
  fs.writeFileSync(path.join(OUTPUT, 'observation.json'), JSON.stringify({ sourceCommit: execFileSync('git', ['rev-parse', 'HEAD'], { cwd: ROOT, encoding: 'utf8' }).trim(), browser: browser?.version(), passed: !process.exitCode, observations, errors }, null, 2) + '\n');
  await browser?.close();
  server?.kill('SIGTERM');
});
