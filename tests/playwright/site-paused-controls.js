const assert = require("node:assert/strict");
const { spawn } = require("node:child_process");
const { createHash } = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");
const { setTimeout: wait } = require("node:timers/promises");
const { chromium } = require("playwright");
const { ROOT, showcaseEntries } = require("../../site/stage");

const PORT = Number(process.env.SITE_PAUSED_CONTROLS_PORT || 8774);
const ORIGIN = `http://127.0.0.1:${PORT}`;
const OUTPUT = path.join(ROOT, "target/site-paused-controls");
const INTERACTIVE_ID = "throughput-console";
const PAUSED_FRAME = 30;
const REPEATED_INPUTS = 25;
const CONSOLE_HEIGHT = 540;
const NEEDLE_SCAN_ROW = 328;
const STANDBY_NEEDLE_LIMIT = 0.3;
const ENGAGED_NEEDLE_FLOOR = 0.6;

async function waitForServer() {
  const deadline = Date.now() + 20000;
  while (Date.now() < deadline) {
    try {
      const response = await fetch(`${ORIGIN}/showcase.html`, {
        signal: AbortSignal.timeout(1000),
      });
      await response.arrayBuffer();
      if (response.ok) return;
    } catch {}
    await wait(100);
  }
  throw new Error(`site server did not start on port ${PORT}`);
}

function sha256(bytes) {
  return createHash("sha256").update(bytes).digest("hex");
}

async function capture(page, label, evidence) {
  const snapshot = await page.evaluate(({ id, rowFraction }) => {
    const timeline = window.__RIVE_SHOWCASE_TIMELINES.get(id);
    const canvas = document.querySelector(`[data-showcase-id="${id}"] canvas.scene`);
    const context = canvas.getContext("2d");
    const row = Math.round(canvas.height * rowFraction);
    const { data } = context.getImageData(0, row, canvas.width, 1);
    let sum = 0;
    let count = 0;
    for (let column = 0; column < canvas.width; column += 1) {
      const offset = column * 4;
      if (data[offset] > 200 && data[offset + 1] > 200 && data[offset + 2] > 200) {
        sum += column;
        count += 1;
      }
    }
    return {
      frame: timeline.currentFrame,
      playing: timeline.isPlaying,
      inputs: timeline.readInputs(),
      needle: count ? sum / count / canvas.width : null,
      png: canvas.toDataURL("image/png"),
    };
  }, { id: INTERACTIVE_ID, rowFraction: NEEDLE_SCAN_ROW / CONSOLE_HEIGHT });
  const bytes = Buffer.from(snapshot.png.split(",")[1], "base64");
  fs.writeFileSync(path.join(OUTPUT, `${label}.png`), bytes);
  delete snapshot.png;
  snapshot.sha256 = sha256(bytes);
  evidence.frames[label] = snapshot;
  return snapshot;
}

async function setInput(page, name, value) {
  await page.evaluate(async ({ id, name, value }) => {
    await window.__RIVE_SHOWCASE_TIMELINES.get(id).setInput(name, value);
  }, { id: INTERACTIVE_ID, name, value });
}

(async () => {
  fs.mkdirSync(OUTPUT, { recursive: true });
  const entry = showcaseEntries().find((item) => item.id === INTERACTIVE_ID);
  assert.ok(entry, "interactive showcase must be present");
  const load = entry.controls.find((control) => control.kind === "range").input;
  const armed = entry.controls.find((control) => control.kind === "toggle").input;
  const reset = entry.controls.find((control) => control.kind === "trigger").input;
  const evidence = {
    artifact: entry.artifact,
    artifactSha256: sha256(fs.readFileSync(path.join(ROOT, entry.artifact))),
    source: entry.source,
    sourceSha256: sha256(fs.readFileSync(path.join(ROOT, entry.source))),
    repeatedInputs: REPEATED_INPUTS,
    frames: {},
    errors: [],
  };
  const server = spawn("node", ["site/serve.js"], {
    cwd: ROOT,
    env: { ...process.env, SITE_PORT: String(PORT) },
    stdio: "ignore",
  });
  let browser;
  let page;
  try {
    await waitForServer();
    browser = await chromium.launch();
    page = await browser.newPage({
      viewport: { width: 1280, height: 1000 },
      reducedMotion: "reduce",
    });
    page.on("pageerror", (error) => evidence.errors.push(error.message));
    page.on("console", (message) => {
      if (message.type() === "error") evidence.errors.push(message.text());
    });
    await page.goto(`${ORIGIN}/showcase.html`, { waitUntil: "load" });
    await page.waitForFunction((id) =>
      document.querySelector(`[data-showcase-id="${id}"]`)?.dataset.playbackReady === "true",
    INTERACTIVE_ID, { timeout: 20000, polling: 50 });
    await page.evaluate(async ({ id, frame }) => {
      const timeline = window.__RIVE_SHOWCASE_TIMELINES.get(id);
      await timeline.pause();
      await timeline.seekToFrame(frame);
    }, { id: INTERACTIVE_ID, frame: PAUSED_FRAME });

    const before = await capture(page, "before", evidence);
    assert.equal(before.frame, PAUSED_FRAME);
    assert.equal(before.playing, false);
    for (let index = 0; index < REPEATED_INPUTS; index += 1) {
      await setInput(page, load, before.inputs[load]);
    }
    const repeated = await capture(page, "after-repeated-inputs", evidence);
    assert.equal(repeated.frame, PAUSED_FRAME);
    assert.equal(repeated.sha256, before.sha256,
      "separately dispatched paused input tasks must not move animated regions");

    await setInput(page, load, 100);
    await setInput(page, armed, true);
    const engaged = await capture(page, "armed-high", evidence);
    assert.ok(engaged.needle !== null && engaged.needle > ENGAGED_NEEDLE_FLOOR,
      `paused arm/load controls did not move the needle: ${engaged.needle}`);
    assert.equal(engaged.frame, PAUSED_FRAME);

    await setInput(page, load, 0);
    const low = await capture(page, "armed-low", evidence);
    assert.ok(low.needle !== null && low.needle < STANDBY_NEEDLE_LIMIT,
      `paused slider did not lower the needle: ${low.needle}`);
    assert.equal(low.frame, PAUSED_FRAME);

    await setInput(page, load, 100);
    await setInput(page, armed, false);
    const beforeReset = await capture(page, "before-reset", evidence);
    assert.ok(beforeReset.needle !== null && beforeReset.needle > ENGAGED_NEEDLE_FLOOR);
    await page.evaluate(async ({ id, reset }) => {
      await window.__RIVE_SHOWCASE_TIMELINES.get(id).fireTrigger(reset);
    }, { id: INTERACTIVE_ID, reset });
    const resetFrame = await capture(page, "reset", evidence);
    assert.ok(resetFrame.needle !== null && resetFrame.needle < STANDBY_NEEDLE_LIMIT,
      `paused trigger did not reset the needle: ${resetFrame.needle}`);
    assert.equal(resetFrame.frame, PAUSED_FRAME);

    await page.evaluate(async ({ id, frame }) => {
      await window.__RIVE_SHOWCASE_TIMELINES.get(id).seekToFrame(frame);
    }, { id: INTERACTIVE_ID, frame: PAUSED_FRAME });
    const sameFrame = await capture(page, "same-frame-seek", evidence);
    assert.equal(sameFrame.sha256, resetFrame.sha256);
    await page.evaluate(async ({ id, frame }) => {
      await window.__RIVE_SHOWCASE_TIMELINES.get(id).seekToFrame(frame);
    }, { id: INTERACTIVE_ID, frame: PAUSED_FRAME + 1 });
    const forward = await capture(page, "forward-one-frame", evidence);
    assert.equal(forward.frame, PAUSED_FRAME + 1);
    assert.notEqual(forward.sha256, sameFrame.sha256,
      "explicit forward seek must still advance the animated regions");
    assert.deepEqual(evidence.errors, []);
    evidence.passed = true;
    process.stdout.write("Paused-control playback passed: frozen pixels, live inputs/triggers, and consistent seeking\n");
  } catch (error) {
    evidence.passed = false;
    evidence.failure = error.stack || String(error);
    if (page && !page.isClosed()) {
      try {
        evidence.page = await page.evaluate(() => ({
          url: location.href,
          timelines: Array.from(window.__RIVE_SHOWCASE_TIMELINES?.keys() || []),
          cards: Array.from(document.querySelectorAll("[data-showcase-id]")).map((card) => ({
            id: card.dataset.showcaseId,
            ready: card.dataset.playbackReady,
            playing: card.dataset.playing,
          })),
        }));
        await page.screenshot({ path: path.join(OUTPUT, "failure.png") });
      } catch (captureError) {
        evidence.captureFailure = String(captureError);
      }
    }
    process.stderr.write(`${evidence.failure}\n`);
    process.exitCode = 1;
  } finally {
    fs.writeFileSync(path.join(OUTPUT, "evidence.json"), `${JSON.stringify(evidence, null, 2)}\n`);
    if (browser) await browser.close();
    server.kill("SIGTERM");
  }
})().catch((error) => {
  process.stderr.write(`${error.stack || error}\n`);
  process.exitCode = 1;
});
