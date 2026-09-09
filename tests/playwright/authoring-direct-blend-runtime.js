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

const MODEL_BOUND = process.argv.includes("--model-bound");
const FIXED_WEIGHTS = process.argv.includes("--fixed-weights");
const ONE_DIMENSIONAL = process.argv.includes("--one-dimensional");
assert.ok(!ONE_DIMENSIONAL || MODEL_BOUND, "one-dimensional mode requires --model-bound");
assert.ok(!FIXED_WEIGHTS || !ONE_DIMENSIONAL, "fixed weights require direct blends");
const BASE_MODE = ONE_DIMENSIONAL ? "model-blend-1d" : MODEL_BOUND ? "model-blend" : "direct-blend";
const MODE = FIXED_WEIGHTS ? `${MODEL_BOUND ? "model-" : ""}fixed-blend` : BASE_MODE;
const FIXTURE = `authoring_${MODE.replaceAll("-", "_")}`;
const DIRECTORY = path.join(ROOT, "target/playwright-behavior", MODE);
const SOURCE = path.join(ROOT, "examples/authoring", `${FIXED_WEIGHTS && !MODEL_BOUND ? "fixed-blend" : BASE_MODE}-panel.v0.json`);
const PORT = 8129;
const SETTLE_MS = 250;
const POSITION_TOLERANCE = 1;

function sha256(bytes) {
  return crypto.createHash("sha256").update(bytes).digest("hex");
}

function buildFixture({ label = "dynamic", weights = null, restLast = false } = {}) {
  const directory = label === "dynamic" ? DIRECTORY : path.join(DIRECTORY, label);
  const fixture = label === "dynamic" ? FIXTURE : `${FIXTURE}_${label}`;
  fs.mkdirSync(directory, { recursive: true });
  const source = path.join(directory, "authoring.json");
  const document = JSON.parse(fs.readFileSync(SOURCE, "utf8"));
  if (FIXED_WEIGHTS) {
    const chart = document.behavior.statecharts[0];
    chart.inputs = chart.inputs.filter((input) => input.id !== "foundation");
    const motions = chart.states[0].direct_blend.motions;
    motions[0] = { motion: "rest-track", weight: { kind: "literal", value: 100, unit: "scalar" } };
    if (weights) {
      assert.equal(MODEL_BOUND, false, "constant cases have no model host");
      chart.inputs = chart.inputs.filter((input) => input.kind === "trigger");
      document.parameters = { ...document.parameters,
        left: { value: weights.left, unit: "scalar" },
        right: { value: weights.right, unit: "scalar" },
      };
      motions[1] = { motion: "left-track", weight: { kind: "parameter", name: "left" } };
      motions[2] = { motion: "right-track", weight: { kind: "parameter", name: "right" } };
      if (restLast) motions.push(motions.shift());
    }
  }
  fs.writeFileSync(source, `${JSON.stringify(document, null, 2)}\n`);
  const binary = path.join(directory, `${fixture}.riv`);
  const compiled = spawnSync("cargo", [
    "run", "--quiet", "--", "authoring", "compile", source, "-o", binary, "--json",
  ], { cwd: ROOT, encoding: "utf8" });
  assert.equal(compiled.status, 0, compiled.stderr || compiled.error?.message);
  const report = JSON.parse(compiled.stdout);
  assert.equal(report.ok, true);
  fs.writeFileSync(path.join(directory, "compile.json"), compiled.stdout);
  fs.copyFileSync(binary, path.join(HARNESS_DIR, `${fixture}.riv`));
  const name = (id) => {
    const entries = report.source_map.entries.filter((entry) => entry.authored_id === id);
    assert.equal(entries.length, 1, `unique source entry for ${id}`);
    assert.equal(entries[0].runtime_names.length, 1, `unique runtime name for ${id}`);
    return entries[0].runtime_names[0];
  };
  return {
    fixture, weights, restLast,
    sourceSha256: sha256(fs.readFileSync(source)),
    machine: name("panel"),
    left: weights ? null : name(MODEL_BOUND ? "left-model" : "panel/left-weight"),
    right: weights ? null : name(MODEL_BOUND ? "right-model" : "panel/right-weight"),
    model: MODEL_BOUND ? name("weights") : null,
    leftProperty: MODEL_BOUND ? name("weights/left") : null,
    rightProperty: MODEL_BOUND ? name("weights/right") : null,
    reset: name("panel/reset"),
    resume: name("panel/resume"),
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
    const inputs = runtime.stateMachineInputs(plan.machine);
    const find = (name) => {
      const input = inputs.find((input) => input.name === name);
      if (!input) throw new Error(`missing input ${name}`);
      return input;
    };
    const leftInput = plan.left ? find(plan.left) : null;
    const rightInput = plan.right ? find(plan.right) : null;
    if (plan.weights && inputs.length !== 2) throw new Error("fixed weights introduced extra controls");
    let instance = null;
    let left = leftInput;
    let right = rightInput;
    if (plan.model) {
      const model = runtime.viewModelByName(plan.model);
      if (!model) throw new Error(`missing model ${plan.model}`);
      instance = model.instance();
      if (!instance) throw new Error("missing model instance");
      left = instance.number(plan.leftProperty);
      right = instance.number(plan.rightProperty);
      if (!left || !right) throw new Error("missing model weight properties");
      left.value = 0;
      right.value = 0;
      runtime.bindViewModelInstance(instance);
    }
    window.__DIRECT_BLEND = {
      runtime, instance, left, right, leftInput, rightInput,
      reset: find(plan.reset), resume: find(plan.resume),
    };
  }, plan);
}

async function sample(page, label, expected) {
  await page.waitForTimeout(SETTLE_MS);
  const measured = await page.evaluate(() => {
    const canvas = document.getElementById("canvas-controlled");
    const context = canvas.getContext("2d");
    if (!context) throw new Error("missing canvas context");
    function center(y, rgb) {
      const { data } = context.getImageData(0, y, canvas.width, 1);
      const pixels = [];
      for (let x = 0; x < canvas.width; x++) {
        const index = x * 4;
        if (data[index] === rgb[0] && data[index + 1] === rgb[1] &&
            data[index + 2] === rgb[2] && data[index + 3] === 255) pixels.push(x);
      }
      if (pixels.length < 20) throw new Error(`panel at y=${y} did not render`);
      return { centerX: (pixels[0] + pixels.at(-1)) / 2, coloredPixels: pixels.length };
    }
    return {
      left: center(40, [36, 107, 253]), right: center(120, [34, 197, 94]),
      leftWeight: window.__DIRECT_BLEND.left?.value ?? null,
      rightWeight: window.__DIRECT_BLEND.right?.value ?? null,
      leftInput: window.__DIRECT_BLEND.leftInput?.value ?? null,
      rightInput: window.__DIRECT_BLEND.rightInput?.value ?? null,
    };
  });
  for (const [side, x] of Object.entries(expected)) {
    const position = measured[side].centerX;
    const matches = Array.isArray(x)
      ? position > x[0] && position < x[1]
      : Math.abs(position - x) <= POSITION_TOLERANCE;
    assert.ok(matches, `${label}: expected ${side} at ${JSON.stringify(x)}, got ${JSON.stringify(measured)}`);
  }
  const png = path.join(DIRECTORY, `${label}.png`);
  await captureCanvasPng(page, png);
  return { label, expected, ...measured, pngSha256: sha256(fs.readFileSync(png)) };
}

async function openControlledFixture(browser, plan, errors) {
  const page = await openFixturePage(browser, PORT, plan.fixture, {
    pageOptions: { viewport: { width: 512, height: 512 }, deviceScaleFactor: 1 },
  });
  page.on("pageerror", (error) => errors.push(String(error)));
  page.on("console", (message) => { if (message.type() === "error") errors.push(message.text()); });
  try {
    await mount(page, plan);
    return page;
  } catch (error) {
    await page.close();
    throw error;
  }
}

async function main() {
  const plan = buildFixture();
  const fixedPlans = [];
  let server;
  let browser;
  let page;
  const errors = [];
  const samples = [];
  try {
    server = startServer(PORT);
    await waitForServer(PORT);
    browser = await chromium.launch(visualBrowserLaunchOptions());
    page = await openControlledFixture(browser, plan, errors);
    if (MODEL_BOUND) {
      samples.push(await sample(page, "initial-model", { left: 40, right: 40 }));
      await page.evaluate(() => {
        window.__DIRECT_BLEND.leftInput.value = 90;
        window.__DIRECT_BLEND.rightInput.value = 10;
      });
      samples.push(await sample(page, "input-only", { left: 40, right: 40 }));
      assert.equal(samples.at(-1).leftWeight, 0);
      assert.equal(samples.at(-1).rightWeight, 0);
    }
    for (const [label, left, right, expectedLeft, expectedRight] of [
      ["zero", 0, 0, 40, 40],
      ...(ONE_DIMENSIONAL ? [
        ["between-stops", 25, 75, [41, 119], [121, 199]],
        ["reverse-between-stops", 75, 25, [121, 199], [41, 119]],
      ] : []),
      ["left-half", 50, 0, 120, 40],
      ["independent", 100, 50, 200, 120],
      ["reversed", 0, 100, 40, 200],
      ["clamped", -20, 120, 40, 200],
      ["both-full", 100, 100, 200, 200],
    ]) {
      await page.evaluate(({ left, right }) => {
        window.__DIRECT_BLEND.left.value = left;
        window.__DIRECT_BLEND.right.value = right;
      }, { left, right });
      samples.push(await sample(page, label, { left: expectedLeft, right: expectedRight }));
      assert.equal(samples.at(-1).leftWeight, left);
      assert.equal(samples.at(-1).rightWeight, right);
    }
    await page.evaluate(() => window.__DIRECT_BLEND.reset.fire());
    samples.push(await sample(page, "reset-state", { left: 40, right: 40 }));
    if (ONE_DIMENSIONAL) {
      await page.evaluate(() => {
        window.__DIRECT_BLEND.left.value = 25;
        window.__DIRECT_BLEND.right.value = 75;
      });
      samples.push(await sample(page, "model-change-while-resting", { left: 40, right: 40 }));
    }
    await page.evaluate(() => window.__DIRECT_BLEND.resume.fire());
    samples.push(await sample(page, "resume-state", ONE_DIMENSIONAL
      ? { left: [41, 119], right: [121, 199] }
      : { left: 200, right: 200 }));
    await page.evaluate(() => {
      window.__DIRECT_BLEND.left.value = 0;
      window.__DIRECT_BLEND.right.value = 0;
    });
    samples.push(await sample(page, "zero-again", { left: 40, right: 40 }));
    if (FIXED_WEIGHTS && !MODEL_BOUND) {
      for (const [label, left, right, expected, restLast] of [
        ["fixed-zero", 0, 0, { left: 40, right: 40 }, false],
        ["fixed-fractions", 25.5, 75.5, { left: 80.8, right: 160.8 }, false],
        ["fixed-half", 50, 50, { left: 120, right: 120 }, false],
        ["fixed-full", 100, 100, { left: 200, right: 200 }, false],
        ["fixed-reversed", 75.5, 25.5, { left: 160.8, right: 80.8 }, false],
        ["fixed-rest-last", 25.5, 75.5, { left: 40, right: 40 }, true],
      ]) {
        const fixed = buildFixture({ label, weights: { left, right }, restLast });
        fixedPlans.push(fixed);
        await page.close();
        page = await openControlledFixture(browser, fixed, errors);
        samples.push(await sample(page, label, expected));
        assert.equal(samples.at(-1).leftWeight, null);
        assert.equal(samples.at(-1).rightWeight, null);
        if (label === "fixed-fractions") {
          await page.evaluate(() => window.__DIRECT_BLEND.reset.fire());
          samples.push(await sample(page, "fixed-reset", { left: 40, right: 40 }));
          await page.evaluate(() => window.__DIRECT_BLEND.resume.fire());
          samples.push(await sample(page, "fixed-resume", expected));
        }
      }
    }
    assert.deepEqual(errors, []);
    if (MODEL_BOUND) {
      for (const sample of samples.slice(1)) {
        assert.equal(sample.leftInput, 90, `${sample.label}: model must not mirror input`);
        assert.equal(sample.rightInput, 10, `${sample.label}: model must not mirror input`);
      }
    }
    const evidence = {
      plan, fixedPlans, samples,
      runtimeSha256: sha256(fs.readFileSync(path.join(ROOT, "assets/rive.js"))),
      wasmSha256: sha256(fs.readFileSync(path.join(ROOT, "assets/rive.wasm"))),
      browser: browser.version(),
    };
    fs.writeFileSync(path.join(DIRECTORY, "evidence.json"), `${JSON.stringify(evidence, null, 2)}\n`);
    const behavior = ONE_DIMENSIONAL ? "numeric stops interpolated, clamped, reversed, exited and resumed" : "independent weights clamped, reversed, exited and resumed";
    console.log(`${MODE}: ${behavior}${MODEL_BOUND ? "; model changes, not synthesized inputs, controlled the result" : ""}`);
  } finally {
    if (page) await page.close();
    if (browser) await browser.close();
    if (server) server.kill("SIGTERM");
    cleanupFixtures([FIXTURE, ...fixedPlans.map((plan) => plan.fixture)]);
  }
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
