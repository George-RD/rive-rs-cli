const { chromium } = require("playwright");
const { spawn } = require("node:child_process");
const fs = require("node:fs");
const http = require("node:http");
const path = require("node:path");

const ROOT = path.resolve(__dirname, "..", "..");
const PORT = Number(process.env.SITE_PORT || 8771);
const EXPECTED_SCENES = 4;
const LANDING_ARTIFACT = "examples/authoring/complex-animated-showcase.v0.riv";

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

function collectErrors(page, errors) {
  page.on("console", (message) => {
    if (message.type() === "error") errors.push(message.text());
  });
  page.on("pageerror", (error) => errors.push(String(error.message)));
  page.on("requestfailed", (request) =>
    errors.push(`request failed: ${request.url()} ${request.failure()?.errorText || ""}`)
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
    const errors = [];
    const page = await browser.newPage({ viewport: { width: 1280, height: 1000 } });
    collectErrors(page, errors);

    await page.goto(`http://127.0.0.1:${PORT}/`, { waitUntil: "load" });
    await page.waitForFunction(
      () => document.querySelector(".proof-parcel")?.dataset.playbackReady === "true",
      null,
      { timeout: 20000 }
    );
    await wait(800);
    const landing = await page.evaluate(() => {
      const canvas = document.querySelector(".hero-scene");
      const context = canvas.getContext("2d");
      const { data } = context.getImageData(0, 0, canvas.width, canvas.height);
      let painted = 0;
      for (let i = 3; i < data.length; i += 4) {
        if (data[i] !== 0) painted += 1;
      }
      const parcel = document.querySelector(".proof-parcel");
      return {
        painted,
        artifact: parcel?.dataset.artifact || "",
        primaryHref: document.querySelector(".hero-actions .button-primary")?.getAttribute("href") || "",
        labHref: document.querySelector(".proof-strip a[href=\"lab.html\"]")?.getAttribute("href") || "",
        pageText: document.body.textContent || "",
      };
    });
    if (landing.painted === 0) errors.push("landing hero canvas rendered nothing");
    if (landing.artifact !== LANDING_ARTIFACT) {
      errors.push(`landing proof uses ${landing.artifact || "no artifact"}, expected ${LANDING_ARTIFACT}`);
    }
    if (landing.primaryHref !== "showcase.html") {
      errors.push(`landing primary creation path points to ${landing.primaryHref || "nothing"}`);
    }
    if (landing.labHref !== "lab.html") {
      errors.push("landing page does not preserve a separate Verification Lab path");
    }
    if (landing.pageText.includes("coffee_loader.riv")) {
      errors.push("landing page still presents the coffee-loader reproduction as creation proof");
    }

    const phone = await browser.newPage({ viewport: { width: 390, height: 844 } });
    collectErrors(phone, errors);
    await phone.goto(`http://127.0.0.1:${PORT}/`, { waitUntil: "load" });
    await phone.waitForFunction(
      () => document.querySelector(".proof-parcel")?.dataset.playbackReady === "true",
      null,
      { timeout: 20000 }
    );
    const phoneOverflow = await phone.evaluate(
      () => document.documentElement.scrollWidth - window.innerWidth
    );
    if (phoneOverflow > 1) errors.push(`landing overflows phone viewport by ${phoneOverflow}px`);

    const reduced = await browser.newPage({ viewport: { width: 900, height: 800 } });
    await reduced.emulateMedia({ reducedMotion: "reduce" });
    collectErrors(reduced, errors);
    await reduced.goto(`http://127.0.0.1:${PORT}/`, { waitUntil: "load" });
    await reduced.waitForFunction(
      () => document.querySelector(".proof-parcel")?.dataset.playbackReady === "true",
      null,
      { timeout: 20000 }
    );
    const reducedPlaying = await reduced.evaluate(
      () => document.querySelector(".proof-parcel")?.dataset.playing
    );
    if (reducedPlaying === "true") errors.push("reduced-motion landing autoplayed");

    await page.goto(`http://127.0.0.1:${PORT}/lab.html`, { waitUntil: "load" });
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

    const reported = await page.evaluate(() =>
      Array.from(document.querySelectorAll(".card")).map((card) => ({
        title: card.querySelector("h3").textContent,
        canvases: card.querySelectorAll("canvas.scene").length,
        maxPixelDifference: card.querySelector(".metrics dd").textContent,
      }))
    );
    const expected = JSON.parse(
      fs.readFileSync(path.join(ROOT, "parity", "results.json"), "utf8")
    ).map((rung) => ({
      title: rung.title,
      canvases: 2,
      maxPixelDifference: `${rung.max_pixel_difference.toFixed(4)}%`,
    }));

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
    if (JSON.stringify(reported) !== JSON.stringify(expected)) {
      process.stdout.write(
        `the gallery does not report the measured parity results\n  page:     ${JSON.stringify(reported)}\n  expected: ${JSON.stringify(expected)}\n`
      );
      failed = true;
    }
    if (errors.length > 0) {
      process.stdout.write(`validation errors:\n${errors.map((e) => `  ${e}`).join("\n")}\n`);
      failed = true;
    }

    process.stdout.write(
      failed ? "Site validation failed\n" : `Site validation passed: original landing proof, responsive/reduced-motion landing, ${painted.length} lab scenes, 0 console errors\n`
    );
    shutdown();
    process.exit(failed ? 1 : 0);
  } catch (error) {
    process.stdout.write(`Site validation error: ${error.message}\n`);
    shutdown();
    process.exit(1);
  }
})();
