# Showcase Gallery

Six reference animations, each authored **end to end by an AI agent with fresh context**, equipped with
nothing but [`skills/rive-animation/SKILL.md`](../skills/rive-animation/SKILL.md) and the `rive-cli` binary.
No agent read the Rust source, the test fixtures, or any other scene in this directory.

They exist to prove two things: that the CLI exposes enough of the Rive format for an agent to do real
motion-design work, and that the agent can *verify its own output* — every scene below cleared a measured
gate (compiles, validates, loads in the real Rive WASM runtime, renders non-blank, and animates).

Each contact sheet below is frames 0/15/30/45/60 at 240x240, produced by `rive-cli render --contact-sheet`.

| Scene | Size | Animations | State machines | Peak colours |
|---|---:|---:|---:|---:|
| [Orbital Loader](#orbital-loader) | 1,688 B | 1 | 0 | 1,596 |
| [Interactive Pulse Button](#pulse-button) | 4,041 B | 3 | 1 | 3,416 |
| [Radial Dashboard Gauge](#radial-dashboard) | 2,400 B | 1 | 0 | 2,560 |
| [Audio Equaliser](#audio-equaliser) | 2,040 B | 1 | 0 | 2,845 |
| [Day / Night Toggle](#day-night-toggle) | 4,525 B | 2 | 1 | 484 |
| [Rocket Launch](#rocket-launch) | 3,505 B | 1 | 0 | 686 |

## Reproducing

The `.riv` files are committed and are byte-for-byte reproducible from their specs
(`test_showcase_riv_files_are_up_to_date` in `tests/e2e.rs` enforces this):

```bash
rive-cli generate showcase/orbital_loader.json -o showcase/orbital_loader.riv
rive-cli validate showcase/orbital_loader.riv
rive-cli render   showcase/orbital_loader.riv --frames 0,15,30,45,60 \
  --width 240 --height 240 --scale 1 --background '#0E0E12' --contact-sheet --preview -o frames/
```

`--preview` prints an ASCII coverage map, the dominant colour percentage and the content bounding box —
this is how an agent inspects a render without a vision model.

## Regression coverage

Every scene here is guarded by three layers: `generate` + `validate` in `tests/e2e.rs`, a real-runtime load
in `tests/playwright/regression.js`, and pixel baselines at frames 0/30/60 in
`tests/playwright/visual-regression.js`.

## The scenes

### Orbital Loader
<a id="orbital-loader"></a>

![Orbital Loader](previews/orbital_loader.png)

Indeterminate loading spinner: concentric rings, a gradient-stroked arc sweeping via trim path, a counter-rotating inner ring and an eased core pulse. Seamless loop.

`showcase/orbital_loader.json` &rarr; `showcase/orbital_loader.riv` (1,688 bytes) &middot; animations: `orbital_loop` &middot; peak distinct colours: 1,596

### Interactive Pulse Button
<a id="pulse-button"></a>

![Interactive Pulse Button](previews/pulse_button.png)

State-machine button with `isHovered` (bool) and `press` (trigger) inputs. Distinct resting, hovered and pressed looks with a gradient body, glow ring and ripple.

`showcase/pulse_button.json` &rarr; `showcase/pulse_button.riv` (4,041 bytes) &middot; animations: `rest`, `hovered`, `pressed` · state machines: `PulseButtonMachine` &middot; peak distinct colours: 3,416

```bash
rive-cli render showcase/pulse_button.riv --state-machine 'PulseButtonMachine' \
  --frames 0,20,40 --preview -o frames/
```

### Radial Dashboard Gauge
<a id="radial-dashboard"></a>

![Radial Dashboard Gauge](previews/radial_dashboard.png)

Data-visualisation gauge: a radial progress arc filling via trim path, a tick ring, a counter-rotating secondary arc and a scaling centre indicator.

`showcase/radial_dashboard.json` &rarr; `showcase/radial_dashboard.riv` (2,400 bytes) &middot; animations: `dashboard_pulse` &middot; peak distinct colours: 2,560

### Audio Equaliser
<a id="audio-equaliser"></a>

![Audio Equaliser](previews/audio_equaliser.png)

Seven spectrum bars with per-bar phase offsets and cubic easing, cool-to-warm gradient palette. Seamless loop.

`showcase/audio_equaliser.json` &rarr; `showcase/audio_equaliser.riv` (2,040 bytes) &middot; animations: `spectrum_loop` &middot; peak distinct colours: 2,845

### Day / Night Toggle
<a id="day-night-toggle"></a>

![Day / Night Toggle](previews/day_night_toggle.png)

Pill toggle animating between a warm day scene and a deep-blue night scene, driven by the `isNight` bool input. The knob slides and the palette shifts.

`showcase/day_night_toggle.json` &rarr; `showcase/day_night_toggle.riv` (4,525 bytes) &middot; animations: `day`, `night` · state machines: `DayNightMachine` &middot; peak distinct colours: 484

```bash
rive-cli render showcase/day_night_toggle.riv --state-machine 'DayNightMachine' \
  --frames 0,20,40 --preview -o frames/
```

### Rocket Launch
<a id="rocket-launch"></a>

![Rocket Launch](previews/rocket_launch.png)

Vector rocket accelerating upward with an eased climb, a flickering flame and stars drifting downward to imply speed.

`showcase/rocket_launch.json` &rarr; `showcase/rocket_launch.riv` (3,505 bytes) &middot; animations: `launch` &middot; peak distinct colours: 686
