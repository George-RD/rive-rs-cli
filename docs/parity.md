# Reference Parity Tracking

## What is this?
This document tracks how closely rive-cli-generated files match official Rive runtime files. The goal is structural and visual parity.

## Reference File Inventory

|File|Source|Size|Paired Fixture|Status|
|---|---|---|---|---|
|trim.riv|rive-runtime test suite|184B|comparison_trim|partial|
|quantize_test.riv|rive-runtime test suite|200B|comparison_quantize_test|partial|
|official_test.riv|rive-runtime fire_button.riv|4407B|comparison_official_test|partial|
|clip_tests.decompiled.json|rive-runtime|21KB|comparison_clip_tests|partial|

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
6. **ToC backing-code packing**: the encoder packed 16 two-bit codes per `u32` and the parser decoded the same way, but `rive-runtime/include/rive/runtime_header.hpp:87-104` reloads a fresh `u32` every 4 codes. Any file with more than 4 ToC keys was therefore misaligned by 4 bytes per extra word. `demo/riv/reference/official_test.riv` (8 ToC keys) previously decompiled to 409 objects led by two phantom `type=0` entries and failed `validate`; it now parses to 407 objects starting at Backboard. The committed reference decompiles were regenerated.
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

## How to add a new comparison
1. Acquire official .riv from rive-runtime repo
2. Place in demo/riv/reference/
3. Run `cargo run -- decompile demo/riv/reference/<file>.riv > demo/riv/reference/<file>.decompiled.json`
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
