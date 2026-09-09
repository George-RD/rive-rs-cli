const assert = require("node:assert/strict");
const { spawnSync } = require("node:child_process");
const crypto = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");
const { chromium } = require("playwright");
const { ROOT, visualBrowserLaunchOptions } = require("./shared");

const DIRECTORY = path.join(ROOT, "target/playwright-behavior/automatic-transitions");
const SOURCE = path.join(ROOT, "examples/authoring/automatic-sequence.v0.json");
const FRAMES = [0, 15, 29, 31, 45, 59, 61, 75, 91, 120];
const FPS = 60;
const WIDTH = 240;
const HEIGHT = 160;
const POSITION_TOLERANCE = 2;
const sha256 = (bytes) => crypto.createHash("sha256").update(bytes).digest("hex");
const scalar = (value) => ({ kind: "literal", value, unit: "scalar" });

function cli(args) {
  const result = spawnSync("cargo", ["run", "--quiet", "--", ...args], { cwd: ROOT, encoding: "utf8" });
  assert.equal(result.status, 0, result.stderr || result.error?.message);
  return result.stdout;
}

function compile(label) {
  const document = JSON.parse(fs.readFileSync(SOURCE, "utf8"));
  const chart = document.behavior.statecharts[0];
  assert.deepEqual(document.behavior.models || [], []);
  assert.deepEqual(document.behavior.bindings || [], []);
  assert.deepEqual(chart.inputs || [], []);
  for (const layer of [chart, ...chart.regions]) {
    if (label === "resting" || label === "engaged") {
      layer.initial = label;
      layer.transitions = [];
    } else if (label === "instantaneous") {
      layer.transitions[0].duration_ms = scalar(0);
    } else if (label === "ungated" || label === "zero-gate") {
      delete layer.transitions[0].duration_ms;
      if (label === "zero-gate") layer.transitions[0].exit_time_ms = scalar(0);
      else delete layer.transitions[0].exit_time_ms;
    }
  }
  const directory = path.join(DIRECTORY, label);
  fs.mkdirSync(directory, { recursive: true });
  const source = path.join(directory, "authoring.json");
  const binary = path.join(directory, "sequence.riv");
  fs.writeFileSync(source, `${JSON.stringify(document, null, 2)}\n`);
  const output = cli(["authoring", "compile", source, "-o", binary, "--json"]);
  fs.writeFileSync(path.join(directory, "compile.json"), output);
  const report = JSON.parse(output);
  assert.equal(report.ok, true);
  const entries = report.source_map.entries.filter((entry) => entry.authored_id === "sequence");
  assert.equal(entries.length, 1);
  assert.equal(entries[0].runtime_names.length, 1);
  return { label, directory, binary, machine: entries[0].runtime_names[0],
    sourceSha256: sha256(fs.readFileSync(source)), binarySha256: sha256(fs.readFileSync(binary)) };
}

function pngFiles(directory) {
  return fs.readdirSync(directory, { withFileTypes: true }).flatMap((entry) => {
    const file = path.join(directory, entry.name);
    return entry.isDirectory() ? pngFiles(file) : entry.name.endsWith(".png") ? [file] : [];
  }).sort();
}

async function positions(page, bytes) {
  return page.evaluate(async ({ base64, width, height }) => {
    const image = new Image();
    image.src = `data:image/png;base64,${base64}`;
    await image.decode();
    if (image.width !== width || image.height !== height) throw new Error("unexpected render size");
    const canvas = document.createElement("canvas");
    canvas.width = width;
    canvas.height = height;
    const context = canvas.getContext("2d");
    context.drawImage(image, 0, 0);
    const pixels = context.getImageData(0, 0, width, height).data;
    return Object.fromEntries([
      ["upper", [36, 107, 253]], ["lower", [36, 200, 144]],
    ].map(([name, rgb]) => {
      let minX = width;
      let maxX = -1;
      let count = 0;
      for (let offset = 0; offset < pixels.length; offset += 4) {
        if (pixels[offset + 3] < 250 || !rgb.every((value, channel) => Math.abs(pixels[offset + channel] - value) <= 2)) continue;
        const x = (offset / 4) % width;
        minX = Math.min(minX, x);
        maxX = Math.max(maxX, x);
        count++;
      }
      if (count < 400) throw new Error(`missing ${name} panel: ${count} pixels`);
      return [name, { x: (minX + maxX) / 2, pixels: count }];
    }));
  }, { base64: bytes.toString("base64"), width: WIDTH, height: HEIGHT });
}

async function sample(page, plan) {
  const directory = path.join(plan.directory, "frames");
  cli(["render", plan.binary, "--state-machine", plan.machine, "--frames", FRAMES.join(","),
    "--fps", String(FPS), "-o", directory, "--width", String(WIDTH), "--height", String(HEIGHT), "--scale", "1"]);
  const files = pngFiles(directory);
  assert.equal(files.length, FRAMES.length, `${plan.label}: frame count`);
  const samples = [];
  for (const [index, file] of files.entries()) {
    const bytes = fs.readFileSync(file);
    samples.push({ frame: FRAMES[index], ...await positions(page, bytes), sha256: sha256(bytes),
      file: path.relative(DIRECTORY, file).split(path.sep).join("/") });
  }
  return { label: plan.label, sourceSha256: plan.sourceSha256, binarySha256: plan.binarySha256, samples };
}

function verify(plans) {
  const get = (label, frame) => plans.find((plan) => plan.label === label).samples.find((sample) => sample.frame === frame);
  const rest = get("resting", 120);
  const final = get("engaged", 120);
  for (const frame of FRAMES) {
    assert.equal(get("resting", frame).sha256, rest.sha256);
    assert.equal(get("engaged", frame).sha256, final.sha256);
  }
  for (const key of ["upper", "lower"]) {
    assert.ok(Math.abs(rest[key].x - 39.5) <= POSITION_TOLERANCE, `${key}: resting position`);
    assert.ok(Math.abs(final[key].x - 199.5) <= POSITION_TOLERANCE, `${key}: final position`);
    const [beforeGate, duringBlend, afterBlend] = key === "upper" ? [29, 45, 61] : [59, 75, 91];
    assert.equal(get("timed", beforeGate)[key].x, rest[key].x, `${key}: left before exit gate`);
    const middle = get("timed", duringBlend)[key].x;
    assert.ok(middle > rest[key].x + POSITION_TOLERANCE, `${key}: failed to advance automatically`);
    assert.ok(middle < final[key].x - POSITION_TOLERANCE, `${key}: skipped blend duration`);
    assert.equal(get("timed", afterBlend)[key].x, final[key].x, `${key}: did not reach destination`);
    let previous = rest[key].x;
    for (const frame of FRAMES) {
      const current = get("timed", frame)[key].x;
      assert.ok(current >= previous, `${key}: non-monotonic progression at frame ${frame}`);
      previous = current;
    }
    const afterGate = key === "upper" ? 31 : 61;
    assert.equal(get("instantaneous", afterGate)[key].x, final[key].x, `${key}: instantaneous gate`);
  }
  assert.equal(get("timed", 45).lower.x, rest.lower.x, "region must wait while root blends");
  assert.equal(get("timed", 75).upper.x, final.upper.x, "root must hold while region blends");
  for (const label of ["timed", "instantaneous", "ungated", "zero-gate"]) {
    assert.equal(get(label, 120).sha256, final.sha256, `${label}: final pixels`);
  }
  for (const label of ["ungated", "zero-gate"]) {
    assert.equal(get(label, 15).sha256, final.sha256, `${label}: must not retain an implicit gate`);
  }
}

async function main() {
  fs.rmSync(DIRECTORY, { recursive: true, force: true });
  const plans = ["resting", "engaged", "timed", "instantaneous", "ungated", "zero-gate"].map(compile);
  const browser = await chromium.launch(visualBrowserLaunchOptions());
  try {
    const page = await browser.newPage();
    const results = [];
    for (const plan of plans) results.push(await sample(page, plan));
    fs.writeFileSync(path.join(DIRECTORY, "evidence.json"), `${JSON.stringify({
      frames: FRAMES, fps: FPS, plans: results, browser: browser.version(),
      runtimeSha256: sha256(fs.readFileSync(path.join(ROOT, "assets/rive.js"))),
      wasmSha256: sha256(fs.readFileSync(path.join(ROOT, "assets/rive.wasm"))),
    }, null, 2)}\n`);
    verify(results);
    console.log(`${plans.length * FRAMES.length} automatic-transition frames passed without input or model mutation`);
  } finally {
    await browser.close();
  }
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
