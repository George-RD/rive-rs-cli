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
const NEEDLE_ROW_FRACTION = 0.607;
const STANDBY_NEEDLE_LIMIT = 0.3;
const ENGAGED_NEEDLE_FLOOR = 0.6;

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
      const canvas = card?.querySelector("canvas.scene");
      const context = canvas?.getContext("2d");
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

  const standby = await needleFraction(page, INTERACTIVE_ID);
  if (standby === null || standby > STANDBY_NEEDLE_LIMIT) {
    errors.push(`${INTERACTIVE_ID} standby needle sat at ${standby}`);
  }

  const slider = card.locator('[data-control-kind="range"] input[type="range"]');
  await slider.evaluate((node) => {
    node.value = "100";
    node.dispatchEvent(new Event("input", { bubbles: true }));
  });
  await card.locator('[data-control-kind="toggle"] input[type="checkbox"]').check();
  await wait(900);

  const engaged = await needleFraction(page, INTERACTIVE_ID);
  if (engaged === null || engaged < ENGAGED_NEEDLE_FLOOR) {
    errors.push(`${INTERACTIVE_ID} armed needle sat at ${engaged}, expected the blended load`);
  }

  await card.locator('[data-control-kind="trigger"] button').click();
  await wait(900);

  const released = await needleFraction(page, INTERACTIVE_ID);
  if (released === null || released > STANDBY_NEEDLE_LIMIT) {
    errors.push(`${INTERACTIVE_ID} reset trigger left the needle at ${released}`);
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
    await wait(800);

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
    collectErrors(lifecycle, errors);
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
    await lifecycle.waitForFunction(
      () =>
        Array.from(document.querySelectorAll(".card[data-showcase-id]")).every(
          (card) => card.dataset.playing === "false"
        ),
      undefined,
      { timeout: 5000, polling: POLLING_MS }
    );
    await lifecycle.evaluate(() => {
      window.dispatchEvent(new PageTransitionEvent("pageshow", { persisted: true }));
    });
    await lifecycle.waitForFunction(
      () =>
        Array.from(document.querySelectorAll(".card[data-showcase-id]")).every(
          (card) => card.dataset.playing === "true"
        ),
      undefined,
      { timeout: 5000, polling: POLLING_MS }
    );
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
