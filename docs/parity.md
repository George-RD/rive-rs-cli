# Reference Parity Tracking

## What is this?
This document tracks how closely rive-cli-generated files match official Rive runtime files. The goal is structural and visual parity.

## Reference File Inventory

|File|Source|Size|Paired Fixture|Status|
|---|---|---|---|---|
|trim.riv|rive-runtime test suite|184B|comparison_trim|partial|
|quantize_test.riv|rive-runtime test suite|200B|comparison_quantize_test|partial|
|official_test.riv|rive-runtime fire_button.riv|4407B|official_test|N/A (compatibility)|
|clip_tests.decompiled.json|rive-runtime|21KB|NONE|not started|

## Comparison Fixtures

### comparison_trim
- Target: recreate trim.riv using PointsPath + StraightVertex (original uses Path + CubicVertex)
- Structural parity: High (same object tree, different path representation)
- Visual parity: Unknown (no pixel diff measured)
- Gaps: [encoding-difference] - different path types used

### comparison_quantize_test
- Target: recreate quantize_test.riv
- Structural parity: Unknown
- Visual parity: Unknown
- Gaps: [not-measured]

## Gap Type Vocabulary
- `missing-type`: Object type exists in reference but not in rive-cli
- `property-drift`: Property values differ between reference and generated
- `encoding-difference`: Same semantic content encoded differently (e.g. Path vs PointsPath)
- `visual-mismatch`: Pixel diff exceeds threshold
- `runtime-rejection`: Generated file fails to load in official runtime
- `not-measured`: Comparison exists but hasn't been evaluated

## How to add a new comparison
1. Acquire official .riv from rive-runtime repo
2. Place in demo/riv/reference/
3. Run `cargo run -- decompile demo/riv/reference/<file>.riv > demo/riv/reference/<file>.decompiled.json`
4. Create tests/fixtures/comparison_<name>.json
5. Add to demo/serve.js FIXTURE_OVERRIDES with hasReference=true
6. Run demo and compare side-by-side
7. Record findings in this document

## Action Items
- [ ] Create comparison fixture for clip_tests
- [ ] Measure structural parity for comparison_trim (decompile diff)
- [ ] Measure visual parity for comparison_trim (pixel diff)
- [ ] Measure structural parity for comparison_quantize_test
- [ ] Measure visual parity for comparison_quantize_test
- [ ] Add e2e tests for all comparison fixtures
- [ ] Automate decompile-diff measurement in CI
