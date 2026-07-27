# Reference Parity Tracking

## What is this?
This document tracks how closely rive-cli-generated files match official Rive runtime files. The goal is structural and visual parity.

## Reference File Inventory

Fetched files are pinned to an upstream repository, commit and path in
`parity/official/manifest.json`; legacy `in-repo` files are checksum-pinned and verified locally.

|File|Source|Size|Paired Fixture|Status|
|---|---|---|---|---|
|trim.riv|rive-runtime test suite|184B|comparison_trim|partial|
|quantize_test.riv|rive-runtime test suite|200B|comparison_quantize_test|partial|
|official_test.riv|rive-runtime fire_button.riv|4407B|comparison_official_test|partial|
|fire_button.riv|rive-app/rive-runtime `renderer/webgpu_player/rivs/fire_button.riv` @ `40ff578`|4407B|—|byte-identical to official_test.riv|
|coffee_loader.riv|rive-app/rive-runtime `renderer/webgpu_player/rivs/coffee_loader.riv` @ `40ff578`|2605B|parity/reproductions/coffee_loader|**rung 2 pass**|
|button.riv|rive-app/rive-flutter `example/assets/button.riv` @ `4e4b87d`|806209B|parity/reproductions/button|**rung 1 pass**|
|clip_tests.decompiled.json|rive-runtime|21KB|comparison_clip_tests|partial|

`official_test.riv` and `fire_button.riv` hash identically
(`7c084a8aa6d4589a8b506ad14ad8d99a7358fbb49ac69b8cf1a8a9fc422879c4`). The file that had been carried
in-tree as `official_test.riv` since before this corpus existed **is** upstream `fire_button.riv`.
Both names are retained: `official_test.riv` is referenced by existing docs, tests and the demo,
while `fire_button.riv` records the provenance. `gapType: encoding-difference` (naming only).

## Reproduction ladder

Ground truth is an official, Rive-authored `.riv`. A rung passes when the reproduction reaches
**≤5% maximum pixel difference with no Rive type name missing from the candidate**. Every number
below comes from `rive-cli compare` at `--width 512 --height 512 --scale 2 --background '#0B0E17'`,
and is re-collated into `parity/results.json` by `parity/collate-results.sh`.

|Rung|Official|Objects (official/ours)|Frames|Max pixel diff|Verdict|
|---|---|---|---|---|---|
|1|`button.riv`|64 / 64|0, 15, 30, 45|**0.0000%**|pass|
|2|`coffee_loader.riv`|250 / 247|0, 15, 30, 45|**0.2833%**|pass|
|—|`trim.riv`|15 / 15|0|0.0000%|not a gate, see below|

### Rung 0 — `trim.riv` was discarded as a gate

`parity/reproductions/trim.json` matches the reference's 15-object structure and type histogram,
including the rendered colours, and reaches 0.0000% pixel difference at 512x512. The reproduction
must add names required by SceneSpec, so its named property sets and emission order are not identical.

It is nonetheless **not a meaningful visual gate**. Its `TrimPath` has `modeValue: 2` and default
`start`/`end`, which trims the entire stroke away, so the only thing that draws is the artboard-level
`Fill`. `rive-cli render` reports 2 distinct colours at 512x512 and `BLANK` at 960x540 — for the
official file as much as for ours. A 0.0000% match between two near-empty frames proves little, so
per the plan's contingency the ladder was shifted down and `button.riv` became rung 1. The files stay
committed as evidence; the gallery does not show them.

Two findings came out of it and are fixed:

- **Artboard property key 9 was unnamed by the decompiler.** `NODE_X_ARTBOARD`/`NODE_Y_ARTBOARD`
  (keys 9 and 10, defined on `NodeBase` as `xArtboard`/`yArtboard`) had no entry in
  `generated_registry::property_name`, so `decompile` printed `key9` and a reproduction could not see
  what value to set. Both keys now resolve; pinned by
  `test_artboard_x_and_y_resolve_to_named_properties`. `gapType: property-drift`.
- **`fill` under an artboard was mis-documented, not rejected.** `build_scene` has always accepted a
  `fill` as a direct artboard child (the artboard is a `LayoutComponent`, hence a
  `ShapePaintContainer`), but `rive-cli describe fill` advertised `valid_parents: ["shape"]`. The
  discovery table now reports `shape, artboard, layout_component` for both `fill` and `stroke`.
  `gapType: property-drift`.

The official file leaves `Shape` at local 3 unnamed, while our JSON requires names for keyframe
targeting. This expected structural divergence is labelled `gapType: encoding-difference`; only
structural and visual equivalence are targeted.

### Rung 1 — `button.riv`

64 objects, 31 distinct types, an 805 kB embedded Inter variable font, a text run with two
`TextStyleAxis` variation axes, three animations and a listener-driven state machine.

**Result: 64/64 objects, empty type-delta table, 0.0000% at frames 0, 15, 30 and 45** driven through
`State Machine 1`, and 0.0000% at frames 0 and 30 driven through the `Down` animation.

The font is extracted from the official file's `FileAssetContents` into
`parity/reproductions/assets/Inter.ttf` (805528 B,
`bfff5663c84b220f3c6dbb0e5225c66eab3d79e0d67351bbac151b5109c78a2d`) and re-embedded via
`font_asset.source`.

One defect was found and fixed:

- **`text` silently dropped `x` and `y`.** `ObjectSpec::Text` had no `x`/`y` fields, so a scene
  setting them parsed cleanly and emitted nothing — `Text` extends `Node` and owns keys 13/14. The
  label rendered 63.7 px off, which was the entire 0.5011% difference on the first attempt. With the
  fields wired the rung went to 0.0000%. `gapType: property-drift`.

Remaining drift, all confirmed visually inert at every compared frame:

- `Artboard.styleId` (key 494) cannot be set from JSON; `ArtboardSpec` has no `style_id`.
  `gapType: property-drift`. Tracking: [#123](https://github.com/George-RD/rive-rs-cli/issues/123).
  Acceptance: the schema accepts `style_id` and decompile/generate preserve it.
- `LayoutComponentStyle` emits none of the reference's ten `*UnitsValue` properties; the spec exposes
  `margin_left` but not `margin_left_units_value`. `gapType: property-drift`. Tracking:
  [#123](https://github.com/George-RD/rive-rs-cli/issues/123). Acceptance: all reference unit properties
  are expressible and round-trip without changing layout.
- `Rectangle` emits all four corner radii plus `linkCornerRadius`; the reference emits only
  `cornerRadiusTL` and relies on the link default. `gapType: encoding-difference`. Expected and
  accepted while rendered geometry remains equal.
- `LinearAnimation` emits `fps` and `duration` because `AnimationSpec` requires them; the reference
  omits both and relies on the 60/60 defaults. Same values, so no behavioural difference.
  `gapType: encoding-difference`. Expected and accepted while animation timing remains equal.
- `KeyFrameDouble` emits `frame: 0` and `interpolatorId: 4294967295` where the reference omits both.
  `gapType: encoding-difference`. Expected and accepted while keyframe values remain equal.
- `StateTransition` cannot carry `flags`, `interpolationType` or `interpolatorId`; `TransitionSpec`
  has no fields for them, so the reference's eased 200 ms transitions become linear in ours.
  `gapType: property-drift`. Tracking: [#125](https://github.com/George-RD/rive-rs-cli/issues/125).
  Acceptance: transition metadata is expressible and the reproduced transition uses the reference
  interpolation behavior.

### Rung 2 — `coffee_loader.riv`

250 objects, 30 distinct types, five state-machine layers including a 1D blend state, nine
artboard-scoped `CubicEaseInterpolator`s shared across seven animations, and ninety keyframes.

**Result: 250/247 objects, 0.2833% maximum across frames 0, 15, 30 and 45.** The reproduction went
28.3521% → 7.2622% → 0.2833% as each of the three defects below was fixed.

Defects found and fixed:

- **No transform component could set `rotation`, `scale_x` or `scale_y` from JSON.** `node` and
  `shape` exposed only `x`/`y`; the five parametric shapes (`ellipse`, `rectangle`, `triangle`,
  `polygon`, `star`) exposed none of the five. All of them are `Node` subclasses owning keys 13-17,
  and `rive-cli describe rectangle` already advertised all five as *animatable* — so the tool let you
  keyframe a property it would not let you set. This was the largest single error, worth roughly 21
  percentage points of pixel difference on this file. `gapType: property-drift`.
  It was also silently corrupting existing work: `showcase/pulse_button.json` and
  `showcase/rocket_launch.json` both set `rotation` on shapes and had been ignored since they were
  authored, and `tests/fixtures/comparison_official_test.json` carried four `scale_x`/`scale_y`
  values transcribed from the reference that were never emitted. All three now render as authored;
  four PNG baselines were updated to match.
- **`stroke` could not set `transformAffectsStroke`.** `ObjectSpec::Stroke` had no field, and the
  `Stroke` object defaulted the value to `0` and emitted only when non-zero — so `false` was
  unrepresentable while `true` was implied by omission. The runtime default is `true`, so the object
  now defaults to `1` and emits only when `false`. Existing scenes are byte-identical. Worth roughly
  7 percentage points here, because the smoke and cup strokes sit under scales from 0.47 to 2.13.
  `gapType: property-drift`.
- **`shape` could not be hidden.** The reference marks one shape `drawableFlags: 1`
  (`DrawableFlag::Hidden`). `shape` now takes `hidden: true`. `gapType: property-drift`.

Unresolved, and the reason this rung is 247 objects rather than 250:

- **`stroke.thickness` cannot be keyframed.** The `Stop` animation keyframes key 47 on the stroke at
  local 47. `AGENTS.md` states that strokes expose `is_visible`, not `thickness`, and that rule is
  respected here, so one `KeyedObject`/`KeyedProperty`/`KeyFrameDouble` triple is absent — the whole
  3-object delta. It is visually inert in this file: the single keyframe sets thickness to `6.0`,
  which is already the stroke's static value. Upstream `dev/defs/shapes/paint/stroke.json` at `40ff578`
  marks `thickness` `"animates": true`, and this official file exercises it. `gapType: missing-type`
  (property coverage). Tracking: [#125](https://github.com/George-RD/rive-rs-cli/issues/125).
  Acceptance: a thickness keyframe produces the missing three objects and matches the official
  animation without violating the stroke property contract.
- **`origin_x: 0.0` is unrepresentable.** `Rectangle`/`Ellipse` default `origin_x`/`origin_y` to
  `0.0` and emit only when non-zero, but the Rive default is `0.5`, so an author asking for `0.0`
  gets `0.5`. The reference's `Loader_Fill` rectangle sets `originX: 0.0`; the reproduction
  compensates with `x: 7.9036255` (half the width). `tests/fixtures/constraints.json` already sets
  `origin_x: 0.0` and is silently getting `0.5`. `gapType: property-drift`, unfixed. Tracking:
  [#126](https://github.com/George-RD/rive-rs-cli/issues/126). Acceptance: explicit zero differs from
  omitted and survives generation, validation and decompilation.
- **Interpolators are artboard-scoped when emitted but validated per animation.**
  `register_interpolators` walks every animation and emits each named interpolator once at artboard
  scope, but `validate_artboard_spec` resolves a keyframe's `interpolator` only against its own
  animation's list. Sharing one interpolator across animations therefore requires repeating the whole
  `interpolators` array in each animation that references it. `gapType: property-drift`. Tracking:
  [#125](https://github.com/George-RD/rive-rs-cli/issues/125). Acceptance: a single artboard-scoped
  interpolator declaration resolves from every animation that references its name.
- **Duplicate object names are rejected.** The reference has two nodes named `Smoke`; the
  reproduction calls them `Smoke_A` and `Smoke_B`. Names do not affect rendering.
  `gapType: encoding-difference`. Expected and accepted while object targeting remains unambiguous.
- Object emission order differs throughout: our builder writes parents before children depth-first,
  while the editor emits paints after the shapes that own them. `parentId` relationships are
  identical, so the runtime resolves the same tree. `gapType: encoding-difference`. Expected and
  accepted while hierarchy and rendered output remain equal.

The residual 0.2833% is not attributed to a specific cause. It is stable across frames, survives the
three fixes above, and is consistent with antialiasing along the cup outline given the
`origin_x` compensation.

## Comparison Fixtures

### comparison_trim
- Target: recreate trim.riv using PointsPath + StraightVertex (original uses Path + CubicVertex)
- Structural parity: High (same object tree, different path representation)
- Visual parity: **Pass** (0.0000% pixel diff vs reference)
- Gaps:
  - [encoding-difference] - different path types used (PointsPath/StraightVertex vs Path/CubicVertex)
  - [property-drift] - generated objects include names where reference omits them (SolidColor, Fill, Stroke, etc.)
  - [property-drift] - color values differ by design in fixture
  - [encoding-difference] - file_id differs (generated uses 0, reference uses 9553)

### comparison_quantize_test
- Target: recreate quantize_test.riv
- Structural parity: **Pass** (semantic parity = 0 via `scripts/parity_metric.py`)
- Visual parity: Not measured (local Playwright server did not start on port 8767); the animation is now one-shot with the same start/end values as the reference
- Gaps:
  - [encoding-difference] - generated objects include synthetic component names where the reference omits them; the runtime resolves by index, so this is cosmetic
  - [encoding-difference] - reference ToC declares 236 and 376 as unknown properties; our encoder knows them natively and correctly omits them from the ToC (including known props would break WASM runtime import)
  - [encoding-difference] - default-valued properties (work area 0,0, frame-0 keyframe value 0.0, no interpolator) are omitted by the official encoder and written explicitly by ours; semantics are identical
  - [visual-mismatch] - no pixel-diff measurement exists; the local Playwright server did not start on port 8767, so visual parity is unverified

### comparison_clip_tests
- Target: recreate clip_tests.riv from decompiled reference
- Structural parity: Medium (same types and hierarchy, naming and property emission differ)
- Visual parity: **Pass** (0.0000% pixel diff against own baseline; no reference .riv for direct comparison)
- Gaps:
  - [property-drift] - generated objects include names where reference omits them (Ellipse, Rectangle, SolidColor, Fill, ClippingShape)
  - [property-drift] - reference artboards have non-zero x positions (600.0, 1200.0); fixture sets x=0
  - [encoding-difference] - file_id differs (generated uses 0, reference uses 13748)
  - [encoding-difference] - ToC property keys differ (reference includes 236)
  - [missing-type] - no reference .riv file exists; only decompiled JSON available

### comparison_official_test
- Target: recreate official_test.riv (fire button with gradients, bones, animations, state machine)
- Structural parity: Low (131 objects generated vs 407 in reference; missing all keyframes, state machine internals). The reference count was previously recorded as 409 because the ToC parser was misaligned and produced two phantom `type=0` objects.
- Visual parity: **Fail** (reference shows animated fire button; generated shows only partial static shapes)
- Gaps:
  - [missing-type] - 276 objects missing: KeyFrameDouble (108), KeyedProperty (56), KeyedObject (35), StateTransition (8), CubicEaseInterpolator (10), AnimationState (6), TransitionBoolCondition (6), StateMachineLayer (2), EntryState (2), ExitState (2), AnyState (2), StateMachineBool (1), StateMachineListener (1), ListenerBoolChange (1)
  - [property-drift] - generated objects include synthetic names where reference omits them
  - [encoding-difference] - file_id differs (generated uses 0, reference uses 562856)
  - [visual-mismatch] - missing animations, gradients, and most visual elements
  - [property-drift] - converter does not emit animation keyframes, state machine states/transitions, or interpolators

## Fixture Runtime Gaps

Fixtures the vendored `@rive-app/canvas` runtime (`assets/rive.js`, version 2.39.1) cannot load. They are listed in `KNOWN_RUNTIME_GAPS` in `tests/playwright/shared.js`, so `tests/playwright/regression.js` prints `SKIP <name> (known runtime gap)` instead of failing. Removing a fixture from that set is the acceptance criterion for closing its row.

**These are unresolved, and the original explanation was wrong.** Both rows were first recorded as version lag: the object types postdated the then-vendored 2.35.2 runtime. Two pieces of evidence refute that.

1. Upgrading the vendored bundle from 2.35.2 to 2.39.1 (current latest) changed nothing. `transition_comparators` still logs `Failed to import object of type 481/483/484/486`, and `scripting` still fails.
2. An audit of `rive-runtime` main (`e0d4913`, `runtime-v0.1.217`) found every one of these keys present, class-level `runtime: true`, and constructed by `CoreRegistry::makeCoreInstance` — for example `case TransitionValueBooleanComparatorBase::typeKey: return new TransitionValueBooleanComparator();`. None is editor-only, mis-numbered, or an abstract parent. The abstract comparator parent is key 480, which we do not emit.

So the keys are correct and instantiable, and the failure is not a missing core factory entry. `Failed to import object of type <N>` is not the message `src/file.cpp` prints when the factory misses — that one is `Unknown property key ...`. The remaining suspect is therefore the importer rejecting a correctly-constructed object because of where it sits in the hierarchy, which is the same class of defect already found in `follow_path_constraint` and `nslicer`. **That is a hypothesis, not a finding: it has not been reproduced with a corrected hierarchy.** Confirming or refuting it is the next step, and the skips stay until a fixture demonstrably loads.

| Fixture | gapType | Status |
|---|---|---|
| scripting | runtime-rejection | Unresolved. Emits types 603, 611, 612, 618, 621, 626, 627, 629, 631; the runtime imports 603/611/629 and rejects the other six, then fails the load. Cause not established. |
| transition_comparators | runtime-rejection | Unresolved, but degraded gracefully: since the ToC fix the file **loads** (`onLoad` fires) and the runtime skips types 481/483/484/486. It stays listed only because the harness treats the `Failed to import object of type <N>` console lines as errors. |
| bidirectional bool transitions | visual-mismatch | Unresolved. A pair of transitions between two animation states conditioned on the same bool (`A -> B` when true, `B -> A` when false) makes the runtime return to `A` a few frames after reaching `B`, even though the input stays true. Reproduced with `tests/fixtures/pointer_interaction.json` at transition durations 0 and 6, with both a chained and an `any`-state topology; one-way transitions behave correctly at every frame sampled. Acceptance: a two-state bool toggle holds `B` for at least 60 frames while the input stays true. Until then, author one-way transitions, or drive the return edge from a second input. |

### Fixed in this cycle

All five original entries were root-caused rather than suppressed; three were writer or fixture defects, not runtime limits.

| Fixture | Root cause | Fix |
|---|---|---|
| follow_path_constraint | The constraint was parented to the artboard while targeting a sibling shape, so the artboard depended on its own descendant. Runtime reported `Dependency cycle!`. | Reparented the constraint under the constrained `Follower` shape, matching `constraints.json`. |
| mesh | `MeshBase` (type 109) declares `triangleIndexBytes` (key 223, `CoreBytesType`). The runtime allocates its index buffer from that property; omitting it leaves the buffer null and `onAddedClean` returns `InvalidObject`. Keys 219/220 also surfaced as `Unknown property key ... missing from property ToC` because the ToC was empty. | `Mesh` now always emits key 223 (empty payload). Bytes and String share field id 1, so it is declared in the ToC as a string. |
| nslicer | Two defects. (a) Property keys 697-700 (`initialWidth`/`initialHeight`/`width`/`height`) were emitted on `NSlicer`, but `NSlicerBase` declares no fields — they belong to `NSlicedNode` (`rive-runtime/src/layout/n_sliced_node.cpp:55-69,138-140`). (b) `NSlicer` must be a direct child of an `Image` (`src/layout/n_slicer.cpp:11-34`); the fixture nested it under an `NSlicedNode` instead, and the combined constructs stalled the runtime with neither callback firing. | Moved the four size properties to `NSlicedNode` in the object model, spec, and builder. Split the fixture into two artboards: `ImageSlice` (`Image -> NSlicer -> Axis*`) and `VectorSlice` (`NSlicedNode -> Axis* + vector content`), which are the two mutually exclusive constructs the runtime supports. |

`ForcedEdge` (type 112) is retained but inert: `dev/defs/shapes/forced_edge.json` in rive-runtime is `runtime: false`, so it has no generated base and no `core_registry` entry. Runtimes skip it. Its properties 219/220 are likewise unregistered, which is why they need ToC declarations to be skippable.

## Fixed Gaps

The following parity gaps were identified and fixed:

1. **Artboard property keys**: Artboard was using NODE_X(13)/NODE_Y(14) for x/y. Fixed to use NODE_X_ARTBOARD(9)/NODE_Y_ARTBOARD(10).
2. **Artboard property ordering**: Reordered Artboard properties to match reference convention: name -> width -> height -> x -> y -> origin_x -> origin_y -> default_state_machine_id.
3. **Missing animation quantize**: LinearAnimation had a `quantize` field that was never emitted in `properties()`. Added emission when non-zero.
4. **Missing JSON schema fields**: Added `x`, `y`, `origin_x`, `origin_y` to `ArtboardSpec` and `quantize`, `work_start`, `work_end`, `enable_work_area` to `AnimationSpec` so users can set these from JSON.
5. **Node children support**: Added `children: Option<Vec<ObjectSpec>>` to `Node` in spec.rs and updated builder to recurse through children, enabling proper hierarchy construction from decompiled references.
6. **ToC backing-code packing**: the encoder packed 16 two-bit codes per `u32` and the parser decoded the same way, but `rive-runtime/include/rive/runtime_header.hpp:87-104` reloads a fresh `u32` every 4 codes. Any file with more than 4 ToC keys was therefore misaligned by 4 bytes per extra word. `parity/official/official_test.riv` (8 ToC keys) previously decompiled to 409 objects led by two phantom `type=0` entries and failed `validate`; it now parses to 407 objects starting at Backboard. The committed reference decompiles were regenerated.
7. **Empty ToC**: `encode_riv` only added keys whose backing type was *unknown*, while `encode_toc` panicked on exactly those keys, so the ToC was always empty by construction. The encoder now declares every property key the file writes. This is what allows a runtime to skip object types it does not know instead of aborting the object stream.
8. **File assets nested inside the artboard**: `image_asset`/`font_asset`/`audio_asset` were emitted as artboard children, so the runtime's artboard-local index space disagreed with the writer's by one per asset. Every `parentId` after an asset resolved to the wrong object; a scene with an asset plus any drawable made `@rive-app/canvas` 2.39.1 hang indefinitely instead of reporting an error. Assets are now hoisted to file scope, between the Backboard and the first Artboard, which is where Rive's own exporter puts them. `gapType: encoding-difference`.
9. **`text_style` emitted `TextStyle` (573) instead of `TextStylePaint` (137)**: `TextStyle` is a plain `ContainerComponent` in the runtime; only its subclass `TextStylePaint` implements `ShapePaintContainer` and draws glyphs. Text therefore rendered nothing no matter what font or fill was attached, and every committed text baseline was a flat colour. `text_style` now emits 137 and accepts `fill`/`stroke` children. `gapType: missing-type`.
10. **Unscheduled `--input` was discarded by `play()`**: the render harness applied state machine inputs before `instance.play()`, which rebuilds the state machine instance and resets them. Inputs are now applied immediately after the first advance, through the same path as scheduled inputs. `gapType: visual-mismatch`.

## Gap Type Vocabulary
- `missing-type`: Object type exists in reference but not in rive-cli
- `property-drift`: Property values differ between reference and generated
- `encoding-difference`: Same semantic content encoded differently (e.g. Path vs PointsPath)
- `visual-mismatch`: Pixel diff exceeds threshold or images differ semantically
- `runtime-rejection`: Generated file fails to load in official runtime
- `not-measured`: Comparison exists but hasn't been evaluated

## Vision Model Approval Gates

A vision comparison pipeline is available at `tests/playwright/vision-compare.js`. It renders both reference and generated `.riv` files side-by-side and computes pixel-level diff.

To run:
```bash
node tests/playwright/vision-compare.js
```

Results are written to `target/playwright-vision/` as PNG screenshots.

For fixtures with non-zero diff, a vision model API (e.g. GPT-4V, Claude Vision) can be integrated to determine semantic likeness rather than requiring pixel-perfect identity. The comparison script outputs structured results that can be fed into such an API.

A vision model gate script is available at `tests/playwright/vision-model-gate.js`. It sends reference and generated screenshots to OpenAI's GPT-4o vision model for semantic likeness judgment. To use:

```bash
export OPENAI_API_KEY=sk-...
node tests/playwright/vision-model-gate.js
```

A multi-provider vision gate orchestrator is available at `scripts/vision_gate_orchestrator.py`. It supports OpenAI GPT-4o, Anthropic Claude 3 Opus, and Google Gemini Pro Vision in parallel. To use:

```bash
export OPENAI_API_KEY=sk-...
export ANTHROPIC_API_KEY=sk-ant-...
export GOOGLE_API_KEY=...
python3 scripts/vision_gate_orchestrator.py
```

### Current Vision Comparison Results

| Fixture | Pixel Diff | Status | Notes |
|---|---|---|---|
| comparison_trim | 0.0000% | Pass | Pixel-perfect match |
| comparison_quantize_test | Not measured | Structural pass | Pingpong loop removed; animation start/end values now match reference. Visual comparison attempted but local server did not start on port 8767. |
| comparison_clip_tests | N/A | No reference .riv | Only decompiled JSON available |
| comparison_official_test | ~100% | Fail | Missing animations, state machines, interpolators |

## How to add a reproduction rung
1. Add the official file to `parity/official/manifest.json` with its upstream repo, commit SHA, path,
   SHA-256 and byte size, then run `bash parity/fetch-official.sh` to download and verify it.
2. `rive-cli decompile parity/official/<name>.riv` and read the object tree.
3. Author `parity/reproductions/<name>.json`, generate `parity/reproductions/<name>.riv`, commit both.
4. `rive-cli compare parity/official/<name>.riv parity/reproductions/<name>.riv --frames … --json`
5. Add the rung to `RUNGS` in `parity/collate-results.sh` and re-run it to refresh
   `parity/results.json`; the site gallery and `tests/playwright/site-validation.js` read that file.
6. Bump `EXPECTED_SCENES` in `tests/playwright/site-validation.js` to `2 * rungs + 1`.
7. Record the numbers and every divergence in the Reproduction ladder section above, each labelled
   with a `gapType`.

## How to add a comparison fixture (older workflow)
1. Acquire official .riv from rive-runtime repo
2. Place in `parity/official/`
3. Run `cargo run -- decompile parity/official/<file>.riv > parity/official/<file>.decompiled.json`
4. Create tests/fixtures/comparison_<name>.json
5. Add to `tests/playwright/shared.js` `FIXTURES` (runtime regression). Fixtures with committed PNG baselines are the ones absent from `RUNTIME_ONLY_FIXTURES`; add the name there too if it should stay runtime-only.
6. Add e2e test in tests/e2e.rs
7. Run Playwright visual regression with `--update` to create baselines
8. Run vision comparison: `node tests/playwright/vision-compare.js`
9. Record findings in this document

## Action Items
- [x] Create comparison fixture for clip_tests
- [x] Create comparison fixture for official_test
- [x] Measure structural parity for all comparison fixtures
- [ ] Measure visual parity for comparison_quantize_test (pixel diff never captured)
- [x] Add e2e tests for all comparison fixtures
- [x] Implement vision comparison pipeline
- [x] Implement vision model API gate script
- [x] Fix comparison_quantize_test animation quantize/work area drift
- [ ] Fix comparison_official_test missing animation/state machine data
- [ ] Automate decompile-diff measurement in CI
- [ ] Integrate vision model API for semantic likeness approval
- [ ] Acquire additional official .riv test files for broader coverage
- [x] Root-cause three of the five `runtime-rejection` fixtures (`follow_path_constraint`, `mesh`, `nslicer` were writer/fixture defects and are fixed)
- [x] Upgrade the vendored runtime to `@rive-app/canvas` 2.39.1 — did **not** close the remaining two rows, refuting the version-lag theory
- [ ] Establish why the importer rejects types 481/483/484/486 and 612/618/621/626/627/631 when `CoreRegistry::makeCoreInstance` constructs all of them; the leading hypothesis is object placement in the fixture hierarchy, unverified
- [x] Pin the official corpus in `parity/official/` with SHA-256 provenance and a refresh script
- [x] Add `rive-cli compare` (structural type-delta table plus per-frame pixel difference)
- [x] Walk the reproduction ladder: `button.riv` at 0.0000%, `coffee_loader.riv` at 0.2833%
- [x] Point the site at the measured comparison instead of self-authored showcases
- [ ] Make `origin_x: 0.0` representable on parametric shapes (currently indistinguishable from unset,
      so `tests/fixtures/constraints.json` silently gets 0.5). Tracking:
      [#126](https://github.com/George-RD/rive-rs-cli/issues/126); acceptance: explicit zero survives
      generation, validation and decompilation.
- [ ] Let a keyframe reference an interpolator declared in another animation without repeating the
      whole `interpolators` array. Tracking: [#125](https://github.com/George-RD/rive-rs-cli/issues/125);
      acceptance: one artboard-scoped declaration resolves from every referencing animation.
- [ ] Expose `StateTransition` `flags`, `interpolationType` and `interpolatorId` in `TransitionSpec`.
      Tracking: [#125](https://github.com/George-RD/rive-rs-cli/issues/125); acceptance: transition
      metadata round-trips and reproduces the reference interpolation behavior.
- [ ] Expose `Artboard.style_id` and `LayoutComponentStyle`'s `*_units_value` properties. Tracking:
      [#123](https://github.com/George-RD/rive-rs-cli/issues/123); acceptance: all reference properties
      are expressible and layout output remains unchanged when defaults are used.
- [ ] Decide whether `stroke.thickness` should be keyframable; upstream marks it `"animates": true`
      and `coffee_loader.riv` uses it, but `AGENTS.md` forbids it. Tracking:
      [#125](https://github.com/George-RD/rive-rs-cli/issues/125); acceptance: the decision is encoded
      in the property resolver and the coffee-loader object delta is either removed or documented.
- [ ] Widen the corpus beyond three reproductions now that the ladder is walked. Tracking:
      [#125](https://github.com/George-RD/rive-rs-cli/issues/125); acceptance: each added official file
      has a pinned manifest entry, reproduction, compare result and site validation coverage.
