const { chromium } = require("playwright");
const { spawn } = require("node:child_process");
const http = require("node:http");
const {
  plan,
  showcaseEntries,
  assertShowcaseEntriesExist,
  ROOT,
} = require("../../site/stage");

const PORT = Number(process.env.SHOWCASE_PORT || 8773);
const POLLING_MS = 50;
const PRODUCTION_ID = "horaxon-signal-to-action";
const INTERACTIVE_ID = "throughput-console";
const CONSOLE_ARTBOARD_WIDTH = 960;
const CONSOLE_ARTBOARD_HEIGHT = 540;
const NEEDLE_ONLY_SCAN_ROW = 328;
const STANDBY_NEEDLE_MAX_X = 288;
const ENGAGED_NEEDLE_MIN_X = 576;
const NEEDLE_ROW_FRACTION = NEEDLE_ONLY_SCAN_ROW / CONSOLE_ARTBOARD_HEIGHT;
const STANDBY_NEEDLE_LIMIT = STANDBY_NEEDLE_MAX_X / CONSOLE_ARTBOARD_WIDTH;
const ENGAGED_NEEDLE_FLOOR = ENGAGED_NEEDLE_MIN_X / CONSOLE_ARTBOARD_WIDTH;
const SETTLE_FRAMES = 120;
const LIFECYCLE_TIMEOUT_MS = 20000;
const NEEDLE_TIMEOUT_MS = 15000;

function wait(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

async function waitForServer(port, timeoutMs = 20000) {
  const deadline = Date.now() + timeoutMs;
  while (Date.now() < deadline) {
    const ok = await new Promise((resolve) => {
      const request = http.get(
        { hostname: "127.0.0.1", port, path: "/showcase.html", timeout: 2000 },
        (res) => {
          res.resume();
          resolve(res.statusCode === 200);
        }
      );
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

function assertStagingContract(entries) {
  const staged = new Set(plan().map(([, to]) => to));
  for (const entry of entries) {
    for (const field of [
      "artifact",
      "source",
      "evidence",
      "consumerAttestation",
      "consumerEvidence",
    ]) {
      if (entry[field] && !staged.has(entry[field])) {
        throw new Error(`staging plan omitted ${entry.id} ${field} ${entry[field]}`);
      }
    }
  }

  const production = entries.find((entry) => entry.id === PRODUCTION_ID);
  if (!production || production.provenance !== "production") {
    throw new Error("Horaxon production proof is missing its production provenance type");
  }
  for (const field of [
    "artifact",
    "source",
    "evidence",
    "consumerAttestation",
    "consumerEvidence",
  ]) {
    if (!production[field] || /^https?:\/\//.test(production[field])) {
      throw new Error(`Horaxon ${field} is not a retained local path: ${production[field] || "missing"}`);
    }
  }

  let namedFailure = "";
  try {
    assertShowcaseEntriesExist([
      {
        id: "missing-contract",
        provenance: "production",
        artifact: "missing/showcase.riv",
        source: "missing/showcase.json",
      },
    ]);
  } catch (error) {
    namedFailure = String(error.message || error);
  }
  if (!namedFailure.includes("missing-contract.artifact: missing/showcase.riv")) {
    throw new Error(`missing showcase artifact error was not named: ${namedFailure}`);
  }
  if (!namedFailure.includes("missing-contract.evidence is missing")) {
    throw new Error(`missing production evidence error was not named: ${namedFailure}`);
  }
  if (!namedFailure.includes("missing-contract.consumerAttestation is missing")) {
    throw new Error(`missing production consumer attestation error was not named: ${namedFailure}`);
  }
  if (!namedFailure.includes("missing-contract.consumerEvidence is missing")) {
    throw new Error(`missing production consumer evidence error was not named: ${namedFailure}`);
  }
}

async function needleFraction(page, id) {
  return page.evaluate(
    ({ id, rowFraction }) => {
      const card = document.querySelector(`[data-showcase-id="${id}"]`);
      const canvas = card && card.querySelector("canvas.scene");
      const context = canvas && canvas.getContext("2d");
      if (!canvas || !context) return null;
      const row = Math.round(canvas.height * rowFraction);
      const { data } = context.getImageData(0, row, canvas.width, 1);
      let total = 0;
      let count = 0;
      for (let column = 0; column < canvas.width; column += 1) {
        const offset = column * 4;
        if (data[offset] > 200 && data[offset + 1] > 200 && data[offset + 2] > 200) {
          total += column;
          count += 1;
        }
      }
      return count === 0 ? null : total / count / canvas.width;
    },
    { id, rowFraction: NEEDLE_ROW_FRACTION }
  );
}

async function waitForNeedle(page, compare, bound) {
  const deadline = Date.now() + NEEDLE_TIMEOUT_MS;
  while (Date.now() < deadline) {
    let fraction = null;
    try {
      fraction = await needleFraction(page, INTERACTIVE_ID);
    } catch {
      fraction = null;
    }
    if (fraction !== null && (compare === "above" ? fraction >= bound : fraction <= bound)) {
      return true;
    }
    await wait(POLLING_MS);
  }
  return false;
}

async function waitForPlayingState(page, expected, label, pageErrors = []) {
  try {
    await page.waitForFunction(
      (expected) =>
        Array.from(document.querySelectorAll(".card[data-showcase-id]")).every(
          (card) => card.dataset.playing === expected
        ),
      expected,
      { timeout: LIFECYCLE_TIMEOUT_MS, polling: POLLING_MS }
    );
  } catch {
    const observed = await page.evaluate(() =>
      Array.from(document.querySelectorAll(".card[data-showcase-id]")).map((card) => ({
        id: card.dataset.showcaseId,
        playing: card.dataset.playing,
        ready: card.dataset.playbackReady,
      }))
    );
    const reported = pageErrors.length ? ` page errors: ${JSON.stringify(pageErrors)}` : "";
    throw new Error(
      `${label} did not reach data-playing="${expected}" within ${LIFECYCLE_TIMEOUT_MS}ms: ${JSON.stringify(observed)}.${reported}`
    );
  }
}

async function pauseTimeline(page, id) {
  await page.evaluate(async (id) => {
    const timeline = window.__RIVE_SHOWCASE_TIMELINES?.get(id);
    if (timeline) await timeline.pause();
  }, id);
}

async function advanceTimeline(page, id, frames) {
  await page.evaluate(
    async ({ id, frames }) => {
      const timeline = window.__RIVE_SHOWCASE_TIMELINES?.get(id);
      if (!timeline) return;
      await timeline.seekToFrame(timeline.currentFrame + frames);
    },
    { id, frames }
  );
}

async function readRuntimeInputs(page, id) {
  return page.evaluate(
    (id) => window.__RIVE_SHOWCASE_TIMELINES?.get(id)?.readInputs() || null,
    id
  );
}

async function waitForRuntimeInput(page, id, name, expected) {
  try {
    await page.waitForFunction(
      ({ id, name, expected }) => {
        const values = window.__RIVE_SHOWCASE_TIMELINES?.get(id)?.readInputs();
        return Boolean(values) && values[name] === expected;
      },
      { id, name, expected },
      { timeout: 15000, polling: POLLING_MS }
    );
    return true;
  } catch {
    return false;
  }
}

async function driveInteractiveControls(page, errors) {
  const card = page.locator(`[data-showcase-id="${INTERACTIVE_ID}"]`);
  if ((await card.count()) === 0) {
    errors.push(`${INTERACTIVE_ID} card was not rendered`);
    return;
  }
  if ((await card.locator("[data-scene-controls]").count()) === 0) {
    errors.push(`${INTERACTIVE_ID} card has no state-machine controls`);
    return;
  }

  await pauseTimeline(page, INTERACTIVE_ID);

  if (!(await waitForNeedle(page, "below", STANDBY_NEEDLE_LIMIT))) {
    errors.push(
      `${INTERACTIVE_ID} standby needle sat at ${await needleFraction(page, INTERACTIVE_ID)}`
    );
  }

  const loadInput = await card
    .locator('[data-control-kind="range"]')
    .getAttribute("data-control-input");
  const armedInput = await card
    .locator('[data-control-kind="toggle"]')
    .getAttribute("data-control-input");

  const slider = card.locator('[data-control-kind="range"] input[type="range"]');
  await slider.evaluate((node) => {
    node.value = "100";
    node.dispatchEvent(new Event("input", { bubbles: true }));
  });
  await card.locator('[data-control-kind="toggle"] input[type="checkbox"]').check();

  if (!(await waitForRuntimeInput(page, INTERACTIVE_ID, loadInput, 100))) {
    errors.push(
      `${INTERACTIVE_ID} slider did not reach the runtime; inputs were ${JSON.stringify(
        await readRuntimeInputs(page, INTERACTIVE_ID)
      )}`
    );
  }
  if (!(await waitForRuntimeInput(page, INTERACTIVE_ID, armedInput, true))) {
    errors.push(
      `${INTERACTIVE_ID} toggle did not reach the runtime; inputs were ${JSON.stringify(
        await readRuntimeInputs(page, INTERACTIVE_ID)
      )}`
    );
  }

  await advanceTimeline(page, INTERACTIVE_ID, SETTLE_FRAMES);

  if (!(await waitForNeedle(page, "above", ENGAGED_NEEDLE_FLOOR))) {
    errors.push(
      `${INTERACTIVE_ID} armed needle sat at ${await needleFraction(
        page,
        INTERACTIVE_ID
      )} with inputs ${JSON.stringify(await readRuntimeInputs(page, INTERACTIVE_ID))}`
    );
  }

  await card.locator('[data-control-kind="trigger"] button').click();
  await waitForRuntimeInput(page, INTERACTIVE_ID, armedInput, false);
  await advanceTimeline(page, INTERACTIVE_ID, SETTLE_FRAMES);

  if (!(await waitForNeedle(page, "below", STANDBY_NEEDLE_LIMIT))) {
    errors.push(
      `${INTERACTIVE_ID} reset trigger left the needle at ${await needleFraction(
        page,
        INTERACTIVE_ID
      )} with inputs ${JSON.stringify(await readRuntimeInputs(page, INTERACTIVE_ID))}`
    );
  }
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

async function readCards(page) {
  return page.evaluate(() =>
    Array.from(document.querySelectorAll(".card[data-showcase-id]")).map((card) => {
      const canvas = card.querySelector("canvas.scene");
      const context = canvas?.getContext("2d");
      let painted = 0;
      if (canvas && context) {
        const { data } = context.getImageData(0, 0, canvas.width, canvas.height);
        for (let i = 3; i < data.length; i += 4) {
          if (data[i] !== 0) painted += 1;
        }
      }
      return {
        id: card.dataset.showcaseId,
        provenance: card.dataset.provenance,
        ready: card.dataset.playbackReady,
        playing: card.dataset.playing,
        painted,
        aria: canvas?.getAttribute("aria-label") || "",
        source: card.querySelector("[data-source-link=\"true\"]")?.getAttribute("href") || "",
        evidence: card.querySelector("[data-evidence-link=\"true\"]")?.getAttribute("href") || "",
        consumerAttestation:
          card.querySelector("[data-consumer-attestation-link=\"true\"]")?.getAttribute("href") || "",
        consumerEvidence:
          card.querySelector("[data-consumer-evidence-link=\"true\"]")?.getAttribute("href") || "",
        text: card.textContent || "",
      };
    })
  );
}

async function readEvidenceStatuses(page) {
  return page.evaluate(async () =>
    Promise.all(
      Array.from(
        document.querySelectorAll(
          "[data-source-link=\"true\"], [data-evidence-link=\"true\"], [data-consumer-attestation-link=\"true\"], [data-consumer-evidence-link=\"true\"]"
        )
      ).map(async (link) => {
        const response = await fetch(link.href);
        return { href: link.getAttribute("href"), status: response.status };
      })
    )
  );
}

(async () => {
  const entries = showcaseEntries();
  assertStagingContract(entries);

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
    await page.goto(`http://127.0.0.1:${PORT}/showcase.html`, { waitUntil: "load" });
    await page.waitForFunction(
      (expected) => {
        const cards = Array.from(document.querySelectorAll(".card[data-showcase-id]"));
        return (
          cards.length === expected &&
          cards.every((card) => card.dataset.playbackReady === "true")
        );
      },
      entries.length,
      { timeout: 20000, polling: POLLING_MS }
    );
    await page.waitForFunction(
      () =>
        Array.from(document.querySelectorAll(".card[data-showcase-id]")).every((card) => {
          const canvas = card.querySelector("canvas.scene");
          const context = canvas?.getContext("2d");
          if (!canvas || !context) return false;
          const { data } = context.getImageData(0, 0, canvas.width, canvas.height);
          for (let index = 3; index < data.length; index += 4) {
            if (data[index] !== 0) return true;
          }
          return false;
        }),
      undefined,
      { timeout: 30000, polling: POLLING_MS }
    );

    const cards = await readCards(page);
    const expectedIds = entries.map((entry) => entry.id);
    if (JSON.stringify(cards.map((card) => card.id)) !== JSON.stringify(expectedIds)) {
      errors.push(`manifest/card ids diverged: ${JSON.stringify(cards.map((card) => card.id))}`);
    }
    for (const card of cards) {
      if (card.painted === 0) errors.push(`${card.id} rendered nothing`);
      if (card.playing !== "true") errors.push(`${card.id} did not enter live playback`);
      if (!card.aria) errors.push(`${card.id} is missing a canvas text alternative`);
      if (!card.source) errors.push(`${card.id} is missing a retained source link`);
    }

    const productionCard = cards.find((card) => card.id === PRODUCTION_ID);
    if (!productionCard) {
      errors.push("Horaxon production card was not rendered");
    } else {
      if (productionCard.provenance !== "production") {
        errors.push(`Horaxon card provenance is ${productionCard.provenance || "missing"}`);
      }
      if (!productionCard.evidence) errors.push("Horaxon card is missing its provenance link");
      if (!productionCard.consumerAttestation) {
        errors.push("Horaxon card is missing its consumer attestation link");
      }
      if (!productionCard.consumerEvidence) {
        errors.push("Horaxon card is missing its retained consumer evidence link");
      }
      if (!productionCard.text.includes("Production consumer")) {
        errors.push("Horaxon card is not visibly labelled as production consumer proof");
      }
      if (!productionCard.text.includes("not a customer endorsement")) {
        errors.push("Horaxon card does not preserve the endorsement claim boundary");
      }
    }

    await wait(1500);
    const decisionFlowPlaying = await page.evaluate(
      () => document.querySelector('[data-showcase-id="decision-flow"]')?.dataset.playing
    );
    if (decisionFlowPlaying !== "true") {
      errors.push("decision-flow one-shot did not restart as an intentional showcase loop");
    }

    await driveInteractiveControls(page, errors);

    const evidenceStatuses = await readEvidenceStatuses(page);
    for (const evidence of evidenceStatuses) {
      if (evidence.status !== 200) {
        errors.push(`source/provenance link ${evidence.href} returned HTTP ${evidence.status}`);
      }
    }

    const lifecycle = await browser.newPage({ viewport: { width: 1280, height: 1000 } });
    const lifecycleErrors = [];
    collectErrors(lifecycle, errors);
    collectErrors(lifecycle, lifecycleErrors);
    await lifecycle.goto(`http://127.0.0.1:${PORT}/showcase.html`, { waitUntil: "load" });
    await lifecycle.waitForFunction(
      (expected) => {
        const cards = Array.from(document.querySelectorAll(".card[data-showcase-id]"));
        return (
          cards.length === expected &&
          cards.every(
            (card) =>
              card.dataset.playbackReady === "true" && card.dataset.playing === "true"
          )
        );
      },
      entries.length,
      { timeout: 20000, polling: POLLING_MS }
    );
    await lifecycle.evaluate(() => {
      window.dispatchEvent(new PageTransitionEvent("pagehide", { persisted: true }));
    });
    await waitForPlayingState(lifecycle, "false", "bfcache pause", lifecycleErrors);
    await lifecycle.evaluate(() => {
      window.dispatchEvent(new PageTransitionEvent("pageshow", { persisted: true }));
    });
    await waitForPlayingState(lifecycle, "true", "bfcache resume", lifecycleErrors);
    await lifecycle.close();

    const phone = await browser.newPage({ viewport: { width: 390, height: 844 } });
    collectErrors(phone, errors);
    await phone.goto(`http://127.0.0.1:${PORT}/showcase.html`, { waitUntil: "load" });
    await phone.waitForFunction(
      (expected) => document.querySelectorAll(".card[data-showcase-id]").length === expected,
      entries.length,
      { timeout: 20000 }
    );
    const phoneOverflow = await phone.evaluate(
      () => document.documentElement.scrollWidth - window.innerWidth
    );
    if (phoneOverflow > 1) errors.push(`showcase overflows phone viewport by ${phoneOverflow}px`);

    const reduced = await browser.newPage({ viewport: { width: 900, height: 800 } });
    await reduced.emulateMedia({ reducedMotion: "reduce" });
    collectErrors(reduced, errors);
    await reduced.goto(`http://127.0.0.1:${PORT}/showcase.html`, { waitUntil: "load" });
    await reduced.waitForFunction(
      (expected) => {
        const cards = Array.from(document.querySelectorAll(".card[data-showcase-id]"));
        return (
          cards.length === expected &&
          cards.every((card) => card.dataset.playbackReady === "true")
        );
      },
      entries.length,
      { timeout: 20000, polling: POLLING_MS }
    );
    const reducedPlaying = await reduced.evaluate(() =>
      Array.from(document.querySelectorAll(".card[data-showcase-id]"))
        .filter((card) => card.dataset.playing === "true")
        .map((card) => card.dataset.showcaseId)
    );
    if (reducedPlaying.length > 0) {
      errors.push(`reduced-motion page autoplayed: ${reducedPlaying.join(", ")}`);
    }

    if (errors.length > 0) {
      process.stdout.write(
        `Showcase validation failed:\n${errors.map((error) => `  ${error}`).join("\n")}\n`
      );
      shutdown();
      process.exit(1);
    }

    process.stdout.write(
      `Showcase validation passed: ${entries.length} manifest-driven cards including local Horaxon production provenance, hashed consumer evidence, bounded one-shot looping, live runtime paint, state-machine input controls, bfcache pause/resume, phone layout, reduced motion\n`
    );
    shutdown();
    process.exit(0);
  } catch (error) {
    process.stdout.write(`Showcase validation error: ${error.message}\n`);
    shutdown();
    process.exit(1);
  }
})();
