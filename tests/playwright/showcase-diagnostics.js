const fs = require("node:fs/promises");
const path = require("node:path");

const DIAGNOSTIC_TIMEOUT_MS = 5000;

async function readShowcaseCards(page) {
  return page.evaluate(() =>
    Array.from(document.querySelectorAll(".card[data-showcase-id]")).map((card) => {
      const canvas = card.querySelector("canvas.scene");
      const timeline = window.__RIVE_SHOWCASE_TIMELINES?.get(card.dataset.showcaseId);
      let painted = 0;
      let paintError = null;
      try {
        const context = canvas?.getContext("2d");
        if (canvas && context) {
          const { data } = context.getImageData(0, 0, canvas.width, canvas.height);
          for (let i = 3; i < data.length; i += 4) {
            if (data[i] !== 0) painted += 1;
          }
        }
      } catch (error) {
        painted = null;
        paintError = String(error.message || error);
      }
      return {
        id: card.dataset.showcaseId,
        provenance: card.dataset.provenance ?? null,
        ready: card.dataset.playbackReady ?? null,
        playing: card.dataset.playing ?? null,
        frame: timeline?.currentFrame ?? null,
        reducedMotion: timeline?.reducedMotion ?? null,
        width: canvas?.width ?? null,
        height: canvas?.height ?? null,
        painted,
        paintError,
        aria: canvas?.getAttribute("aria-label") || "",
        source: card.querySelector('[data-source-link="true"]')?.getAttribute("href") || "",
        evidence: card.querySelector('[data-evidence-link="true"]')?.getAttribute("href") || "",
        consumerAttestation:
          card.querySelector('[data-consumer-attestation-link="true"]')?.getAttribute("href") || "",
        consumerEvidence:
          card.querySelector('[data-consumer-evidence-link="true"]')?.getAttribute("href") || "",
        text: card.textContent || "",
      };
    })
  );
}

async function retainShowcaseFailure({ page, directory, stage, error, errors }) {
  const report = {
    stage,
    error: {
      name: error?.name || "Error",
      message: String(error?.message || error),
      stack: error?.stack || null,
    },
    errors: [...errors],
    url: null,
    viewport: null,
    cards: [],
    screenshot: null,
    diagnosticErrors: [],
  };

  async function attempt(operation, action) {
    let timer;
    try {
      return await Promise.race([
        Promise.resolve().then(action),
        new Promise((_, reject) => {
          timer = setTimeout(
            () => reject(new Error(`${operation} diagnostic timed out after ${DIAGNOSTIC_TIMEOUT_MS}ms`)),
            DIAGNOSTIC_TIMEOUT_MS
          );
        }),
      ]);
    } catch (failure) {
      report.diagnosticErrors.push({ operation, message: String(failure.message || failure) });
      return null;
    } finally {
      clearTimeout(timer);
    }
  }

  await attempt("directory", () => fs.mkdir(directory, { recursive: true }));
  if (page) {
    const metadata = await attempt("page", () => ({ url: page.url(), viewport: page.viewportSize() }));
    if (metadata) Object.assign(report, metadata);
    report.cards = (await attempt("cards", () => readShowcaseCards(page))) || [];
    const screenshot = await attempt("screenshot", async () => {
      await page.screenshot({
        path: path.join(directory, "failure.png"),
        fullPage: true,
        timeout: DIAGNOSTIC_TIMEOUT_MS,
      });
      return "failure.png";
    });
    report.screenshot = screenshot;
  }
  await attempt("report", () =>
    fs.writeFile(path.join(directory, "failure.json"), `${JSON.stringify(report, null, 2)}\n`)
  );
  return report;
}

module.exports = { readShowcaseCards, retainShowcaseFailure };
