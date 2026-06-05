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
- Structural parity: Medium (object tree matches, key properties now aligned)
- Visual parity: **Fail** (~100% pixel diff due to animation frame timing; both show gray background with ellipse but captured at different positions in pingpong loop)
- Gaps:
  - [property-drift] - generated objects include names where reference omits them
  - [encoding-difference] - file_id differs (generated uses 0, reference uses 11807)
  - [encoding-difference] - ToC property keys differ (reference includes 236, 376)
  - [visual-mismatch] - ellipse captured at different animation frame due to lack of frame sync in static screenshot (both files use autoplay=false but state machine may initialize at different times)

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
- Structural parity: Low (131 objects generated vs 409 in reference; missing all keyframes, state machine internals)
- Visual parity: **Fail** (reference shows animated fire button; generated shows only partial static shapes)
- Gaps:
  - [missing-type] - 278 objects missing: KeyFrameDouble (108), KeyedProperty (56), KeyedObject (35), StateTransition (8), CubicEaseInterpolator (10), AnimationState (6), TransitionBoolCondition (6), StateMachineLayer (2), EntryState (2), ExitState (2), AnyState (2), StateMachineBool (1), StateMachineListener (1), ListenerBoolChange (1)
  - [property-drift] - generated objects include synthetic names where reference omits them
  - [encoding-difference] - file_id differs (generated uses 0, reference uses 11807)
  - [visual-mismatch] - missing animations, gradients, and most visual elements
  - [property-drift] - converter does not emit animation keyframes, state machine states/transitions, or interpolators

## Fixed Gaps

The following parity gaps were identified and fixed:

1. **Artboard property keys**: Artboard was using NODE_X(13)/NODE_Y(14) for x/y. Fixed to use NODE_X_ARTBOARD(9)/NODE_Y_ARTBOARD(10).
2. **Artboard property ordering**: Reordered Artboard properties to match reference convention: name -> width -> height -> x -> y -> origin_x -> origin_y -> default_state_machine_id.
3. **Missing animation quantize**: LinearAnimation had a `quantize` field that was never emitted in `properties()`. Added emission when non-zero.
4. **Missing JSON schema fields**: Added `x`, `y`, `origin_x`, `origin_y` to `ArtboardSpec` and `quantize`, `work_start`, `work_end`, `enable_work_area` to `AnimationSpec` so users can set these from JSON.
5. **Node children support**: Added `children: Option<Vec<ObjectSpec>>` to `Node` in spec.rs and updated builder to recurse through children, enabling proper hierarchy construction from decompiled references.

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
| comparison_quantize_test | ~100% | Fail | Animation frame timing difference (expected for animated content) |
| comparison_clip_tests | N/A | No reference .riv | Only decompiled JSON available |
| comparison_official_test | ~100% | Fail | Missing animations, state machines, interpolators |

## How to add a new comparison
1. Acquire official .riv from rive-runtime repo
2. Place in demo/riv/reference/
3. Run `cargo run -- decompile demo/riv/reference/<file>.riv > demo/riv/reference/<file>.decompiled.json`
4. Create tests/fixtures/comparison_<name>.json
5. Add to tests/playwright/shared.js FIXTURES array
6. Add e2e test in tests/e2e.rs
7. Run Playwright visual regression with `--update` to create baselines
8. Run vision comparison: `node tests/playwright/vision-compare.js`
9. Record findings in this document

## Action Items
- [x] Create comparison fixture for clip_tests
- [x] Create comparison fixture for official_test
- [x] Measure structural parity for all comparison fixtures
- [x] Measure visual parity for all comparison fixtures (pixel diff)
- [x] Add e2e tests for all comparison fixtures
- [x] Implement vision comparison pipeline
- [x] Implement vision model API gate script
- [ ] Fix comparison_quantize_test animation quantize/work area drift
- [ ] Fix comparison_official_test missing animation/state machine data
- [ ] Automate decompile-diff measurement in CI
- [ ] Integrate vision model API for semantic likeness approval
- [ ] Acquire additional official .riv test files for broader coverage
