const SHOWCASES = [
  {
    id: "wordmark",
    title: "Wordmark",
    tier: "advanced",
    tag: "embedded font",
    animation: "wordmark",
    blurb:
      "An animated logotype set in an embedded SIL OFL font. The word and its tagline arrive on staggered beats, a gradient underline sweeps in via trim path, and an orbiting mark rotates through the loop.",
  },
  {
    id: "liquid_loader",
    title: "Liquid Loader",
    tier: "advanced",
    tag: "path morphing",
    animation: "liquid",
    blurb:
      "The outline itself deforms. Six cubic vertices of a closed path are keyframed in position and handle length, so the silhouette morphs rather than merely transforming.",
  },
  {
    id: "textured_scene",
    title: "Textured Scene",
    tier: "advanced",
    tag: "embedded image",
    animation: "scene",
    blurb:
      "An illustrated vignette combining an embedded PNG with hand-authored bezier ridges, a radial sun halo, a dashed light arc and drifting mist. The layers parallax against each other.",
  },
  {
    id: "control_panel",
    title: "Control Panel",
    tier: "advanced",
    tag: "pointer + blend state",
    stateMachine: "Panel",
    blurb:
      "Press the button — the pointer event runs through a real Rive listener. The dial is driven by a 1D blend state reading the level input.",
    controls: [
      { kind: "trigger-bool", input: "pressed", label: "Press button" },
      { kind: "number", input: "level", label: "Level", min: 0, max: 100, value: 35 },
    ],
  },
  {
    id: "orbital_loader",
    title: "Orbital Loader",
    tier: "basics",
    tag: "trim paths",
    animation: "orbital_loop",
    blurb:
      "Indeterminate spinner: concentric rings, a gradient-stroked arc sweeping via trim path, a counter-rotating inner ring and an eased core pulse.",
  },
  {
    id: "pulse_button",
    title: "Pulse Button",
    tier: "basics",
    tag: "state machine",
    stateMachine: "PulseButtonMachine",
    blurb:
      "A state-machine button with hover and press inputs, each with its own resting look, glow ring and ripple.",
    controls: [
      { kind: "bool", input: "isHovered", label: "Hovered" },
      { kind: "trigger", input: "press", label: "Press" },
    ],
  },
  {
    id: "radial_dashboard",
    title: "Radial Dashboard",
    tier: "basics",
    tag: "gauge",
    animation: "dashboard_pulse",
    blurb:
      "A data-visualisation gauge: a radial progress arc filling via trim path, a tick ring, a counter-rotating secondary arc and a scaling centre indicator.",
  },
  {
    id: "audio_equaliser",
    title: "Audio Equaliser",
    tier: "basics",
    tag: "phase offsets",
    animation: "spectrum_loop",
    blurb:
      "Seven spectrum bars with per-bar phase offsets and cubic easing, on a cool-to-warm gradient palette.",
  },
  {
    id: "day_night_toggle",
    title: "Day / Night Toggle",
    tier: "basics",
    tag: "state machine",
    stateMachine: "DayNightMachine",
    blurb:
      "A pill toggle animating between a warm day scene and a deep-blue night scene, driven by a single bool input.",
    controls: [{ kind: "bool", input: "isNight", label: "Night" }],
  },
  {
    id: "rocket_launch",
    title: "Rocket Launch",
    tier: "basics",
    tag: "eased motion",
    animation: "launch",
    blurb:
      "A vector rocket accelerating upward with an eased climb, a flickering flame and stars drifting downward to imply speed.",
  },
];

const RIV_DIR = "showcase";

function el(tag, className, text) {
  const node = document.createElement(tag);
  if (className) node.className = className;
  if (text !== undefined) node.textContent = text;
  return node;
}

function mountScene(canvas, scene) {
  const params = {
    src: `${RIV_DIR}/${scene.id}.riv`,
    canvas,
    autoplay: true,
    fit: rive.Fit.contain,
    alignment: rive.Alignment.center,
  };
  if (scene.stateMachine) {
    params.stateMachines = [scene.stateMachine];
  } else if (scene.animation) {
    params.animations = [scene.animation];
  }
  const instance = new rive.Rive(params);
  instance.on(rive.EventType.Load, () => {
    instance.resizeDrawingSurfaceToCanvas();
  });
  return instance;
}

function buildControls(scene, instance) {
  const wrap = el("div", "controls");
  const inputsFor = () => instance.stateMachineInputs(scene.stateMachine) || [];
  for (const control of scene.controls) {
    if (control.kind === "bool") {
      const button = el("button", null, control.label);
      button.setAttribute("aria-pressed", "false");
      button.addEventListener("click", () => {
        const input = inputsFor().find((i) => i.name === control.input);
        if (!input) return;
        input.value = !input.value;
        button.setAttribute("aria-pressed", String(input.value));
      });
      wrap.appendChild(button);
    } else if (control.kind === "trigger") {
      const button = el("button", null, control.label);
      button.addEventListener("click", () => {
        const input = inputsFor().find((i) => i.name === control.input);
        if (input) input.fire();
      });
      wrap.appendChild(button);
    } else if (control.kind === "trigger-bool") {
      const button = el("button", null, control.label);
      button.addEventListener("click", () => {
        const input = inputsFor().find((i) => i.name === control.input);
        if (!input) return;
        input.value = true;
        window.setTimeout(() => {
          input.value = false;
        }, 900);
      });
      wrap.appendChild(button);
    } else if (control.kind === "number") {
      const label = el("label");
      label.appendChild(el("span", null, control.label));
      const range = document.createElement("input");
      range.type = "range";
      range.min = String(control.min);
      range.max = String(control.max);
      range.value = String(control.value);
      const readout = el("span", null, String(control.value));
      const apply = () => {
        const input = inputsFor().find((i) => i.name === control.input);
        if (input) input.value = Number(range.value);
        readout.textContent = range.value;
      };
      range.addEventListener("input", apply);
      label.appendChild(range);
      label.appendChild(readout);
      wrap.appendChild(label);
      window.setTimeout(apply, 400);
    }
  }
  return wrap;
}

async function buildCard(scene) {
  const card = el("article", "card");

  const stage = el("div", "stage");
  const canvas = el("canvas", "scene");
  canvas.width = 600;
  canvas.height = 600;
  canvas.setAttribute("aria-label", `${scene.title} animation`);
  stage.appendChild(canvas);
  card.appendChild(stage);

  const body = el("div", "card-body");
  body.appendChild(el("h3", null, scene.title));

  const tags = el("div", "tags");
  tags.appendChild(el("span", `tag ${scene.tier === "basics" ? "basics" : ""}`.trim(), scene.tag));
  tags.appendChild(el("span", `tag ${scene.tier === "basics" ? "basics" : ""}`.trim(), scene.tier));
  body.appendChild(tags);

  body.appendChild(el("p", null, scene.blurb));

  const instance = mountScene(canvas, scene);
  if (scene.controls) {
    body.appendChild(buildControls(scene, instance));
  }

  const details = el("details", "source");
  details.appendChild(el("summary", null, `${scene.id}.json`));
  const pre = el("pre", "terminal", "loading…");
  details.appendChild(pre);
  details.addEventListener(
    "toggle",
    async () => {
      if (!details.open || pre.dataset.loaded) return;
      pre.dataset.loaded = "1";
      try {
        const response = await fetch(`${RIV_DIR}/${scene.id}.json`);
        pre.textContent = await response.text();
      } catch (error) {
        pre.textContent = `could not load ${scene.id}.json: ${error}`;
      }
    },
    { once: false }
  );
  body.appendChild(details);

  card.appendChild(body);
  return card;
}

async function main() {
  rive.RuntimeLoader.setWasmUrl("assets/rive.wasm");

  const hero = document.querySelector(".hero-art .scene");
  if (hero) {
    mountScene(hero, { id: "wordmark", animation: "wordmark" });
  }

  const grid = document.getElementById("grid");
  for (const scene of SHOWCASES) {
    grid.appendChild(await buildCard(scene));
  }

  const transcript = document.getElementById("transcript");
  try {
    const response = await fetch("verify.txt");
    transcript.textContent = await response.text();
  } catch (error) {
    transcript.textContent = `could not load verify.txt: ${error}`;
  }
}

main();
