const assert = require("node:assert/strict");
const { execFileSync, spawn } = require("node:child_process");
const { createHash } = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");
const { setTimeout: wait } = require("node:timers/promises");
const { chromium } = require("playwright");

const ROOT = process.cwd();
const OUTPUT = path.join(ROOT, "target/readiness-investigation");
const ORIGIN = "http://127.0.0.1:8779";
const INTERACTIVE_ID = "throughput-console";
const ROUNDS = 4;
const sha256 = (bytes) => createHash("sha256").update(bytes).digest("hex");
const gitFile = (ref, file) => execFileSync("git", ["show", `${ref}:${file}`]);

(async () => {
  fs.mkdirSync(OUTPUT, { recursive: true });
  const variants = {
    historical: gitFile("c1c8fef906b71e0ef716bfd5ea0ccccf5ba5db4b", "site/playback.js"),
    base: gitFile("602ca840f750e966ce85d116cea48ee682d172ba", "site/playback.js"),
    current: fs.readFileSync("site/playback.js"),
  };
  const report = {
    head: execFileSync("git", ["rev-parse", "HEAD"], { encoding: "utf8" }).trim(),
    node: process.version,
    playwright: require("playwright/package.json").version,
    rounds: ROUNDS,
    timeout: 20000,
    polling: "raf",
    viewport: { width: 1280, height: 1000 },
    reducedMotion: "reduce",
    hashes: {},
    results: [],
  };
  for (const [name, bytes] of Object.entries(variants)) {
    fs.writeFileSync(path.join(OUTPUT, `${name}-playback.js`), bytes);
    report.hashes[name] = sha256(bytes);
  }
  fs.writeFileSync(path.join(OUTPUT, "historical-paused-controls.js"),
    gitFile("c1c8fef906b71e0ef716bfd5ea0ccccf5ba5db4b", "tests/playwright/site-paused-controls.js"));
  for (const file of ["assets/rive.js", "assets/rive.wasm", "examples/authoring/interactive-console.v0.riv", "site/showcase.js"]) {
    report.hashes[file] = sha256(fs.readFileSync(file));
  }
  const server = spawn(process.execPath, ["site/serve.js"], {
    env: { ...process.env, SITE_PORT: "8779" },
    stdio: ["ignore", "pipe", "pipe"],
  });
  const serverLog = fs.createWriteStream(path.join(OUTPUT, "server.log"));
  server.stdout.pipe(serverLog);
  server.stderr.pipe(serverLog);
  let browser;
  try {
    const deadline = Date.now() + 20000;
    let serverReady = false;
    while (Date.now() < deadline) {
      try {
        const response = await fetch(`${ORIGIN}/showcase.html`, { signal: AbortSignal.timeout(1000) });
        await response.arrayBuffer();
        if (response.ok) { serverReady = true; break; }
      } catch {}
      await wait(100);
    }
    assert.ok(serverReady, "staged site server must start");
    browser = await chromium.launch();
    report.chromium = browser.version();
    for (let round = 0; round < ROUNDS; round += 1) {
      const names = round % 2 === 0 ? ["historical", "base", "current"] : ["current", "base", "historical"];
      for (const name of names) {
        const label = `${name}-${round + 1}`;
        const directory = path.join(OUTPUT, label);
        fs.mkdirSync(directory, { recursive: true });
        fs.writeFileSync(path.join(ROOT, "target/site/playback.js"), variants[name]);
        const context = await browser.newContext({ viewport: report.viewport, reducedMotion: report.reducedMotion });
        const page = await context.newPage();
        const result = { label, errors: [], requestsFailed: [], responsesFailed: [] };
        page.on("pageerror", (error) => result.errors.push(error.message));
        page.on("console", (message) => { if (message.type() === "error") result.errors.push(message.text()); });
        page.on("requestfailed", (request) => result.requestsFailed.push({ url: request.url(), error: request.failure() }));
        page.on("response", (response) => { if (response.status() >= 400) result.responsesFailed.push({ url: response.url(), status: response.status() }); });
        try {
          await page.goto(`${ORIGIN}/showcase.html`, { waitUntil: "load" });
          const started = Date.now();
          try {
            await page.waitForFunction((id) =>
              document.querySelector(`[data-showcase-id="${id}"]`)?.dataset.playbackReady === "true",
            INTERACTIVE_ID, { timeout: 20000 });
            result.ready = true;
          } catch (error) {
            result.ready = false;
            result.failure = error.stack || String(error);
          }
          result.elapsedMs = Date.now() - started;
          result.snapshot = await page.evaluate(() => ({
            url: location.href,
            timelines: Array.from(window.__RIVE_SHOWCASE_TIMELINES?.keys() || []),
            cards: Array.from(document.querySelectorAll("[data-showcase-id]")).map((card) => ({
              id: card.dataset.showcaseId,
              ready: card.dataset.playbackReady,
              playing: card.dataset.playing,
              frame: window.__RIVE_SHOWCASE_TIMELINES?.get(card.dataset.showcaseId)?.currentFrame,
              message: card.querySelector(".status")?.textContent,
            })),
          }));
          await page.screenshot({ path: path.join(directory, "page.png"), timeout: 5000 });
        } catch (error) {
          result.probeFailure = error.stack || String(error);
        } finally {
          fs.writeFileSync(path.join(directory, "evidence.json"), JSON.stringify(result, null, 2) + "\n");
          report.results.push(result);
          fs.writeFileSync(path.join(OUTPUT, "evidence.json"), JSON.stringify(report, null, 2) + "\n");
          console.log(JSON.stringify(result));
          await context.close();
        }
      }
    }
    assert.ok(report.results.filter((result) => result.label.startsWith("current-")).every((result) =>
      result.ready && !result.probeFailure && result.errors.length === 0), "current readiness must pass every fixed-count run");
  } finally {
    fs.writeFileSync(path.join(OUTPUT, "evidence.json"), JSON.stringify(report, null, 2) + "\n");
    if (browser) await browser.close();
    server.kill("SIGTERM");
    serverLog.end();
  }
})().catch((error) => { console.error(error); process.exitCode = 1; });
