const assert = require("node:assert/strict");
const { spawnSync } = require("node:child_process");
const crypto = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");
const { chromium } = require("playwright");
const {
  ROOT, HARNESS_DIR, startServer, waitForServer, cleanupFixtures,
  visualBrowserLaunchOptions, captureCanvasPng, openFixturePage,
} = require("./shared");

const FIXTURE = "authoring_trigger_binding";
const DIRECTORY = path.join(ROOT, "target/playwright-behavior/trigger-binding");
const SOURCE = path.join(ROOT, "examples/authoring/trigger-binding.v0.json");
const PORT = 8131;
const SETTLE_MS = 250;
const POSITION_TOLERANCE = 1;
const RESTING_X = 40;
const ENGAGED_X = 160;

function sha256(bytes) {
  return crypto.createHash("sha256").update(bytes).digest("hex");
}

function buildFixture() {
  fs.mkdirSync(DIRECTORY, { recursive: true });
  const binary = path.join(DIRECTORY, `${FIXTURE}.riv`);
  const compiled = spawnSync("cargo", [
    "run", "--quiet", "--", "authoring", "compile", SOURCE, "-o", binary, "--json",
  ], { cwd: ROOT, encoding: "utf8" });
  assert.equal(compiled.status, 0, compiled.stderr || compiled.error?.message);
  const report = JSON.parse(compiled.stdout);
  assert.equal(report.ok, true);
  fs.writeFileSync(path.join(DIRECTORY, "compile.json"), compiled.stdout);
  fs.copyFileSync(SOURCE, path.join(DIRECTORY, "authoring.json"));
  fs.copyFileSync(binary, path.join(HARNESS_DIR, `${FIXTURE}.riv`));
  const name = (id) => {
    const entries = report.source_map.entries.filter((entry) => entry.authored_id === id);
    assert.equal(entries.length, 1, `unique source entry for ${id}`);
    assert.equal(entries[0].runtime_names.length, 1, `unique runtime name for ${id}`);
    return entries[0].runtime_names[0];
  };
  return {
    fixture: FIXTURE,
    machine: name("gate"),
    model: name("gate-model"),
    property: name("gate-model/advance"),
    boundInput: name("gate-advance"),
    machineTrigger: name("gate/reset"),
    binarySha256: sha256(fs.readFileSync(binary)),
  };
}

async function mount(page, plan) {
  await page.evaluate(async (plan) => {
    window.__RIVE_INSTANCE?.cleanup();
    document.getElementById("canvas").remove();
    const canvas = document.createElement("canvas");
    canvas.id = "canvas-controlled";
    canvas.width = 240;
    canvas.height = 160;
    canvas.style.width = "240px";
    canvas.style.height = "160px";
    document.body.appendChild(canvas);
    let runtime;
    await new Promise((resolve, reject) => {
      runtime = new rive.Rive({
        src: `${plan.fixture}.riv`, canvas, autoplay: true,
        autoBind: false, stateMachines: [plan.machine],
        onLoad: resolve,
        onLoadError: (error) => reject(new Error(String(error))),
      });
    });
    const model = runtime.viewModelByName(plan.model);
    if (!model) throw new Error(`missing model ${plan.model}`);
    const instance = model.instance();
    if (!instance) throw new Error("missing view-model instance");
    const property = instance.trigger(plan.property);
    if (!property) throw new Error(`missing trigger property ${plan.property}`);
    runtime.bindViewModelInstance(instance);
    const inputs = runtime.stateMachineInputs(plan.machine);
    const boundInput = inputs.find((input) => input.name === plan.boundInput);
    if (!boundInput) throw new Error(`missing synthesized input ${plan.boundInput}`);
    const machineTrigger = inputs.find((input) => input.name === plan.machineTrigger);
    if (!machineTrigger) throw new Error(`missing machine trigger ${plan.machineTrigger}`);
    window.__TRIGGER_BINDING = { runtime, instance, property, boundInput, machineTrigger };
  }, plan);
  await page.waitForTimeout(SETTLE_MS);
}

async function sample(page, label, expectedX) {
  await page.waitForTimeout(SETTLE_MS);
  const measured = await page.evaluate(() => {
    const canvas = document.getElementById("canvas-controlled");
    const context = canvas.getContext("2d");
    if (!context) throw new Error("missing canvas context");
    const { data } = context.getImageData(0, 80, canvas.width, 1);
    const pixels = [];
    for (let x = 0; x < canvas.width; x++) {
      const index = x * 4;
      if (data[index] === 36 && data[index + 1] === 107 && data[index + 2] === 253 && data[index + 3] === 255) pixels.push(x);
    }
    if (pixels.length < 70) throw new Error("panel did not render enough expected blue pixels");
    return { centerX: (pixels[0] + pixels[pixels.length - 1]) / 2, coloredPixels: pixels.length };
  });
  assert.ok(
    Math.abs(measured.centerX - expectedX) <= POSITION_TOLERANCE,
    `${label}: expected panel at ${expectedX}, got ${JSON.stringify(measured)}`,
  );
  const png = path.join(DIRECTORY, `${label}.png`);
  await captureCanvasPng(page, png);
  return { label, expectedX, ...measured, pngSha256: sha256(fs.readFileSync(png)) };
}

async function fireModelTrigger(page) {
  await page.evaluate(() => { window.__TRIGGER_BINDING.property.trigger(); });
}

async function fireMachineTrigger(page) {
  await page.evaluate(() => { window.__TRIGGER_BINDING.machineTrigger.fire(); });
}

async function fireBoundInput(page) {
  await page.evaluate(() => { window.__TRIGGER_BINDING.boundInput.fire(); });
}

async function main() {
  const plan = buildFixture();
  let server;
  let browser;
  let page;
  const errors = [];
  try {
    server = startServer(PORT);
    await waitForServer(PORT);
    browser = await chromium.launch(visualBrowserLaunchOptions());
    page = await openFixturePage(browser, PORT, FIXTURE, {
      pageOptions: { viewport: { width: 512, height: 512 }, deviceScaleFactor: 1 },
    });
    page.on("pageerror", (error) => errors.push(String(error)));
    page.on("console", (message) => { if (message.type() === "error") errors.push(message.text()); });
    await mount(page, plan);
    const samples = [await sample(page, "initial", RESTING_X)];

    await fireBoundInput(page);
    samples.push(await sample(page, "synthesized-input-only", RESTING_X));

    await fireModelTrigger(page);
    samples.push(await sample(page, "model-trigger-engaged", ENGAGED_X));

    await fireModelTrigger(page);
    samples.push(await sample(page, "model-trigger-while-engaged-holds", ENGAGED_X));

    await fireMachineTrigger(page);
    samples.push(await sample(page, "machine-trigger-released", RESTING_X));

    samples.push(await sample(page, "released-state-does-not-relatch", RESTING_X));

    await fireModelTrigger(page);
    samples.push(await sample(page, "model-trigger-re-engaged", ENGAGED_X));

    assert.deepEqual(errors, []);
    const evidence = {
      plan, samples,
      runtimeSha256: sha256(fs.readFileSync(path.join(ROOT, "assets/rive.js"))),
      wasmSha256: sha256(fs.readFileSync(path.join(ROOT, "assets/rive.wasm"))),
      browser: browser.version(),
    };
    fs.writeFileSync(path.join(DIRECTORY, "evidence.json"), `${JSON.stringify(evidence, null, 2)}\n`);
    console.log("model trigger drove the transition, the synthesized input alone did not, and the consumed trigger did not relatch on return");
  } finally {
    if (page) await page.close();
    if (browser) await browser.close();
    if (server) server.kill("SIGTERM");
    cleanupFixtures([FIXTURE]);
  }
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
