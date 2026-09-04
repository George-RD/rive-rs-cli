const SHOWCASE_URL = "showcase.json";

function el(tag, className, text) {
  const node = document.createElement(tag);
  if (className) node.className = className;
  if (text !== undefined) node.textContent = text;
  return node;
}

function provenanceLabel(entry) {
  if (entry.provenance === "production") return "Production consumer";
  return entry.provenance === "authoring" ? "AuthoringSpec" : "SceneSpec";
}

function buildSource(entry) {
  const details = el("details", "source");
  details.appendChild(el("summary", null, entry.sourceLabel));

  const links = el("div", "hero-actions");
  const sourceLink = el("a", "text-link", "Open retained source ↗");
  sourceLink.href = entry.source;
  sourceLink.dataset.sourceLink = "true";
  links.appendChild(sourceLink);

  if (entry.evidence) {
    const evidenceLink = el("a", "text-link", `${entry.evidenceLabel || "Provenance"} ↗`);
    evidenceLink.href = entry.evidence;
    evidenceLink.dataset.evidenceLink = "true";
    links.appendChild(evidenceLink);
  }

  if (entry.consumerEvidence) {
    const consumerEvidenceLink = el("a", "text-link", "Consumer evidence ↗");
    consumerEvidenceLink.href = entry.consumerEvidence;
    consumerEvidenceLink.dataset.consumerEvidenceLink = "true";
    links.appendChild(consumerEvidenceLink);
  }

  if (entry.consumerAttestation) {
    const consumerLink = el("a", "text-link", "Consumer attestation ↗");
    consumerLink.href = entry.consumerAttestation;
    consumerLink.dataset.consumerAttestationLink = "true";
    links.appendChild(consumerLink);
  }

  if (entry.provenance === "authoring" || entry.sourceType === "authoring") {
    const guideLink = el("a", "text-link", "Authoring CLI ↗");
    guideLink.href = "docs/authoring-cli.md";
    links.appendChild(guideLink);
  }
  details.appendChild(links);

  const pre = el("pre", "terminal", "loading source…");
  details.appendChild(pre);
  details.addEventListener("toggle", async () => {
    if (!details.open || details.dataset.loaded === "true") return;
    details.dataset.loaded = "true";
    try {
      const response = await fetch(entry.source);
      if (!response.ok) throw new Error(`HTTP ${response.status}`);
      pre.textContent = await response.text();
    } catch (error) {
      pre.textContent = `could not load ${entry.source}: ${error.message || error}`;
    }
  });
  return details;
}

function initialInputs(entry) {
  const inputs = {};
  for (const control of entry.controls || []) {
    if (control.kind === "range") inputs[control.input] = Number(control.value ?? control.min ?? 0);
    if (control.kind === "toggle") inputs[control.input] = control.value === true;
  }
  return inputs;
}

function buildControls(entry, timeline) {
  const controls = entry.controls || [];
  if (controls.length === 0) return null;

  const panel = el("div", "scene-controls");
  panel.dataset.sceneControls = "true";
  const toggles = new Map();
  for (const control of controls) {
    const row = el("label", "scene-control");
    row.dataset.controlKind = control.kind;
    row.dataset.controlInput = control.input;

    if (control.kind === "trigger") {
      const button = el("button", "button button-quiet", control.label);
      button.type = "button";
      button.addEventListener("click", () => {
        const cleared = (control.clears || []).map((name) => {
          const box = toggles.get(name);
          if (box) box.checked = false;
          return timeline.setInput(name, false);
        });
        void Promise.all(cleared)
          .then(() => timeline.fireTrigger(control.input))
          .catch((error) => console.error(`could not fire ${control.input}`, error));
      });
      row.appendChild(button);
      panel.appendChild(row);
      continue;
    }

    row.appendChild(el("span", "scene-control-label", control.label));

    if (control.kind === "toggle") {
      const box = document.createElement("input");
      box.type = "checkbox";
      box.checked = control.value === true;
      box.addEventListener("change", () => {
        void timeline
          .setInput(control.input, box.checked)
          .catch((error) => console.error(`could not set ${control.input}`, error));
      });
      toggles.set(control.input, box);
      row.appendChild(box);
      panel.appendChild(row);
      continue;
    }

    const slider = document.createElement("input");
    slider.type = "range";
    slider.min = String(control.min ?? 0);
    slider.max = String(control.max ?? 100);
    slider.step = String(control.step ?? 1);
    slider.value = String(control.value ?? control.min ?? 0);
    const readout = el("output", "scene-control-value", slider.value);
    slider.addEventListener("input", () => {
      readout.textContent = slider.value;
      void timeline
        .setInput(control.input, Number(slider.value))
        .catch((error) => console.error(`could not set ${control.input}`, error));
    });
    row.appendChild(slider);
    row.appendChild(readout);
    panel.appendChild(row);
  }
  return panel;
}

function buildCard(entry) {
  const card = el("article", "card");
  card.dataset.showcaseId = entry.id;
  card.dataset.provenance = entry.provenance;
  card.dataset.playbackReady = "false";
  card.dataset.playing = "false";

  const stage = el("div", "stage");
  const canvas = el("canvas", "scene");
  canvas.width = 960;
  canvas.height = 540;
  canvas.setAttribute("aria-label", entry.alt);
  stage.appendChild(canvas);
  card.appendChild(stage);

  const body = el("div", "card-body");
  body.appendChild(el("h3", null, entry.title));
  body.appendChild(el("p", null, entry.summary));

  const tags = el("div", "tags");
  tags.appendChild(
    el(
      "span",
      entry.provenance === "authoring" ? "tag basics" : "tag",
      provenanceLabel(entry)
    )
  );
  tags.appendChild(el("span", "tag", entry.capability));
  body.appendChild(tags);

  const actions = el("div", "hero-actions");
  const toggle = el("button", "button button-quiet", "Pause");
  toggle.type = "button";
  toggle.disabled = true;
  toggle.setAttribute("aria-label", `Pause ${entry.title}`);
  actions.appendChild(toggle);
  body.appendChild(actions);
  card.appendChild(body);

  const timeline = RivePlayback.createTimeline({
    canvas,
    src: entry.artifact,
    stateMachine: entry.stateMachine,
    animation: entry.animation,
    inputs: initialInputs(entry),
    endFrame: entry.endFrame,
    loop: entry.loop,
    autoplay: true,
    onPlayingChange(playing) {
      card.dataset.playing = String(playing);
      toggle.textContent = playing ? "Pause" : "Play";
      toggle.setAttribute("aria-label", `${playing ? "Pause" : "Play"} ${entry.title}`);
    },
  });

  toggle.addEventListener("click", async () => {
    toggle.disabled = true;
    try {
      if (timeline.isPlaying) await timeline.pause();
      else await timeline.play();
    } finally {
      toggle.disabled = false;
    }
  });

  const controls = buildControls(entry, timeline);
  if (controls) body.appendChild(controls);
  body.appendChild(buildSource(entry));

  timeline.ready
    .then(() => {
      card.dataset.playbackReady = "true";
      toggle.disabled = false;
    })
    .catch((error) => {
      card.dataset.playbackReady = "error";
      toggle.textContent = "Unavailable";
      console.error(`could not start ${entry.id} showcase playback`, error);
    });

  return { card, timeline };
}

async function main() {
  const response = await fetch(SHOWCASE_URL);
  if (!response.ok) throw new Error(`could not load ${SHOWCASE_URL}: HTTP ${response.status}`);
  const entries = await response.json();
  if (!Array.isArray(entries)) throw new Error("showcase manifest must be an array");

  const grid = document.getElementById("showcase-grid");
  const timelines = [];
  const byId = new Map();
  for (const entry of entries) {
    const built = buildCard(entry);
    grid.appendChild(built.card);
    timelines.push(built.timeline);
    byId.set(entry.id, built.timeline);
  }
  window.__RIVE_SHOWCASE_TIMELINES = byId;

  const resumeAfterBfcache = new Set();

  window.addEventListener("resize", () => {
    for (const timeline of timelines) timeline.resize();
  });
  window.addEventListener("pagehide", (event) => {
    if (event.persisted) {
      resumeAfterBfcache.clear();
      for (const timeline of timelines) {
        if (!timeline.isPlaying) continue;
        resumeAfterBfcache.add(timeline);
        void timeline
          .pause()
          .catch((error) => console.error("could not pause showcase playback", error));
      }
      return;
    }
    resumeAfterBfcache.clear();
    for (const timeline of timelines) timeline.destroy();
  });
  window.addEventListener("pageshow", (event) => {
    if (!event.persisted) return;
    const pending = [...resumeAfterBfcache];
    resumeAfterBfcache.clear();
    for (const timeline of pending) {
      void timeline
        .play()
        .catch((error) => console.error("could not resume showcase playback", error));
    }
  });
}

main().catch((error) => console.error("could not build showcase", error));
