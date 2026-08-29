const RESULTS_URL = "parity/results.json";

function el(tag, className, text) {
  const node = document.createElement(tag);
  if (className) node.className = className;
  if (text !== undefined) node.textContent = text;
  return node;
}

function formatPercentage(value) {
  return `${value.toFixed(4)}%`;
}

function buildSide(label, ariaLabel) {
  const side = el("div", "side");
  side.appendChild(el("p", "side-label", label));
  const canvas = el("canvas", "scene");
  canvas.width = 600;
  canvas.height = 600;
  canvas.setAttribute("aria-label", ariaLabel);
  side.appendChild(canvas);
  return { side, canvas };
}

function buildPlaybackControls(rung) {
  const controls = el("div", "playback-controls");
  const toggle = el("button", "playback-toggle", "Play");
  toggle.type = "button";
  toggle.disabled = true;
  toggle.setAttribute("aria-label", `Play ${rung.title} comparison`);
  controls.appendChild(toggle);

  const frameLabel = el("label", "playback-frame-label", "Representative frame");
  const frame = el("select", "playback-frame");
  frame.disabled = true;
  frame.setAttribute("aria-label", `Representative frame for ${rung.title}`);
  for (const measured of rung.frames) {
    const option = el("option", null, String(measured.index));
    option.value = String(measured.index);
    frame.appendChild(option);
  }
  frameLabel.appendChild(frame);
  controls.appendChild(frameLabel);

  const status = el("span", "playback-status", "loading both files…");
  status.setAttribute("aria-live", "polite");
  controls.appendChild(status);

  return { controls, toggle, frame, status };
}

function buildMetrics(rung) {
  const metrics = el("dl", "metrics");
  const rows = [
    ["max pixel difference", formatPercentage(rung.max_pixel_difference)],
    ["objects", `${rung.reference_object_count} official / ${rung.candidate_object_count} ours`],
    ["frames compared", rung.frames.map((frame) => frame.index).join(", ")],
    [
      "type names missing",
      rung.missing_type_names.length === 0 ? "none" : rung.missing_type_names.join(", "),
    ],
  ];
  for (const [term, value] of rows) {
    metrics.appendChild(el("dt", null, term));
    metrics.appendChild(el("dd", null, value));
  }
  return metrics;
}

function buildDeltaTable(rung) {
  if (rung.type_deltas.length === 0) {
    return el("p", "delta-none", "Every Rive type appears the same number of times in both files.");
  }
  const table = el("table", "deltas");
  const head = el("tr");
  for (const column of ["type", "official", "ours", "delta"]) {
    head.appendChild(el("th", null, column));
  }
  table.appendChild(head);
  for (const row of rung.type_deltas) {
    const tr = el("tr");
    tr.appendChild(el("td", null, row.type_name));
    tr.appendChild(el("td", null, String(row.reference)));
    tr.appendChild(el("td", null, String(row.candidate)));
    tr.appendChild(el("td", null, row.delta > 0 ? `+${row.delta}` : String(row.delta)));
    table.appendChild(tr);
  }
  return table;
}

function buildSource(rung) {
  const details = el("details", "source");
  details.appendChild(el("summary", null, `${rung.id}.json — the JSON we compiled`));
  const pre = el("pre", "terminal", "loading…");
  details.appendChild(pre);
  details.addEventListener("toggle", async () => {
    if (!details.open || pre.dataset.loaded) return;
    pre.dataset.loaded = "1";
    try {
      const response = await fetch(rung.source);
      pre.textContent = await response.text();
    } catch (error) {
      pre.textContent = `could not load ${rung.source}: ${error}`;
    }
  });
  return details;
}

function buildCard(rung) {
  const card = el("article", "card");
  card.dataset.rungId = rung.id;
  card.dataset.playbackReady = "false";

  const stage = el("div", "stage compare");
  const official = buildSide("Official Rive file", `${rung.title} official animation`);
  const reproduction = buildSide("Compiled from our JSON", `${rung.title} compiled animation`);
  stage.appendChild(official.side);
  stage.appendChild(reproduction.side);
  card.appendChild(stage);

  const controls = buildPlaybackControls(rung);
  card.appendChild(controls.controls);

  const timeline = RivePlayback.createPairedTimeline({
    left: { canvas: official.canvas, src: rung.official },
    right: { canvas: reproduction.canvas, src: rung.reproduction },
    stateMachine: rung.state_machine,
    fps: RivePlayback.CAPTURE_FPS,
    onFrame(frame) {
      card.dataset.logicalFrame = String(frame);
      controls.status.textContent = `frame ${frame} · ${RivePlayback.CAPTURE_FPS} fps`;
    },
    onPlayingChange(playing) {
      controls.toggle.textContent = playing ? "Pause" : "Play";
      controls.toggle.setAttribute(
        "aria-label",
        `${playing ? "Pause" : "Play"} ${rung.title} comparison`
      );
    },
  });

  controls.toggle.addEventListener("click", async () => {
    controls.toggle.disabled = true;
    try {
      if (timeline.isPlaying) {
        await timeline.pause();
      } else {
        await timeline.play();
      }
    } finally {
      controls.toggle.disabled = false;
    }
  });

  controls.frame.addEventListener("change", async () => {
    controls.frame.disabled = true;
    controls.toggle.disabled = true;
    try {
      await timeline.pause();
      await timeline.seekToFrame(Number(controls.frame.value));
    } finally {
      controls.frame.disabled = false;
      controls.toggle.disabled = false;
    }
  });

  timeline.ready
    .then(async () => {
      controls.frame.disabled = false;
      controls.toggle.disabled = false;
      await timeline.play();
      card.dataset.playbackReady = "true";
    })
    .catch((error) => {
      card.dataset.playbackReady = "error";
      controls.status.textContent = "playback unavailable";
      console.error(`could not start ${rung.id} playback`, error);
    });

  const body = el("div", "card-body");
  body.appendChild(el("h3", null, rung.title));

  const tags = el("div", "tags");
  tags.appendChild(el("span", "tag", rung.upstream));
  tags.appendChild(
    el(
      "span",
      `tag ${rung.max_pixel_difference === 0 ? "basics" : ""}`.trim(),
      formatPercentage(rung.max_pixel_difference)
    )
  );
  body.appendChild(tags);

  body.appendChild(el("p", null, rung.note));
  body.appendChild(buildMetrics(rung));
  body.appendChild(buildDeltaTable(rung));
  body.appendChild(buildSource(rung));

  card.appendChild(body);
  return { card, timeline };
}

async function main() {
  const grid = document.getElementById("grid");
  const response = await fetch(RESULTS_URL);
  const rungs = await response.json();
  const timelines = [];
  for (const rung of rungs) {
    const built = buildCard(rung);
    grid.appendChild(built.card);
    timelines.push(built.timeline);
  }

  window.addEventListener("resize", () => {
    for (const timeline of timelines) timeline.resize();
  });
  window.addEventListener(
    "pagehide",
    () => {
      for (const timeline of timelines) timeline.destroy();
    },
    { once: true }
  );

  const transcript = document.getElementById("transcript");
  try {
    const verify = await fetch("verify.txt");
    transcript.textContent = await verify.text();
  } catch (error) {
    transcript.textContent = `could not load verify.txt: ${error}`;
  }
}

main();
