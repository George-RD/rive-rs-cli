const { chromium } = require("playwright");
const { spawn } = require("node:child_process");
const http = require("node:http");
const path = require("node:path");

const ROOT = path.resolve(__dirname, "..", "..");
const PORT = Number(process.env.SITE_PORT || 8771);
const EXPECTED_SCENES = 11;

function wait(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

async function waitForServer(port, timeoutMs = 20000) {
  const deadline = Date.now() + timeoutMs;
  while (Date.now() < deadline) {
    const ok = await new Promise((resolve) => {
      const request = http.get({ hostname: "127.0.0.1", port, path: "/", timeout: 2000 }, (res) => {
        res.resume();
        resolve(res.statusCode === 200);
      });
      request.on("timeout", () => {
        request.destroy();
        resolve(false);
      });
      request.on("error", () => resolve(false));
    });
    if (ok) return;
    await wait(150);
  }
  throw new Error(`site server did not start on port ${port}`);
}

(async () => {
  const server = spawn("node", ["site/serve.js"], {
    cwd: ROOT,
    env: { ...process.env, SITE_PORT: String(PORT) },
    stdio: "ignore",
  });
  let browser;
  const shutdown = () => {
    if (browser) browser.close().catch(() => {});
    server.kill("SIGTERM");
  };
  process.on("exit", shutdown);

  try {
    await waitForServer(PORT);
    browser = await chromium.launch();
    const page = await browser.newPage({ viewport: { width: 1280, height: 1000 } });
    const errors = [];
    page.on("console", (message) => {
      if (message.type() === "error") errors.push(message.text());
    });
    page.on("pageerror", (error) => errors.push(String(error.message)));
    page.on("requestfailed", (request) =>
      errors.push(`request failed: ${request.url()} ${request.failure()?.errorText || ""}`)
    );

    await page.goto(`http://127.0.0.1:${PORT}/`, { waitUntil: "load" });
    await page.waitForFunction(
      (expected) => document.querySelectorAll("canvas.scene").length >= expected,
      EXPECTED_SCENES,
      { timeout: 20000 }
    );
    await wait(3500);

    const painted = await page.evaluate(() =>
      Array.from(document.querySelectorAll("canvas.scene")).map((canvas) => {
        const context = canvas.getContext("2d");
        if (!context) return { label: canvas.getAttribute("aria-label"), painted: 0 };
        const { data } = context.getImageData(0, 0, canvas.width, canvas.height);
        let painted = 0;
        for (let i = 3; i < data.length; i += 4) {
          if (data[i] !== 0) painted += 1;
        }
        return { label: canvas.getAttribute("aria-label"), painted, total: data.length / 4 };
      })
    );

    const blank = painted.filter((entry) => entry.painted === 0);
    const transcript = await page.textContent("#transcript");

    const snapshot = async () =>
      page.evaluate(() => {
        const canvas = Array.from(document.querySelectorAll("canvas.scene")).find(
          (node) => node.getAttribute("aria-label") === "Control Panel animation"
        );
        const context = canvas.getContext("2d");
        const { data } = context.getImageData(0, 0, canvas.width, canvas.height);
        let sum = 0;
        for (let i = 0; i < data.length; i += 40) sum += data[i];
        return sum;
      });

    const cardFor = (label) =>
      page.locator(".card", { has: page.locator(`canvas[aria-label="${label}"]`) });
    const panel = cardFor("Control Panel animation");
    const before = await snapshot();
    await panel.locator('input[type="range"]').fill("100");
    await wait(1200);
    const afterLevel = await snapshot();
    await panel.getByRole("button", { name: "Press button" }).click();
    await wait(400);
    const afterPress = await snapshot();

    for (const entry of painted) {
      const pct = ((entry.painted / entry.total) * 100).toFixed(1);
      process.stdout.write(`${entry.painted > 0 ? "PASS" : "FAIL"} ${entry.label} (${pct}% painted)\n`);
    }

    let failed = false;
    if (blank.length > 0) {
      process.stdout.write(`${blank.length} scene(s) rendered nothing\n`);
      failed = true;
    }
    if (!transcript || !transcript.includes("non-background bounds")) {
      process.stdout.write("verification transcript did not load\n");
      failed = true;
    }
    if (afterLevel === before) {
      process.stdout.write("the level slider did not change the control panel render\n");
      failed = true;
    }
    if (afterPress === afterLevel) {
      process.stdout.write("pressing the button did not change the control panel render\n");
      failed = true;
    }
    if (errors.length > 0) {
      process.stdout.write(`console errors:\n${errors.map((e) => `  ${e}`).join("\n")}\n`);
      failed = true;
    }

    process.stdout.write(
      failed ? "Site validation failed\n" : `Site validation passed: ${painted.length} scenes, 0 console errors\n`
    );
    shutdown();
    process.exit(failed ? 1 : 0);
  } catch (error) {
    process.stdout.write(`Site validation error: ${error.message}\n`);
    shutdown();
    process.exit(1);
  }
})();
