const assert = require("node:assert/strict");
const fs = require("node:fs/promises");
const os = require("node:os");
const path = require("node:path");
const { test } = require("node:test");
const { chromium } = require("playwright");
const { retainShowcaseFailure } = require("./showcase-diagnostics");

const launchOptions = process.env.RIVE_CHROME
  ? { executablePath: process.env.RIVE_CHROME }
  : {};

test("startup failure retains stage, page errors, per-card frame/paint state and screenshot", async () => {
  const directory = await fs.mkdtemp(path.join(os.tmpdir(), "showcase-diagnostics-"));
  const browser = await chromium.launch(launchOptions);
  try {
    const page = await browser.newPage({ viewport: { width: 640, height: 480 } });
    await page.setContent(`
      <article class="card" data-showcase-id="painted" data-playback-ready="true" data-playing="false">
        <canvas class="scene" width="4" height="3" aria-label="painted scene"></canvas>
      </article>
      <article class="card" data-showcase-id="waiting" data-playback-ready="false" data-playing="false">
        <canvas class="scene" width="4" height="3"></canvas>
      </article>
    `);
    await page.evaluate(() => {
      document.querySelector("canvas").getContext("2d").fillRect(0, 0, 2, 1);
      window.__RIVE_SHOWCASE_TIMELINES = new Map([
        ["painted", { currentFrame: 30, isPlaying: false, reducedMotion: true }],
      ]);
    });
    const failure = new Error("first paint timed out");
    const errors = ["runtime load error", "request failed: /missing.riv"];
    const report = await retainShowcaseFailure({
      page, directory, stage: "desktop:first-paint", error: failure, errors,
    });
    const saved = JSON.parse(await fs.readFile(path.join(directory, "failure.json"), "utf8"));
    assert.deepEqual(saved, report);
    assert.equal(saved.stage, "desktop:first-paint");
    assert.equal(saved.error.message, failure.message);
    assert.equal(saved.error.stack, failure.stack);
    assert.deepEqual(saved.errors, errors);
    assert.deepEqual(saved.viewport, { width: 640, height: 480 });
    assert.deepEqual(saved.cards.map(({ id, ready, playing, frame, painted }) =>
      ({ id, ready, playing, frame, painted })), [
      { id: "painted", ready: "true", playing: "false", frame: 30, painted: 2 },
      { id: "waiting", ready: "false", playing: "false", frame: null, painted: 0 },
    ]);
    assert.equal(saved.cards[0].width, 4);
    assert.equal(saved.cards[0].height, 3);
    assert.equal(saved.cards[0].reducedMotion, true);
    assert.deepEqual(saved.diagnosticErrors, []);
    assert.equal(saved.screenshot, "failure.png");
    const png = await fs.readFile(path.join(directory, saved.screenshot));
    assert.deepEqual([...png.subarray(0, 8)], [137, 80, 78, 71, 13, 10, 26, 10]);
  } finally {
    await browser.close();
    await fs.rm(directory, { recursive: true, force: true });
  }
});

test("a closed page cannot replace the original failure or prevent its JSON report", async () => {
  const directory = await fs.mkdtemp(path.join(os.tmpdir(), "showcase-diagnostics-"));
  const browser = await chromium.launch(launchOptions);
  try {
    const page = await browser.newPage();
    await page.close();
    const report = await retainShowcaseFailure({
      page, directory, stage: "lifecycle:ready", error: new Error("original readiness timeout"), errors: [],
    });
    const saved = JSON.parse(await fs.readFile(path.join(directory, "failure.json"), "utf8"));
    assert.deepEqual(saved, report);
    assert.equal(saved.error.message, "original readiness timeout");
    assert.equal(saved.stage, "lifecycle:ready");
    assert.equal(saved.screenshot, null);
    assert.deepEqual(saved.cards, []);
    assert.ok(saved.diagnosticErrors.some((error) => error.operation === "cards"));
    assert.ok(saved.diagnosticErrors.some((error) => error.operation === "screenshot"));
  } finally {
    await browser.close();
    await fs.rm(directory, { recursive: true, force: true });
  }
});

test("an unreadable canvas does not hide other cards or look like an unpainted canvas", async () => {
  const directory = await fs.mkdtemp(path.join(os.tmpdir(), "showcase-diagnostics-"));
  const browser = await chromium.launch(launchOptions);
  try {
    const page = await browser.newPage();
    await page.setContent(`
      <article class="card" data-showcase-id="unreadable" data-playback-ready="true">
        <canvas class="scene" width="4" height="3"></canvas>
      </article>
      <article class="card" data-showcase-id="healthy" data-playback-ready="true">
        <canvas class="scene" width="4" height="3"></canvas>
      </article>
    `);
    await page.evaluate(() => {
      const contexts = [...document.querySelectorAll("canvas")].map((canvas) => canvas.getContext("2d"));
      contexts[0].getImageData = () => { throw new DOMException("unreadable canvas", "SecurityError"); };
      contexts[1].fillRect(0, 0, 3, 2);
    });
    const report = await retainShowcaseFailure({
      page, directory, stage: "desktop:first-paint", error: new Error("paint probe failed"), errors: [],
    });
    assert.equal(report.cards.length, 2);
    assert.equal(report.cards[0].id, "unreadable");
    assert.equal(report.cards[0].painted, null);
    assert.match(report.cards[0].paintError, /unreadable canvas/);
    assert.equal(report.cards[1].painted, 6);
    assert.equal(report.cards[1].paintError, null);
  } finally {
    await browser.close();
    await fs.rm(directory, { recursive: true, force: true });
  }
});

test("a pre-browser failure survives an unwritable artifact directory", async () => {
  const root = await fs.mkdtemp(path.join(os.tmpdir(), "showcase-diagnostics-"));
  const directory = path.join(root, "not-a-directory");
  try {
    await fs.writeFile(directory, "occupied");
    const report = await retainShowcaseFailure({
      page: null, directory, stage: "server", error: new Error("server startup failed"), errors: [],
    });
    assert.equal(report.error.message, "server startup failed");
    assert.equal(report.stage, "server");
    assert.equal(report.screenshot, null);
    assert.ok(report.diagnosticErrors.some((error) => error.operation === "directory"));
    assert.ok(report.diagnosticErrors.some((error) => error.operation === "report"));
  } finally {
    await fs.rm(root, { recursive: true, force: true });
  }
});

test("a stalled page probe is bounded and still retains the original error", { timeout: 15000 }, async () => {
  const directory = await fs.mkdtemp(path.join(os.tmpdir(), "showcase-diagnostics-"));
  try {
    const page = {
      url: () => "http://127.0.0.1/showcase.html",
      viewportSize: () => ({ width: 1280, height: 1000 }),
      evaluate: () => new Promise(() => {}),
      screenshot: async () => { throw new Error("renderer unavailable"); },
    };
    const report = await retainShowcaseFailure({
      page, directory, stage: "desktop:ready", error: new Error("original timeout"), errors: [],
    });
    assert.equal(report.error.message, "original timeout");
    assert.ok(report.diagnosticErrors.some((error) =>
      error.operation === "cards" && /timed out/.test(error.message)));
    assert.equal(JSON.parse(await fs.readFile(path.join(directory, "failure.json"), "utf8")).error.message,
      "original timeout");
  } finally {
    await fs.rm(directory, { recursive: true, force: true });
  }
});
