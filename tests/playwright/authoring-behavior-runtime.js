const { chromium } = require("playwright");
const crypto = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");
const {
  ROOT,
  HARNESS_DIR,
  OUT_DIR,
  run,
  startServer,
  waitForServer,
  cleanupFixtures,
  visualBrowserLaunchOptions,
  captureCanvasPng,
  openFixturePage,
} = require("./shared");

const FIXTURE = "authoring_behavior_binding";
const INPUT = path.join(ROOT, "examples", "authoring", "behavior-binding.v0.json");
const INTERACTION_FIXTURE = "authoring_pointer_statechart";
const INTERACTION_INPUT = path.join(
  ROOT,
  "examples",
  "authoring",
  "pointer-statechart.v0.json",
);
const CURRENT_DIR = path.join(ROOT, "target", "playwright-behavior");
const INTERACTION_DIR = path.join(CURRENT_DIR, "typed-interaction");
const BASELINE_DIR = path.join(ROOT, "tests", "playwright", "baselines");
const PORT = Number(process.env.PLAYWRIGHT_PORT || 8767);
const PLAN = {
  fixture: FIXTURE,
  stateMachine: "auth__behavior_2dstage__gate__state_machine",
  viewModel: "auth__behavior_2dstage__gate_2dmodel__view_model",
  property: "auth__behavior_2dstage__gate_2dmodel__enabled__view_model_property",
};
const INTERACTION_PLAN = {
  stateMachine: "auth__interaction_2dstage__gate__state_machine",
  input: "auth__interaction_2dstage__gate__pressed__input",
};

function buildFixture() {
  fs.mkdirSync(OUT_DIR, { recursive: true });
  const output = path.join(OUT_DIR, `${FIXTURE}.riv`);
  run("cargo", ["run", "--quiet", "--", "authoring", "compile", INPUT, "-o", output]);
  fs.copyFileSync(output, path.join(HARNESS_DIR, `${FIXTURE}.riv`));
}

function buildInteractionFixture() {
  fs.mkdirSync(OUT_DIR, { recursive: true });
  const output = path.join(OUT_DIR, `${INTERACTION_FIXTURE}.riv`);
  run("cargo", [
    "run",
    "--quiet",
    "--",
    "authoring",
    "compile",
    INTERACTION_INPUT,
    "-o",
    output,
  ]);
  return output;
}

function listPngs(directory) {
  const pngs = [];
  const visit = (current) => {
    for (const entry of fs.readdirSync(current, { withFileTypes: true })) {
      const entryPath = path.join(current, entry.name);
      if (entry.isDirectory()) {
        visit(entryPath);
      } else if (entry.isFile() && entry.name.endsWith(".png")) {
        pngs.push(entryPath);
      }
    }
  };
  visit(directory);
  return pngs.sort();
}

function renderInteraction(riv, label, extraArgs = []) {
  const output = path.join(INTERACTION_DIR, label);
  fs.mkdirSync(output, { recursive: true });
  run("cargo", [
    "run",
    "--quiet",
    "--",
    "render",
    riv,
    "--state-machine",
    INTERACTION_PLAN.stateMachine,
    "--frames",
    "0,12",
    "-o",
    output,
    "--width",
    "240",
    "--height",
    "160",
    "--scale",
    "1",
    ...extraArgs,
  ]);
  const pngs = listPngs(output);
  if (pngs.length < 2) {
    throw new Error(`${label} typed interaction render produced fewer than two PNG frames`);
  }
  return {
    first: fs.readFileSync(pngs[0]),
    last: fs.readFileSync(pngs[pngs.length - 1]),
    pngs,
  };
}

function sha256(bytes) {
  return crypto.createHash("sha256").update(bytes).digest("hex");
}

function verifyCliInteraction() {
  fs.rmSync(INTERACTION_DIR, { recursive: true, force: true });
  const riv = buildInteractionFixture();
  const control = renderInteraction(riv, "control");
  const directInput = renderInteraction(riv, "input", [
    "--input",
    `${INTERACTION_PLAN.input}=true@1`,
  ]);
  const pointer = renderInteraction(riv, "pointer", ["--pointer", "down:60,100@1"]);

  if (!control.first.equals(directInput.first) || !control.first.equals(pointer.first)) {
    throw new Error("typed interaction changed before its scheduled input or pointer event");
  }
  if (control.last.equals(directInput.last)) {
    throw new Error("--input did not drive the authored typed statechart transition");
  }
  if (control.last.equals(pointer.last)) {
    throw new Error("--pointer did not drive the authored typed statechart transition");
  }
  if (!directInput.last.equals(pointer.last)) {
    throw new Error("direct input and pointer listener did not converge on the same authored state");
  }

  const evidence = {
    stateMachine: INTERACTION_PLAN.stateMachine,
    input: INTERACTION_PLAN.input,
    pointer: "down:60,100@1",
    control: sha256(control.last),
    directInput: sha256(directInput.last),
    pointerInput: sha256(pointer.last),
  };
  fs.writeFileSync(
    path.join(INTERACTION_DIR, "evidence.json"),
    `${JSON.stringify(evidence, null, 2)}\n`,
  );
  console.log("typed authored statechart responds to render --input and --pointer");
}

function assertSamePng(actualPath, baselinePath, label) {
  if (!fs.existsSync(baselinePath)) {
    throw new Error(`missing ${label} baseline; run this contract with --update and review it`);
  }
  const actual = fs.readFileSync(actualPath);
  const baseline = fs.readFileSync(baselinePath);
  if (!actual.equals(baseline)) {
    throw new Error(`${label} runtime evidence differs from its retained baseline`);
  }
}

async function mountBehavior(page) {
  const runtimeErrors = [];
  page.on("pageerror", (error) => runtimeErrors.push(String(error)));
  page.on("console", (message) => {
    if (message.type() === "error") {
      runtimeErrors.push(message.text());
    }
  });

  const result = await page.evaluate(async (plan) => {
    const originalCanvas = document.getElementById("canvas");
    if (!originalCanvas) {
      return { ok: false, error: "missing canvas" };
    }
    originalCanvas.style.display = "none";

    const controlledCanvas = document.createElement("canvas");
    controlledCanvas.id = "canvas-controlled";
    controlledCanvas.style.width = originalCanvas.style.width;
    controlledCanvas.style.height = originalCanvas.style.height;
    controlledCanvas.width = originalCanvas.width;
    controlledCanvas.height = originalCanvas.height;
    originalCanvas.parentElement.appendChild(controlledCanvas);

    try {
      let runtime;
      await new Promise((resolve, reject) => {
        runtime = new rive.Rive({
          src: `${plan.fixture}.riv`,
          canvas: controlledCanvas,
          autoplay: true,
          autoBind: false,
          stateMachines: [plan.stateMachine],
          onLoad: resolve,
          onLoadError: (error) => reject(new Error(String(error || "rive load error"))),
        });
      });

      const viewModel = runtime.viewModelByName(plan.viewModel);
      if (!viewModel) {
        throw new Error(`missing view model ${plan.viewModel}`);
      }
      const instance = viewModel.instance();
      if (!instance) {
        throw new Error(`could not create a view-model instance for ${plan.viewModel}`);
      }
      const property = instance.boolean(plan.property);
      if (!property) {
        throw new Error(`missing boolean property ${plan.property}`);
      }

      property.value = false;
      runtime.bindViewModelInstance(instance);
      window.__BEHAVIOR_RIVE = runtime;
      window.__BEHAVIOR_BOOL = property;
      await new Promise((resolve) => setTimeout(resolve, 150));
      return { ok: true, error: "" };
    } catch (error) {
      return { ok: false, error: String(error?.message || error || "unknown error") };
    }
  }, PLAN);

  if (!result.ok) {
    throw new Error(`${FIXTURE}.riv failed behavior setup: ${result.error}`);
  }
  if (runtimeErrors.length > 0) {
    throw new Error(`${FIXTURE}.riv runtime errors: ${runtimeErrors.join(" | ")}`);
  }

  return runtimeErrors;
}

async function setBoundBoolean(page, value) {
  const changed = await page.evaluate(async (nextValue) => {
    if (!window.__BEHAVIOR_BOOL) {
      return false;
    }
    window.__BEHAVIOR_BOOL.value = nextValue;
    await new Promise((resolve) => setTimeout(resolve, 250));
    return true;
  }, value);
  if (!changed) {
    throw new Error("official runtime did not expose the bound view-model boolean");
  }
}

async function main() {
  const update = process.argv.includes("--update");
  fs.mkdirSync(CURRENT_DIR, { recursive: true });
  fs.mkdirSync(BASELINE_DIR, { recursive: true });

  let server;
  let browser;
  let page;
  try {
    verifyCliInteraction();
    buildFixture();
    server = startServer(PORT);
    await waitForServer(PORT);
    browser = await chromium.launch(visualBrowserLaunchOptions());
    page = await openFixturePage(browser, PORT, FIXTURE, {
      pageOptions: { viewport: { width: 512, height: 512 }, deviceScaleFactor: 2 },
    });

    const runtimeErrors = await mountBehavior(page);
    const resting = path.join(CURRENT_DIR, `${FIXTURE}-resting.png`);
    const engaged = path.join(CURRENT_DIR, `${FIXTURE}-engaged.png`);
    const restingBaseline = path.join(BASELINE_DIR, `${FIXTURE}-resting.png`);
    const engagedBaseline = path.join(BASELINE_DIR, `${FIXTURE}-engaged.png`);

    await captureCanvasPng(page, resting);
    await setBoundBoolean(page, true);
    await captureCanvasPng(page, engaged);

    if (runtimeErrors.length > 0) {
      throw new Error(`${FIXTURE}.riv runtime errors: ${runtimeErrors.join(" | ")}`);
    }

    if (fs.readFileSync(resting).equals(fs.readFileSync(engaged))) {
      throw new Error(
        "changing the authored view-model boolean did not produce a visible state-machine transition",
      );
    }

    if (update) {
      fs.copyFileSync(resting, restingBaseline);
      fs.copyFileSync(engaged, engagedBaseline);
      console.log("updated behavior runtime baselines");
    } else {
      assertSamePng(resting, restingBaseline, "resting");
      assertSamePng(engaged, engagedBaseline, "engaged");
      console.log("behavior view-model binding transitioned in the official Rive runtime");
    }
  } finally {
    if (page) {
      await page.close();
    }
    if (browser) {
      await browser.close();
    }
    if (server) {
      server.kill("SIGTERM");
    }
    cleanupFixtures([FIXTURE]);
  }
}

main().catch((error) => {
  console.error(error.message || error);
  process.exit(1);
});
