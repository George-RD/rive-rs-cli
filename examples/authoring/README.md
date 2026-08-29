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
The broader behavior-compiler roadmap remains open for capabilities such as blend
states and parallel layers.

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
node tests/playwright/authoring-behavior-runtime.js
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
