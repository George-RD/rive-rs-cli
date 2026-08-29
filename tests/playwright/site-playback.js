const { chromium } = require("playwright");
const { spawn } = require("node:child_process");
const fs = require("node:fs");
const http = require("node:http");
const path = require("node:path");

const ROOT = path.resolve(__dirname, "..", "..");
const PORT = Number(process.env.SITE_PLAYBACK_PORT || 8772);

function wait(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

async function waitForServer(port, timeoutMs = 20000) {
  const deadline = Date.now() + timeoutMs;
  while (Date.now() < deadline) {
    const ok = await new Promise((resolve) => {
      const request = http.get({ hostname: "127.0.0.1", port, path: "/lab.html", timeout: 2000 }, (res) => {
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

async function canvasSnapshots(card) {
  return card.locator("canvas.scene").evaluateAll((canvases) =>
    canvases.map((canvas) => canvas.toDataURL("image/png"))
  );
}

async function selectFrame(page, card, frame) {
  const id = await card.getAttribute("data-rung-id");
  await card.locator("select.playback-frame").selectOption(String(frame));
  await page.waitForFunction(
    ({ rungId, expected }) => {
      const target = document.querySelector(`[data-rung-id="${rungId}"]`);
      if (!target) return false;
      const status = target.querySelector(".playback-status")?.textContent || "";
      const toggle = target.querySelector("button.playback-toggle");
      return status.startsWith(`frame ${expected} `) && toggle?.textContent === "Play";
    },
    { rungId: id, expected: frame },
    { timeout: 10000 }
  );
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

    await page.goto(`http://127.0.0.1:${PORT}/lab.html`, { waitUntil: "load" });
    await page.waitForFunction(
      () => {
        const cards = Array.from(document.querySelectorAll(".card"));
        return cards.length === 2 && cards.every((card) => {
          const toggle = card.querySelector("button.playback-toggle");
          const frame = card.querySelector("select.playback-frame");
          const status = card.querySelector(".playback-status")?.textContent || "";
          return toggle && frame && !toggle.disabled && !frame.disabled && toggle.textContent === "Play" && status.startsWith("frame 0 ");
        });
      },
      null,
      { timeout: 20000 }
    );

    const coffeeCard = page.locator('.card[data-rung-id="coffee_loader"]');
    const coffeeToggle = coffeeCard.locator("button.playback-toggle");
    const initial = await canvasSnapshots(coffeeCard);

    await coffeeToggle.click();
    await page.waitForFunction(
      () => {
        const card = document.querySelector('.card[data-rung-id="coffee_loader"]');
        const toggle = card?.querySelector("button.playback-toggle");
        const status = card?.querySelector(".playback-status")?.textContent || "";
        const match = status.match(/^frame (\d+) /);
        return toggle?.textContent === "Pause" && Number(match?.[1] || 0) > 0;
      },
      null,
      { timeout: 10000 }
    );
    await wait(300);
    await coffeeToggle.click();
    await page.waitForFunction(
      () => document.querySelector('.card[data-rung-id="coffee_loader"] button.playback-toggle')?.textContent === "Play"
    );

    const paused = await canvasSnapshots(coffeeCard);
    for (let index = 0; index < initial.length; index += 1) {
      if (initial[index] === paused[index]) {
        errors.push(`shared play did not visibly advance coffee-loader canvas ${index + 1}`);
      }
    }
    await wait(300);
    const held = await canvasSnapshots(coffeeCard);
    if (JSON.stringify(paused) !== JSON.stringify(held)) {
      errors.push("shared pause did not hold both coffee-loader canvases stable");
    }

    await selectFrame(page, coffeeCard, 30);
    const firstThirty = await canvasSnapshots(coffeeCard);
    await selectFrame(page, coffeeCard, 45);
    await selectFrame(page, coffeeCard, 30);
    const secondThirty = await canvasSnapshots(coffeeCard);
    for (let index = 0; index < firstThirty.length; index += 1) {
      if (firstThirty[index] !== secondThirty[index]) {
        errors.push(`state-machine backward seek drifted on canvas ${index + 1}`);
      }
    }

    await selectFrame(page, coffeeCard, 15);
    const firstFifteen = await canvasSnapshots(coffeeCard);
    await selectFrame(page, coffeeCard, 30);
    await selectFrame(page, coffeeCard, 15);
    const secondFifteen = await canvasSnapshots(coffeeCard);
    if (JSON.stringify(firstFifteen) !== JSON.stringify(secondFifteen)) {
      errors.push("repeated paused seek to frame 15 was not stable");
    }

    const buttonCard = page.locator('.card[data-rung-id="button"]');
    await selectFrame(page, buttonCard, 30);
    const buttonThirty = await canvasSnapshots(buttonCard);
    if (buttonThirty.length !== 2 || buttonThirty[0] !== buttonThirty[1]) {
      errors.push("zero-difference button pair was not visibly frame-locked at representative frame 30");
    }

    const expectedById = Object.fromEntries(
      JSON.parse(fs.readFileSync(path.join(ROOT, "parity", "results.json"), "utf8")).map((rung) => [
        rung.id,
        rung.frames.map((frame) => frame.index),
      ])
    );
    const controls = await page.evaluate(() =>
      Array.from(document.querySelectorAll(".card")).map((card) => ({
        id: card.dataset.rungId,
        frames: Array.from(card.querySelectorAll("select.playback-frame option")).map((option) => Number(option.value)),
      }))
    );
    for (const control of controls) {
      if (JSON.stringify(control.frames) !== JSON.stringify(expectedById[control.id])) {
        errors.push(`representative frames for ${control.id} do not match parity/results.json`);
      }
    }

    if (errors.length > 0) {
      process.stdout.write(`Site playback validation failed:\n${errors.map((error) => `  ${error}`).join("\n")}\n`);
      shutdown();
      process.exit(1);
    }

    process.stdout.write("Site playback validation passed: shared play/pause, frame lock, representative seek, and stable backward seek\n");
    shutdown();
    process.exit(0);
  } catch (error) {
    process.stdout.write(`Site playback validation error: ${error.message}\n`);
    shutdown();
    process.exit(1);
  }
})();
