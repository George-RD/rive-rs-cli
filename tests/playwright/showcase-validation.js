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
    for (const field of ["artifact", "source", "evidence", "consumerAttestation"]) {
      if (entry[field] && !staged.has(entry[field])) {
        throw new Error(`staging plan omitted ${entry.id} ${field} ${entry[field]}`);
      }
    }
  }

  const production = entries.find((entry) => entry.id === PRODUCTION_ID);
  if (!production || production.provenance !== "production") {
    throw new Error("Horaxon production proof is missing its production provenance type");
  }
  for (const field of ["artifact", "source", "evidence", "consumerAttestation"]) {
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
          "[data-source-link=\"true\"], [data-evidence-link=\"true\"], [data-consumer-attestation-link=\"true\"]"
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
      if (!productionCard.text.includes("Production consumer")) {
        errors.push("Horaxon card is not visibly labelled as production consumer proof");
      }
      if (!productionCard.text.includes("not a customer endorsement")) {
        errors.push("Horaxon card does not preserve the endorsement claim boundary");
      }
    }

    const evidenceStatuses = await readEvidenceStatuses(page);
    for (const evidence of evidenceStatuses) {
      if (evidence.status !== 200) {
        errors.push(`source/provenance link ${evidence.href} returned HTTP ${evidence.status}`);
      }
    }

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
      `Showcase validation passed: ${entries.length} manifest-driven cards including local Horaxon production provenance, live runtime paint, phone layout, reduced motion\n`
    );
    shutdown();
    process.exit(0);
  } catch (error) {
    process.stdout.write(`Showcase validation error: ${error.message}\n`);
    shutdown();
    process.exit(1);
  }
})();
