const { chromium } = require("playwright");
const { spawn } = require("node:child_process");
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
    canvases.map((canvas) => ({
      frame: Number(canvas.dataset.logicalFrame),
      image: canvas.toDataURL("image/png"),
    }))
  );
}

async function selectFrame(card, frame) {
  await card.locator("select.playback-frame").selectOption(String(frame));
  await card.page().waitForFunction(
    ({ id, expected }) => {
      const target = document.querySelector(`[data-rung-id="${id}"]`);
      if (!target) return false;
      const canvases = Array.from(target.querySelectorAll("canvas.scene"));
      return canvases.length === 2 && canvases.every((canvas) => Number(canvas.dataset.logicalFrame) === expected);
    },
    { id: await card.getAttribute("data-rung-id"), expected: frame },
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
        const cards = Array.from(document.querySelectorAll(".card[data-playback-ready='true']"));
        return cards.length === 2 && cards.every((card) => card.querySelectorAll("canvas.scene").length === 2);
      },
      null,
      { timeout: 20000 }
    );

    const firstCard = page.locator(".card").first();
    const firstToggle = firstCard.locator("button.playback-toggle");
    if ((await firstToggle.textContent()) !== "Pause") {
      errors.push("shared playback did not begin after both canvases became ready");
    }

    await firstToggle.click();
    await page.waitForFunction(
      () => document.querySelector(".card button.playback-toggle")?.textContent === "Play"
    );
    const pausedFrames = await firstCard.locator("canvas.scene").evaluateAll((canvases) =>
      canvases.map((canvas) => Number(canvas.dataset.logicalFrame))
    );
    await wait(300);
    const heldFrames = await firstCard.locator("canvas.scene").evaluateAll((canvases) =>
      canvases.map((canvas) => Number(canvas.dataset.logicalFrame))
    );
    if (new Set(pausedFrames).size !== 1 || JSON.stringify(pausedFrames) !== JSON.stringify(heldFrames)) {
      errors.push(`pause did not hold one shared logical frame: ${pausedFrames} -> ${heldFrames}`);
    }

    await firstToggle.click();
    await wait(300);
    await firstToggle.click();
    const resumedFrames = await firstCard.locator("canvas.scene").evaluateAll((canvases) =>
      canvases.map((canvas) => Number(canvas.dataset.logicalFrame))
    );
    if (new Set(resumedFrames).size !== 1 || resumedFrames[0] <= heldFrames[0]) {
      errors.push(`play did not advance both canvases together: ${heldFrames} -> ${resumedFrames}`);
    }

    await selectFrame(firstCard, 30);
    const selectedFrames = await firstCard.locator("canvas.scene").evaluateAll((canvases) =>
      canvases.map((canvas) => Number(canvas.dataset.logicalFrame))
    );
    if (selectedFrames.some((frame) => frame !== 30)) {
      errors.push(`representative frame seek did not frame-lock the pair: ${selectedFrames}`);
    }

    const coffeeCard = page.locator('.card[data-rung-id="coffee_loader"]');
    const coffeeToggle = coffeeCard.locator("button.playback-toggle");
    if ((await coffeeToggle.textContent()) === "Pause") {
      await coffeeToggle.click();
    }
    await selectFrame(coffeeCard, 15);
    const firstFifteen = await canvasSnapshots(coffeeCard);
    await selectFrame(coffeeCard, 45);
    await selectFrame(coffeeCard, 15);
    const secondFifteen = await canvasSnapshots(coffeeCard);
    for (let index = 0; index < firstFifteen.length; index += 1) {
      if (firstFifteen[index].frame !== 15 || secondFifteen[index].frame !== 15) {
        errors.push("backward seek did not return the state-machine pair to frame 15");
        break;
      }
      if (firstFifteen[index].image !== secondFifteen[index].image) {
        errors.push(`state-machine backward seek drifted on canvas ${index + 1}`);
      }
    }

    const controls = await page.evaluate(() =>
      Array.from(document.querySelectorAll(".card")).map((card) => ({
        id: card.dataset.rungId,
        ready: card.dataset.playbackReady,
        frames: Array.from(card.querySelectorAll("select.playback-frame option")).map((option) => Number(option.value)),
      }))
    );
    const expectedFrames = [0, 15, 30, 45];
    if (controls.some((control) => control.ready !== "true" || JSON.stringify(control.frames) !== JSON.stringify(expectedFrames))) {
      errors.push(`playback controls are not derived from parity representative frames: ${JSON.stringify(controls)}`);
    }

    if (errors.length > 0) {
      process.stdout.write(`Site playback validation failed:\n${errors.map((error) => `  ${error}`).join("\n")}\n`);
      shutdown();
      process.exit(1);
    }

    process.stdout.write("Site playback validation passed: shared play/pause, representative seek, and stable backward seek\n");
    shutdown();
    process.exit(0);
  } catch (error) {
    process.stdout.write(`Site playback validation error: ${error.message}\n`);
    shutdown();
    process.exit(1);
  }
})();
