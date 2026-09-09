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

const DIRECTORY = path.join(ROOT, "target/playwright-behavior/transition-guards");
const SOURCE = path.join(ROOT, "examples/authoring/behavior-binding.v0.json");
const PORT = 8130;
const SETTLE_MS = 250;
const FPS = 60;
const scalar = (value) => ({ kind: "literal", value, unit: "scalar" });
const sha256 = (bytes) => crypto.createHash("sha256").update(bytes).digest("hex");

function cli(args) {
  const result = spawnSync("cargo", ["run", "--quiet", "--", ...args], { cwd: ROOT, encoding: "utf8" });
  assert.equal(result.status, 0, result.stderr || result.error?.message);
  return result.stdout;
}

function compile(label, document) {
  const directory = path.join(DIRECTORY, label);
  fs.mkdirSync(directory, { recursive: true });
  const source = path.join(directory, "authoring.json");
  const binary = path.join(directory, "guard.riv");
  fs.writeFileSync(source, `${JSON.stringify(document, null, 2)}\n`);
  const output = cli(["authoring", "compile", source, "-o", binary, "--json"]);
  fs.writeFileSync(path.join(directory, "compile.json"), output);
  const report = JSON.parse(output);
  assert.equal(report.ok, true);
  const name = (id) => {
    const entries = report.source_map.entries.filter((entry) => entry.authored_id === id);
    assert.equal(entries.length, 1, `unique authored id ${id}`);
    assert.equal(entries[0].runtime_names.length, 1, `unique runtime name ${id}`);
    return entries[0].runtime_names[0];
  };
  return {
    label, directory, binary, name,
    sourceSha256: sha256(fs.readFileSync(source)),
    binarySha256: sha256(fs.readFileSync(binary)),
    machine: name("gate"),
  };
}

function document() {
  const input = JSON.parse(fs.readFileSync(SOURCE, "utf8"));
  input.behavior.models[0].properties.push({ kind: "number", id: "level", value: scalar(0) });
  input.behavior.bindings.push({ id: "gate-level", model: "gate-model", property: "level" });
  input.behavior.statecharts[0].inputs = [
    { kind: "bool", id: "armed", value: false },
    { kind: "number", id: "load", value: scalar(0) },
    { kind: "trigger", id: "release" },
  ];
  input.behavior.statecharts[0].transitions[0].when = { all: [
    { input: "armed", equals: true },
    { binding: "gate-enabled", equals: true },
    { input: "load", compare: "greater_or_equal", value: scalar(60) },
    { binding: "gate-level", compare: "greater_or_equal", value: scalar(80) },
    { trigger: "release" },
  ] };
  return input;
}

function buildModelPlan(inRegion) {
  const input = document();
  const label = inRegion ? "region" : "root";
  if (inRegion) {
    const chart = input.behavior.statecharts[0];
    chart.regions = [{ id: "secondary", initial: chart.initial, states: chart.states, transitions: chart.transitions }];
    chart.initial = "idle";
    chart.states = [{ id: "idle", motion: "rest-track" }];
    chart.transitions = [];
  }
  const plan = compile(label, input);
  plan.fixture = `authoring_guard_${label}`;
  fs.copyFileSync(plan.binary, path.join(HARNESS_DIR, `${plan.fixture}.riv`));
  plan.names = Object.fromEntries([
    ["model", "gate-model"], ["enabled", "gate-model/enabled"], ["level", "gate-model/level"],
    ["enabledInput", "gate-enabled"], ["levelInput", "gate-level"],
    ["armed", "gate/armed"], ["load", "gate/load"], ["release", "gate/release"],
  ].map(([key, id]) => [key, plan.name(id)]));
  return plan;
}

async function mount(browser, plan, errors) {
  const page = await openFixturePage(browser, PORT, plan.fixture, {
    pageOptions: { viewport: { width: 512, height: 512 }, deviceScaleFactor: 1 },
  });
  page.on("pageerror", (error) => errors.push(String(error)));
  page.on("console", (message) => { if (message.type() === "error") errors.push(message.text()); });
  try {
    await page.evaluate(async ({ fixture, machine, names }) => {
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
          src: `${fixture}.riv`, canvas, autoplay: true, autoBind: false,
          stateMachines: [machine], onLoad: resolve,
          onLoadError: (error) => reject(new Error(String(error))),
        });
      });
      const instance = runtime.viewModelByName(names.model).instance();
      const enabled = instance.boolean(names.enabled);
      const level = instance.number(names.level);
      enabled.value = false;
      level.value = 0;
      runtime.bindViewModelInstance(instance);
      const inputs = runtime.stateMachineInputs(machine);
      const input = (key) => {
        const found = inputs.find((candidate) => candidate.name === names[key]);
        if (!found) throw new Error(`missing input ${key}`);
        return found;
      };
      window.__GUARD = {
        runtime, instance, enabled, level, armed: input("armed"), load: input("load"),
        release: input("release"), enabledInput: input("enabledInput"), levelInput: input("levelInput"),
      };
    }, { fixture: plan.fixture, machine: plan.machine, names: plan.names });
    await page.waitForTimeout(SETTLE_MS);
    return page;
  } catch (error) {
    await page.close();
    throw error;
  }
}

async function sample(page, plan, label, expectedX) {
  await page.waitForTimeout(SETTLE_MS);
  const measured = await page.evaluate(() => {
    const canvas = document.getElementById("canvas-controlled");
    const { data } = canvas.getContext("2d").getImageData(0, 80, canvas.width, 1);
    const pixels = [];
    for (let x = 0; x < canvas.width; x++) {
      const i = x * 4;
      if (data[i] === 36 && data[i + 1] === 107 && data[i + 2] === 253 && data[i + 3] === 255) pixels.push(x);
    }
    if (pixels.length < 70) throw new Error("guard panel did not render");
    const guard = window.__GUARD;
    return {
      centerX: (pixels[0] + pixels.at(-1)) / 2,
      armed: guard.armed.value, enabled: guard.enabled.value, load: guard.load.value, level: guard.level.value,
      enabledInput: guard.enabledInput.value, levelInput: guard.levelInput.value,
    };
  });
  assert.ok(Math.abs(measured.centerX - expectedX) <= 1, `${plan.label}/${label}: expected ${expectedX}, got ${JSON.stringify(measured)}`);
  const png = path.join(plan.directory, `${label}.png`);
  await captureCanvasPng(page, png);
  return { scope: plan.label, label, expectedX, ...measured, pngSha256: sha256(fs.readFileSync(png)) };
}

async function verifyModelGuards(browser, plan, errors) {
  const samples = [];
  const masks = [0, 1, 2, 4, 8, 16, 30, 29, 27, 23, 15, 31];
  for (const mask of masks) {
    const page = await mount(browser, plan, errors);
    try {
      await page.evaluate((mask) => {
        const guard = window.__GUARD;
        guard.armed.value = Boolean(mask & 1);
        guard.enabled.value = Boolean(mask & 2);
        guard.load.value = mask & 4 ? 60 : 59;
        guard.level.value = mask & 8 ? 80 : 79;
      }, mask);
      await page.waitForTimeout(SETTLE_MS);
      if (mask & 16) await page.evaluate(() => window.__GUARD.release.fire());
      samples.push(await sample(page, plan, `mask-${mask}`, mask === 31 ? 160 : 40));
      if (mask === 30) {
        await page.evaluate(() => { window.__GUARD.armed.value = true; });
        samples.push(await sample(page, plan, "blocked-trigger-not-replayed", 40));
        await page.evaluate(() => window.__GUARD.release.fire());
        samples.push(await sample(page, plan, "fresh-trigger-releases", 160));
      }
    } finally {
      await page.close();
    }
  }
  const page = await mount(browser, plan, errors);
  try {
    await page.evaluate(() => {
      const guard = window.__GUARD;
      guard.armed.value = true;
      guard.load.value = 60;
      guard.enabledInput.value = true;
      guard.levelInput.value = 80;
    });
    await page.evaluate(() => window.__GUARD.release.fire());
    samples.push(await sample(page, plan, "inputs-cannot-stand-in-for-models", 40));
    assert.equal(samples.at(-1).enabled, false);
    assert.equal(samples.at(-1).level, 0);
  } finally {
    await page.close();
  }
  return samples;
}

function pngFiles(directory) {
  return fs.readdirSync(directory, { withFileTypes: true }).flatMap((entry) => {
    const filename = path.join(directory, entry.name);
    return entry.isDirectory() ? pngFiles(filename) : entry.name.endsWith(".png") ? [filename] : [];
  }).sort();
}

function verifyTimedGuard() {
  const input = document();
  input.behavior.models = [];
  input.behavior.bindings = [];
  const chart = input.behavior.statecharts[0];
  chart.inputs = chart.inputs.filter((item) => item.kind !== "trigger");
  chart.transitions[0].when.all = [chart.transitions[0].when.all[0], chart.transitions[0].when.all[2]];
  for (const track of input.motion.tracks) {
    track.duration_frames = scalar(120);
    track.keyframes[1].frame = scalar(120);
  }
  const instant = compile("instant", input);
  chart.transitions[0].exit_time_ms = scalar(1000);
  chart.transitions[0].duration_ms = scalar(1000);
  const timed = compile("timed", input);
  const frames = [0, 30, 54, 66, 90, 126];
  const render = (plan, label, armed, load) => {
    const directory = path.join(plan.directory, label);
    cli(["render", plan.binary, "--state-machine", plan.machine, "--frames", frames.join(","),
      "--fps", String(FPS), "-o", directory, "--width", "240", "--height", "160", "--scale", "1",
      "--input", `${plan.name("gate/armed")}=${armed}@1`, "--input", `${plan.name("gate/load")}=${load}@1`]);
    const pngs = pngFiles(directory);
    assert.equal(pngs.length, frames.length, `${label}: frame count`);
    return pngs.map((png) => sha256(fs.readFileSync(png)));
  };
  const control = render(timed, "neither", false, 0);
  const armedOnly = render(timed, "armed-only", true, 0);
  const loadOnly = render(timed, "load-only", false, 60);
  const ungated = render(instant, "all", true, 60);
  const gated = render(timed, "all", true, 60);
  assert.equal(new Set(control).size, 1);
  assert.deepEqual(armedOnly, control);
  assert.deepEqual(loadOnly, control);
  assert.notEqual(ungated.at(-1), control[0]);
  for (const index of [0, 1, 2]) assert.equal(gated[index], control[0], "guard left before exit time");
  for (const index of [3, 4]) {
    assert.notEqual(gated[index], control[0], "guard did not begin blending");
    assert.notEqual(gated[index], ungated.at(-1), "guard skipped blend duration");
  }
  assert.notEqual(gated[3], gated[4]);
  assert.equal(gated.at(-1), ungated.at(-1));
  return { frames, fps: FPS, control, armedOnly, loadOnly, ungated, gated,
    timedSourceSha256: timed.sourceSha256, timedBinarySha256: timed.binarySha256 };
}

async function main() {
  const plans = [buildModelPlan(false), buildModelPlan(true)];
  const errors = [];
  let server;
  let browser;
  try {
    server = startServer(PORT);
    await waitForServer(PORT);
    browser = await chromium.launch(visualBrowserLaunchOptions());
    const samples = [];
    for (const plan of plans) samples.push(...await verifyModelGuards(browser, plan, errors));
    assert.deepEqual(errors, []);
    const timing = verifyTimedGuard();
    fs.writeFileSync(path.join(DIRECTORY, "evidence.json"), `${JSON.stringify({
      plans: plans.map(({ label, sourceSha256, binarySha256, names }) => ({ label, sourceSha256, binarySha256, names })),
      samples, timing, browser: browser.version(),
      runtimeSha256: sha256(fs.readFileSync(path.join(ROOT, "assets/rive.js"))),
      wasmSha256: sha256(fs.readFileSync(path.join(ROOT, "assets/rive.wasm"))),
    }, null, 2)}\n`);
    console.log(`${samples.length} model/input/trigger guard samples and ${timing.frames.length * 5} timed frames passed`);
  } finally {
    if (browser) await browser.close();
    if (server) server.kill("SIGTERM");
    cleanupFixtures(plans.map((plan) => plan.fixture));
  }
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
