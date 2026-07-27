const RESULTS_URL = "parity/results.json";

function el(tag, className, text) {
  const node = document.createElement(tag);
  if (className) node.className = className;
  if (text !== undefined) node.textContent = text;
  return node;
}

function mountScene(canvas, file, options = {}) {
  const params = {
    canvas,
    src: file,
    autoplay: true,
    fit: rive.Fit.contain,
    alignment: rive.Alignment.center,
  };
  if (options.stateMachine) {
    params.stateMachines = [options.stateMachine];
  } else if (options.animation) {
    params.animations = [options.animation];
  }
  const instance = new rive.Rive(params);
  instance.on(rive.EventType.Load, () => {
    instance.resizeDrawingSurfaceToCanvas();
  });
  return instance;
}

function formatPercentage(value) {
  return `${value.toFixed(4)}%`;
}

function buildSide(label, file, ariaLabel, rung) {
  const side = el("div", "side");
  side.appendChild(el("p", "side-label", label));
  const canvas = el("canvas", "scene");
  canvas.width = 600;
  canvas.height = 600;
  canvas.setAttribute("aria-label", ariaLabel);
  side.appendChild(canvas);
  mountScene(canvas, file, { stateMachine: rung.state_machine });
  return side;
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

  const stage = el("div", "stage compare");
  stage.appendChild(
    buildSide("Official Rive file", rung.official, `${rung.title} official animation`, rung)
  );
  stage.appendChild(
    buildSide(
      "Compiled from our JSON",
      rung.reproduction,
      `${rung.title} reproduction animation`,
      rung
    )
  );
  card.appendChild(stage);

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
  return card;
}

async function main() {
  rive.RuntimeLoader.setWasmUrl("assets/rive.wasm");

  const hero = document.querySelector(".hero-art .scene");
  if (hero) {
    mountScene(hero, "parity/official/coffee_loader.riv", { stateMachine: "State Machine 1" });
  }

  const grid = document.getElementById("grid");
  const response = await fetch(RESULTS_URL);
  const rungs = await response.json();
  for (const rung of rungs) {
    grid.appendChild(buildCard(rung));
  }

  const transcript = document.getElementById("transcript");
  try {
    const verify = await fetch("verify.txt");
    transcript.textContent = await verify.text();
  } catch (error) {
    transcript.textContent = `could not load verify.txt: ${error}`;
  }
}

main();
