const { chromium } = require("playwright");
const { spawnSync, spawn } = require("node:child_process");
const fs = require("node:fs");
const path = require("node:path");

const ROOT = path.resolve(__dirname, "..", "..");
const HARNESS_DIR = path.join(ROOT, "tests", "playwright");
const OUT_DIR = path.join(ROOT, "target", "playwright-riv");
const SCREENSHOT_DIR = path.join(ROOT, "target", "playwright-snapshots");
const FIXTURES = ["minimal", "shapes", "animation", "state_machine", "path"];
const PORT = 8765;

function run(command, args, cwd = ROOT) {
  const result = spawnSync(command, args, { cwd, stdio: "inherit" });
  if (result.status !== 0) {
    throw new Error(`${command} ${args.join(" ")} failed with exit code ${result.status}`);
  }
}

async function main() {
  fs.mkdirSync(OUT_DIR, { recursive: true });
  fs.mkdirSync(SCREENSHOT_DIR, { recursive: true });

  for (const fixture of FIXTURES) {
    const input = path.join(ROOT, "tests", "fixtures", `${fixture}.json`);
    const output = path.join(OUT_DIR, `${fixture}.riv`);
    run("cargo", ["run", "--quiet", "--", "generate", input, "-o", output]);
    fs.copyFileSync(output, path.join(HARNESS_DIR, `${fixture}.riv`));
  }

  const server = spawn("python3", ["-m", "http.server", String(PORT), "--bind", "127.0.0.1"], {
    cwd: HARNESS_DIR,
    stdio: "ignore",
  });

  const browser = await chromium.launch({ headless: true });

  try {
    for (const fixture of FIXTURES) {
      const page = await browser.newPage();
      const runtimeErrors = [];
      page.on("pageerror", (err) => runtimeErrors.push(String(err)));
      page.on("console", (msg) => {
        if (msg.type() === "error") {
          runtimeErrors.push(msg.text());
        }
      });

      await page.goto(`http://127.0.0.1:${PORT}/harness.html?file=${fixture}.riv`, {
        waitUntil: "domcontentloaded",
      });
      await page.waitForTimeout(6000);

      const state = await page.evaluate(() => ({
        ok: window.__RIVE_OK,
        error: window.__RIVE_ERROR,
      }));

      await page.screenshot({ path: path.join(SCREENSHOT_DIR, `${fixture}.png`) });
      await page.close();

      if (runtimeErrors.length > 0) {
        throw new Error(`${fixture}.riv runtime errors: ${runtimeErrors.join(" | ")}`);
      }
      if (!state.ok || state.error) {
        throw new Error(`${fixture}.riv failed to load: ${state.error || "unknown error"}`);
      }
    }
  } finally {
    await browser.close();
    server.kill("SIGTERM");
    for (const fixture of FIXTURES) {
      const filePath = path.join(HARNESS_DIR, `${fixture}.riv`);
      if (fs.existsSync(filePath)) {
        fs.unlinkSync(filePath);
      }
    }
  }
}

main().catch((err) => {
  console.error(err.message || err);
  process.exit(1);
});
