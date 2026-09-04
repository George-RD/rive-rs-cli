---
node: rive-cli.intelligence.authoring
status: open
created: 2026-07-29
---

# P2 — Implement view-model-first behavior and named statecharts

Add typed model properties, bindings, events, named states, named transitions,
parallel regions, and compiler lowering to Rive view models, data bindings, blend
states, listeners, and indexed state-machine objects.

## Acceptance criteria

- Authored transitions never reference array indices.
- Bindings express source, target, and conversion intent with typed validation.
- The compiler selects bindings, poses, or blend animations without changing authored intent.
- Interaction tests drive pointer and input events and retain runtime evidence.
- A complex interactive showcase is reproduced through the frontend.

## Dependency

Depends on the motion slice for pose and blend-state lowering. Do not begin typed
behavior implementation until the one-pass Authoring compiler architecture gate in
`todo.motion-authoring-compiler.md` is complete. Behavior must consume the same
compiler-owned scene draft, resolved-symbol model, runtime-name registry, checked
runtime bindings, and source-map builder; it must not introduce another raw-fragment
re-entry pass or a second full document lowering.

## Evidence

PR #203 / issue #179 establishes the first typed behavior tracer bullet on the shared Authoring compiler state: boolean view-model properties, authored bindings, named states, named transitions, deterministic source maps, canonical-builder validation, and retained official-runtime evidence that mutating the bound view-model boolean changes state. The slice preserves `raw_state_machines` as an expert escape without introducing a second full-document lowering pass.

PR #204 / issue #180 extends that same compiler path with authored boolean state-machine inputs, named Rive events, typed pointer/event listeners, boolean listener actions, and input-driven transitions. Authored visual and event IDs resolve to generated runtime names without exposing runtime indices; invalid input, event, action, and listener references report authored JSON paths. The retained behavior runtime contract compiles the typed fixture through `authoring compile` and drives the resulting `.riv` through both `render --input` and `render --pointer`, requiring both paths to converge on the same visible state.

PR #221 extends the same compiler path with number and trigger inputs, comparison and trigger conditions, one-dimensional blend states, parallel regions, and typed listener actions. Exact head `02d5621` passed CI run `33836991510` across formatting, Clippy, the Rust 1.88 minimum, browser contracts, Cairn architecture validation, official-runtime evidence, demo, site, Playwright, and visual regression, and it merged to `main` as `73598d5` on 2026-09-04.

`BehaviorInputSpec` gains `{"kind": "number", "id": ..., "value": <scalar expression>}` and `{"kind": "trigger", "id": ...}` beside the existing `bool`. Two forms join the existing untagged condition union: the `{"binding": ..., "equals": ...}` and `{"input": ..., "equals": ...}` forms are unchanged, and the new forms are `{"input": ..., "compare": ..., "value": <scalar expression>}`, where `compare` is one of `equal`, `not_equal`, `greater`, `greater_or_equal`, `less`, and `less_or_equal`, and `{"trigger": ...}`. A condition whose form does not match the declared input kind reports `invalid_condition_input` at `$....transitions[i].when.input` or `$....transitions[i].when.trigger`. Listener actions gain `number_change` with a scalar `value` and `trigger_change`; a mismatched action kind reports `invalid_listener_input` at the action's `input` path.

A behavior state's `motion` is now optional and an optional `blend` sits beside it. Exactly one is required: `missing_state_motion` or `ambiguous_state_motion` at the state path. `blend` is `{"input": <number input id>, "stops": [{"motion": <track id>, "value": <scalar expression>}, ...]}` and lowers to a `blend_state_1d` whose children are `blend_animation_1d` entries naming the lowered animations. `invalid_blend_input` fires at `$....states[i].blend.input` when the named input is not a number input, `unknown_behavior_input` when it does not exist, and `unknown_behavior_motion` at `$....states[i].blend.stops[j].motion` for an unknown track.

A review after the merge found two blend invariants that only the published JSON schema enforced, so the typed Rust path accepted documents the schema would reject. The compiler now checks both. A blend needs between 2 and 1000 stops, matching the schema's `minItems` and `maxItems`, and a count outside that range reports `invalid_blend_stops` at `$....states[i].blend.stops`. Stop values must strictly increase, because `BlendState1DInstance::animationIndex` binary-searches its children as ascending thresholds and would otherwise leave a stop unreachable; a value that does not exceed its predecessor reports `invalid_blend_stop_order` at `$....states[i].blend.stops[j].value`.

`regions` adds `{"id", "initial", "states", "transitions"}` entries to a statechart. The statechart's own states remain layer 0, and each region becomes one further state-machine layer with its own entry state, exit state, and entry transition. Region ids are unique within a statechart, and a repeat reports `duplicate_behavior_region` at `$....regions[i].id`. The same review found that uniqueness was checked only among regions. States, transitions, inputs, events, listeners and regions all claim the source-map identity `{statechart}/{id}`, and consumers resolve an entry by first match, so a region taking any of those ids made the lookup ambiguous; that now reports `behavior_region_id_collision` at the same path. Collisions between the other five kinds remain unchecked, each still having only its own uniqueness set; that is a pre-existing gap this slice does not close. Source-map authored ids inside a region are `statechart/region/state` and their scene paths are `/artboard/state_machines/{m}/layers/{n}/states/{i}`.

The same work fixes a defect found while writing the fixture: typed behavior validated its lowered scene with file-asset `source` fields still attached, so a document declaring `font_assets` or `image_assets` alongside a statechart could not compile. Behavior validation now removes asset sources the way the visual path already did, and `file_assets_compose_with_typed_behavior` in `tests/authoring_behavior_blend_contract.rs` pins it.

`tests/authoring_behavior_blend_contract.rs` holds thirteen contracts: typed input lowering, blend lowering over named motion, trigger and comparison conditions without runtime indices, parallel region layers, the exactly-one-motion-source rule, blend input and stop validation, condition kind matching, region identity and initial state, typed listener actions, the asset regression, stop cardinality, stop ordering, and region id collision. `tests/authoring_behavior_blend_runtime.rs` drives `load` at 0, 50, and 100 through `examples/authoring/blend-meter.v0.json` at 240x160 and requires the needle within 3px of 40px at load 0, within 3px of 200px at load 100, and strictly between the two at load 50. Rive mixes the two neighbouring blend animations sequentially rather than as a weighted average, so load 50 renders near 146px rather than the arithmetic midpoint at 120px; the mapping stays monotonic.

`examples/authoring/interactive-console.v0.json` exercises the slice end to end and its compiled `examples/authoring/interactive-console.v0.riv` is committed. The document uses `back_to_front` stacking, a `through` track for the stream token, and a statechart whose own layer holds `standby` on `gauge-standby-track` and `running` on a `blend_state_1d` over `gauge-low-track` at 0 and `gauge-high-track` at 100, entered on `armed` and left on the `reset` trigger. Two regions run beside it: `stream` carries the token, and `alert` escalates on `load >= 60` and settles on `load < 60`. A pointer `click` listener on `arm-surface` sets `armed` true; one on `reset-surface` sets it false and fires the `reset` trigger. It declares `font_assets`, which a statechart document could not do before the asset-source fix. `tests/authoring_console_runtime.rs` renders the committed artifact at 960x540 through the official runtime and holds three contracts: a pointer click on ARM moves the gauge needle onto the blended load, the alert lamp covers more pixels at load 90 than at load 10, and the stream token advances while the statechart's own layer is still in `standby`. `tests/showcase_artifact.rs` lists `interactive-console` among the showcases it recompiles through the public CLI and compares byte for byte; regenerate with `cargo run --quiet -- authoring compile examples/authoring/interactive-console.v0.json -o examples/authoring/interactive-console.v0.riv`.

`examples/authoring/signal-weave.v0.json` proves the same region machinery without any input. Its statechart declares no inputs and no transitions between authored states: layer 0 turns the `halo` group through 360 degrees over 720 frames, a `core` region breathes the centre ellipse on a 120-frame pingpong, and a `courier` region shuttles a token on a `through` track. `tests/authoring_weave_runtime.rs` renders the committed artifact at frames 0 and 60 and measures the three regions separately, avoiding frame 45 because a 720-frame turn maps the 22.5-degree spoke spacing back onto itself there. `tests/playwright/visual-regression.js` pixel-compares it at frames 0, 60, and 120 against committed baselines, which the console cannot be given because its visible state comes from inputs.

`site/playback.js` gained `setInput(name, value)` and `fireTrigger(name)`. Assigned input values are retained and reapplied when a backward seek rebuilds the state-machine instance; a fired trigger is momentary and is not replayed. A `site/showcase.json` entry may declare `controls` of kind `range`, `toggle`, and `trigger`, the last carrying a `clears` list of bool input names, and `site/showcase.js` renders them against the timeline. `tests/playwright/showcase-validation.js` measures the gauge needle in the canvas while the console is in standby, moves the slider to 100 and checks the armed toggle, requires the needle to land on the blended load, then presses reset and requires it back below the standby limit. Its pixel constants come from the 960x540 console artboard: the needle spans y 324 to 357 and the gauge track starts at y 332, so row 328 is the only row that crosses the needle and nothing else. Standby holds the needle near x 122 and load 100 drives it past x 700, so the check bounds it below x 288 in standby and above x 576 when armed. A control change on a paused card settles the state machine without moving the timeline: `settleControlChange` queues at most one advance behind the seek chain, so dragging a slider cannot advance playback by an event-density-dependent number of frames. The console is registered in `tests/playwright/shared.js` as `authoring_interactive_console` and listed in `RUNTIME_ONLY_FIXTURES`, so `tests/playwright/regression.js` loads it while `tests/playwright/visual-regression.js` does not pixel-compare it, because its visible state comes from inputs rather than from a fixed timeline.

Against the acceptance criteria:

- Authored transitions never reference array indices: met. Conditions name a binding, an input, or a trigger by authored id, and region layer indices are assigned by the compiler.
- Bindings express source, target, and conversion intent with typed validation: partly met. Model, property, input, and trigger references are typed and validated at authored paths, but `BehaviorPropertySpec` still has one variant, `bool`, so no numeric or enumerated conversion can be authored.
- The compiler selects bindings, poses, or blend animations without changing authored intent: met. A state names motion tracks and a number input; the compiler emits the `blend_state_1d` and its `blend_animation_1d` children.
- Interaction tests drive pointer and input events and retain runtime evidence: met. `tests/authoring_console_runtime.rs` drives a pointer press and release plus the `load` input through the official runtime, and `tests/playwright/showcase-validation.js` drives the same artifact in the browser.
- A complex interactive showcase is reproduced through the frontend: met. Issue #181 closed the exact-equivalence gate in PR #206, and the console reproduces stacking, waypoint continuity, a blend state, and parallel regions in one document without raw escapes.

This todo remains open. Additive blend states, direct blend states, transition duration and exit time, and view-model properties other than `bool` are not exposed by the AuthoringSpec frontend.
