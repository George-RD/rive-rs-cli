# AuthoringSpec v0 examples

These fixtures exercise the typed authoring frontend before it lowers to the
canonical `SceneSpec` object graph.

| Fixture | Purpose |
|---|---|
| `component-badges.v0.json` | Reusable components, parameters, and instance overrides |
| `text-label.v0.json` | Typed literal text and semantic text styling |
| `typed-motion.v0.json` | Typed visual composition, relative image asset, poses, and timeline motion |
| `behavior-binding.v0.json` | Boolean view-model binding driving a named typed statechart |
| `pointer-statechart.v0.json` | Boolean state-machine input, named event, pointer listener, and input-driven transition |
| `raw-pulse.v0.json` | The explicit raw SceneSpec escape hatch for unsupported concepts |
| `complex-static-showcase.v0.json` | A complex static composition built without raw scene, motion, or behavior escapes |
| `complex-animated-showcase.v0.json` | A complex animated signal-to-action story built without raw scene, motion, or behavior escapes |
| `complex-interactive-showcase.v0.json` | Three-state typed interaction gate with two boolean inputs, a reset event, pointer listeners, and bidirectional transitions |
| `stacking-card.v0.json` | Explicit `back_to_front` stacking on the visual root and on a group |
| `waypoint-transit.v0.json` | A `through` motion track crossing an interior waypoint without losing speed |
| `blend-meter.v0.json` | A number input driving a 1D blend state across two motion tracks |
| `interactive-console.v0.json` | Stacking, waypoint continuity, a blend gauge, and three concurrent state-machine layers in one document |
| `signal-weave.v0.json` | Three statechart regions animating one artboard without any input |

The complex static showcase combines reusable components, expression-backed
parameters, linear and radial gradients, trimmed strokes, text, grid, radial,
mirror, distribute, and along-path patterns, plus align, center, offset, and
spacing constraints.

The complex animated showcase is the motion product gate. It tells a generic
scattered inputs → overload → connected context → one next action story using
stable visual IDs, four named poses, one compact track, a shared cubic easing,
opacity, transforms, and animated width/height. Its representative pose frames
are retained in the Playwright visual-regression evidence loop so runtime motion
failures remain inspectable rather than only structurally detectable.

The complex interactive showcase is the behavior exit gate for the supported
typed subset. One authored statechart selects three short pose tracks through two
boolean inputs, four named transitions, pointer listeners, and a named reset
event. Its contract compares the complete lowering with an explicitly authored
canonical state machine and then sends that SceneSpec through the shared builder.
Blend states and parallel layers are now typed, in `blend-meter.v0.json` and
`interactive-console.v0.json`. Additive blend states, direct blend states,
transition duration and exit time, and view-model number or trigger properties
are not exposed by the AuthoringSpec frontend.

`stacking-card.v0.json` sets `"stacking": "back_to_front"` on the `visual`
section and on the `card` group, so the 32px `cue` rectangle authored second
paints over the 128px `surface` rectangle authored first. Authored order is
retained in diagnostics and source maps while only the emitted child order
changes: the second child keeps the authored path
`$.visual.nodes[0].children[1]` and takes the scene path
`/artboard/children/0/children/0`. `tests/authoring_stacking_contract.rs` covers
the four lowering cases, and `tests/authoring_stacking_runtime.rs` renders
frame 0 at 128x128 through Chromium and asserts the cue owns the centre pixel
under `back_to_front` and the surface owns it under `runtime`.

`waypoint-transit.v0.json` carries a token across 40px, 160px and 280px at frames
0, 30 and 60 of a 320x160 artboard, with the same cubic easing `settle`
(0.23, 1, 0.32, 1) on all three keyframes. The track declares
`"continuity": "through"`, so the segment arriving at frame 30 is emitted as
`linear` without an interpolator and the token holds its speed through the
waypoint, while the segment arriving at frame 60 keeps the authored easing.
`tests/authoring_motion_continuity_runtime.rs` measures the token's horizontal
centre between frame 26 and frame 30: `through` travels at least 12px into the
waypoint, and the same document with `continuity` removed travels at most 2px.
`tests/authoring_motion_continuity_contract.rs` covers the nine lowering cases,
including the `waypoint_not_interior` failure and the `waypoint_stop_start`
warning.

`blend-meter.v0.json` declares one `load` number input and a single `reading`
state whose `blend` interpolates `calm-track` at 0 against `surge-track` at 100.
`tests/authoring_behavior_blend_runtime.rs` drives `load` at 0, 50 and 100
through the state machine and finds the needle on the calm stop, on the surge
stop, and strictly between them. Rive mixes the two neighbouring animations
sequentially rather than as a weighted average, so input 50 renders at about
146px between stops driving 40px and 200px, not at the arithmetic midpoint; the
mapping stays monotonic. `tests/authoring_behavior_blend_contract.rs` covers the
ten lowering cases for typed inputs, blend states, typed conditions, regions,
listener actions, and a statechart declared beside file assets.

`interactive-console.v0.json` combines stacking, waypoint continuity, a blend
gauge, and parallel regions in one 960x540 document, with a committed `.riv`
beside it. It declares the backdrop first under `back_to_front` stacking, moves
the stream token on a `through` track, drives the gauge from a `blend_state_1d`
over the `load` number input, and runs three state-machine layers: the
statechart's own `standby` and `running` states, a `stream` region carrying the
token, and an `alert` region that escalates on `load >= 60` and settles on
`load < 60`. A `click` listener on the `arm-surface` rectangle sets `armed`; one
on `reset-surface` clears it and fires the `reset` trigger.
`tests/authoring_console_runtime.rs` renders the committed artifact and
checks that a pointer press on `arm-surface` moves the needle onto the blended
load, that the alert lamp covers more pixels at load 90 than at load 10, and
that the stream token advances while layer 0 stays in `standby`.
`tests/showcase_artifact.rs` recompiles the committed showcases and fails on
byte drift.

`signal-weave.v0.json` is the animation-only counterpart and also commits its
`.riv`. Its statechart declares no inputs and no transitions between authored
states: layer 0 turns the `halo` group through 360 degrees over 720 frames, a
`core` region breathes the centre ellipse on a 120-frame pingpong, and a
`courier` region shuttles a token across the lane on a `through` track so it
passes the middle waypoint at speed. `tests/authoring_weave_runtime.rs` renders
the committed artifact at frames 0 and 60 and measures each region separately:
the courier travels at least 20px, the spoke centroid inside a left-hand window
moves, and the core highlight changes area. Frame 45 is not usable for that
comparison because a 720-frame turn puts the 22.5-degree spoke spacing exactly
back on itself there.

Compile the high-level typed-motion fixture directly:

```bash
cargo run -- authoring compile examples/authoring/typed-motion.v0.json -o typed-motion.riv
cargo run -- authoring compile examples/authoring/typed-motion.v0.json -o typed-motion.riv --file-id 42 --json
```

Compile and drive the typed interaction fixture through the public runtime path:

```bash
cargo run -- authoring compile examples/authoring/pointer-statechart.v0.json -o pointer-statechart.riv
cargo run -- render pointer-statechart.riv --state-machine auth__interaction_2dstage__gate__state_machine --input auth__interaction_2dstage__gate__pressed__input=true@1 --frames 0,12
cargo run -- render pointer-statechart.riv --state-machine auth__interaction_2dstage__gate__state_machine --pointer down:60,100@1 --frames 0,12
```

Compile the complex animated and interactive showcases through the same public path:

```bash
cargo run -- authoring compile examples/authoring/complex-animated-showcase.v0.json -o complex-animated-showcase.riv
cargo run -- authoring compile examples/authoring/complex-interactive-showcase.v0.json -o complex-interactive-showcase.riv
```

Compile the stacking, waypoint, and blend fixtures the same way:

```bash
cargo run -- authoring compile examples/authoring/stacking-card.v0.json -o stacking-card.riv
cargo run -- authoring compile examples/authoring/waypoint-transit.v0.json -o waypoint-transit.riv
cargo run -- authoring compile examples/authoring/blend-meter.v0.json -o blend-meter.riv
```

Regenerate the committed artifacts in place when a document or the compiler
changes:

```bash
cargo run --quiet -- authoring compile examples/authoring/interactive-console.v0.json -o examples/authoring/interactive-console.v0.riv
cargo run --quiet -- authoring compile examples/authoring/signal-weave.v0.json -o examples/authoring/signal-weave.v0.riv
```

Print the high-level AuthoringSpec schema with:

```bash
cargo run -- authoring schema
```

`authoring compile` is the typed, semantic input path. `generate` remains the
explicit expert path for raw `SceneSpec` JSON:

```bash
cargo run -- generate scene.json -o output.riv
```

Run the durable example and CLI contracts with:

```bash
cargo test --test authoring_examples
cargo test --test authoring_cli
cargo test --test authoring_interaction_contract
cargo test --test authoring_behavior_exit_gate
cargo test --test authoring_stacking_contract
cargo test --test authoring_motion_continuity_contract
cargo test --test authoring_behavior_blend_contract
cargo test --test showcase_artifact
node tests/playwright/authoring-behavior-runtime.js
```

The stacking, waypoint, blend, console, and weave fixtures also carry
official-runtime evidence. These five tests drive headless Chromium and measure
pixels, so they need a Chrome or Chromium executable; set `$RIVE_CHROME` for a
non-standard location:

```bash
cargo test --test authoring_stacking_runtime
cargo test --test authoring_motion_continuity_runtime
cargo test --test authoring_behavior_blend_runtime
cargo test --test authoring_console_runtime
cargo test --test authoring_weave_runtime
```

The example contract lowers every fixture twice to prove deterministic
SceneSpec and source-map output, then builds the canonical object graph. The
complex animated contract additionally proves its motion vocabulary and
canonical builder/encoder/validator path. The typed interaction contract proves
input/event/listener lowering and authored-path diagnostics, while the retained
runtime contract requires both `render --input` and `render --pointer` to produce
the same visible state transition. The behavior exit-gate contract proves a
non-trivial typed statechart lowers exactly to the expected canonical SceneSpec.
The CLI contract compiles typed visual and motion content through the shared
SceneSpec compilation seam, validates the resulting `.riv`, preserves file IDs
and input-relative assets, and checks full source-mapped JSON diagnostics.
