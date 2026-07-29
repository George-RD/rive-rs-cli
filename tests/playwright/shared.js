const { spawnSync, spawn } = require("node:child_process");
const fs = require("node:fs");
const http = require("node:http");
const path = require("node:path");

const ROOT = path.resolve(__dirname, "..", "..");
const HARNESS_DIR = path.join(ROOT, "tests", "playwright");
const OUT_DIR = path.join(ROOT, "target", "playwright-riv");
const SHOWCASE_DIR = path.join(ROOT, "showcase");

const BASE_FIXTURES = [
  "minimal",
  "shapes",
  "animation",
  "state_machine",
  "path",
  "cubic_easing",
  "trim_path",
  "nested_artboard",
  "multi_artboard",
  "artboard_preset",
  "gradients",
  "color_animation",
  "loop_animation",
  "stroke_styles",
  "empty_artboard",
  "icon_set",
  "game_hud",
  "mascot",
  "assets",
  "bones",
  "button_states",
  "blend_animation",
  "clipping_shape",
  "comparison_clip_tests",
  "comparison_official_test",
  "comparison_quantize_test",
  "comparison_trim",
  "cubic_asymmetric",
  "data_binding",
  "draw_rules",
  "elastic_interpolator",
  "event_test",
  "image_node",
  "joystick",
  "keyframe_types",
  "layout",
  "listener_test",
  "loader",
  "nested_simple_animation",
  "points_path",
  "polygon_star",
  "solo_test",
  "text",
  "text_modifiers",
  "triangle",
  "view_model_instances",
  "asset_extensions",
  "constraints",
  "data_converters",
  "effects",
  "events_extended",
  "follow_path_constraint",
  "graphics_misc",
  "layout_extensions",
  "mesh",
  "nested_extensions",
  "new_constraints",
  "nslicer",
  "scripting",
  "transition_comparators",
  "vertex_morph",
  "embedded_assets",
  "pointer_interaction",
];

const SHOWCASE_FIXTURES = fs
  .readdirSync(SHOWCASE_DIR, { withFileTypes: true })
  .filter((entry) => entry.isFile() && entry.name.endsWith(".json"))
  .map((entry) => `showcase_${path.basename(entry.name, ".json")}`)
  .sort();

const FIXTURES = [...BASE_FIXTURES, ...SHOWCASE_FIXTURES];
const ALL_FIXTURES = FIXTURES;

const KNOWN_RUNTIME_GAPS = new Set(["scripting", "transition_comparators"]);

const RUNTIME_ONLY_FIXTURES = new Set([
  "asset_extensions",
  "constraints",
  "data_converters",
  "effects",
  "events_extended",
  "follow_path_constraint",
  "graphics_misc",
  "layout_extensions",
  "mesh",
  "nested_extensions",
  "new_constraints",
  "nslicer",
  "scripting",
  "transition_comparators",
]);

const VISUAL_FIXTURES = ALL_FIXTURES.filter((fixture) => !RUNTIME_ONLY_FIXTURES.has(fixture));

function run(command, args, cwd = ROOT) {
  const result = spawnSync(command, args, { cwd, stdio: "inherit" });
  if (result.status !== 0) {
    throw new Error(`${command} ${args.join(" ")} failed with exit code ${result.status}`);
  }
}

function wait(delayMs) {
  return new Promise((resolve) => setTimeout(resolve, delayMs));
}

async function waitForServer(port, timeoutMs = 5000) {
  const deadline = Date.now() + timeoutMs;
  while (Date.now() < deadline) {
    try {
      await new Promise((resolve, reject) => {
        const request = http.get(
          { hostname: "127.0.0.1", port, path: "/tests/playwright/harness.html", timeout: 2000 },
          (response) => {
            response.resume();
            if (response.statusCode === 200) {
              resolve();
            } else {
              reject(new Error(`server returned status ${response.statusCode}`));
            }
          }
        );
        request.on("timeout", () => {
          request.destroy();
          reject(new Error("request timed out"));
        });
        request.on("error", reject);
      });
      return;
    } catch {
      await wait(100);
    }
  }
  throw new Error(`server did not start on port ${port}`);
}

function inputForFixture(fixture) {
  if (fixture.startsWith("showcase_")) {
    return path.join(SHOWCASE_DIR, `${fixture.slice("showcase_".length)}.json`);
  }
  return path.join(ROOT, "tests", "fixtures", `${fixture}.json`);
}

function buildFixtures(fixtures = ALL_FIXTURES) {
  fs.mkdirSync(OUT_DIR, { recursive: true });
  for (const fixture of fixtures) {
    const input = inputForFixture(fixture);
    const output = path.join(OUT_DIR, `${fixture}.riv`);
    run("cargo", ["run", "--quiet", "--", "generate", input, "-o", output]);
    fs.copyFileSync(output, path.join(HARNESS_DIR, `${fixture}.riv`));
  }
}

function startServer(port) {
  return spawn("python3", ["-m", "http.server", String(port), "--bind", "127.0.0.1"], {
    cwd: ROOT,
    stdio: "ignore",
  });
}

function cleanupFixtures(fixtures = ALL_FIXTURES) {
  for (const fixture of fixtures) {
    fs.rmSync(path.join(HARNESS_DIR, `${fixture}.riv`), { force: true });
  }
}

async function waitForRiveReady(page, timeout = 15000) {
  await page.waitForFunction(
    () => window.__RIVE_OK || window.__RIVE_ERROR,
    undefined,
    { timeout },
  );
}

async function captureCanvasPng(
  page,
  outputPath,
  { selector = "#canvas-controlled", background = "#0f172a", timeout = 15000 } = {},
) {
  const previousBackground = await page.evaluate(
    ({ selector: canvasSelector, background: fill }) => {
      const source = document.querySelector(canvasSelector);
      if (!(source instanceof HTMLCanvasElement)) {
        throw new Error(`missing canvas ${canvasSelector}`);
      }
      const previous = source.style.backgroundColor;
      source.style.backgroundColor = fill || "";
      return previous;
    },
    { selector, background },
  );

  let session;
  let timer;
  try {
    const box = await page.locator(selector).boundingBox();
    if (!box || box.width <= 0 || box.height <= 0) {
      throw new Error(`canvas ${selector} has no visible bounds`);
    }
    session = await page.context().newCDPSession(page);
    const options = {
      format: "png",
      fromSurface: true,
      clip: { x: box.x, y: box.y, width: box.width, height: box.height, scale: 1 },
    };
    const result = await Promise.race([
      session.send("Page.captureScreenshot", options),
      new Promise((_, reject) => {
        timer = setTimeout(
          () => reject(new Error(`timed out capturing canvas ${selector}`)),
          timeout,
        );
      }),
    ]);
    if (!result.data) {
      throw new Error("browser returned no PNG screenshot data");
    }
    fs.writeFileSync(outputPath, Buffer.from(result.data, "base64"));
  } finally {
    if (timer) {
      clearTimeout(timer);
    }
    if (session) {
      try {
        await session.detach();
      } catch {}
    }
    try {
      await page.evaluate(
        ({ selector: canvasSelector, background: fill }) => {
          const source = document.querySelector(canvasSelector);
          if (source instanceof HTMLCanvasElement) {
            source.style.backgroundColor = fill;
          }
        },
        { selector, background: previousBackground },
      );
    } catch {}
  }
}

async function openFixturePage(browser, port, fixture, { artboard, pageOptions } = {}) {
  const page = await browser.newPage(pageOptions);
  const runtimeErrors = [];
  page.on("pageerror", (err) => runtimeErrors.push(String(err)));
  page.on("console", (msg) => {
    if (msg.type() === "error") {
      runtimeErrors.push(msg.text());
    }
  });

  let url = `http://127.0.0.1:${port}/tests/playwright/harness.html?file=${fixture}.riv`;
  if (artboard) {
    url += `&artboard=${encodeURIComponent(artboard)}`;
  }

  await page.goto(url, { waitUntil: "domcontentloaded" });
  try {
    await waitForRiveReady(page);
  } catch (error) {
    const state = await page.evaluate(() => ({
      ok: window.__RIVE_OK,
      error: window.__RIVE_ERROR,
      readyState: document.readyState,
    }));
    const details = runtimeErrors.length > 0 ? `; ${runtimeErrors.join(" | ")}` : "";
    throw new Error(
      `${fixture}.riv did not report runtime readiness ` +
        `(ok=${state.ok}, error=${state.error || "none"}, document=${state.readyState})${details}: ` +
        `${error.message || error}`,
    );
  }

  const state = await page.evaluate(() => ({
    ok: window.__RIVE_OK,
    error: window.__RIVE_ERROR,
  }));

  if (runtimeErrors.length > 0) {
    throw new Error(`${fixture}.riv runtime errors: ${runtimeErrors.join(" | ")}`);
  }
  if (!state.ok || state.error) {
    throw new Error(`${fixture}.riv failed to load: ${state.error || "unknown error"}`);
  }

  return page;
}

module.exports = {
  ROOT,
  HARNESS_DIR,
  OUT_DIR,
  FIXTURES,
  KNOWN_RUNTIME_GAPS,
  SHOWCASE_FIXTURES,
  ALL_FIXTURES,
  VISUAL_FIXTURES,
  run,
  waitForServer,
  buildFixtures,
  startServer,
  cleanupFixtures,
  waitForRiveReady,
  captureCanvasPng,
  openFixturePage,
};
