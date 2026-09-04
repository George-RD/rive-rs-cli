# Testing Strategy

## Overview

This document defines how rive-rs-cli is tested at every layer, how the fixture corpus grows over time, and how visual regression catches animation rendering differences.

**Goal**: every generated `.riv` file is structurally valid, loads in official Rive runtimes, and renders identically across encoder changes.

## Test Layers

### 1. Unit Tests (Rust, inline)

Each module has `#[cfg(test)] mod tests` with focused assertions.

| Module | What's tested | Example |
|--------|---------------|---------|
| `objects/*.rs` | type_key values, property emission, default omission | `test_trim_path_type_key` |
| `encoder/binary_writer.rs` | LEB128 encoding, float/string/color serialization | `test_write_varuint` |
| `encoder/toc.rs` | ToC bit-packing, multi-chunk encoding | `test_toc_encode_17_keys` |
| `builder/scene.rs` | JSON parsing, validation errors, object wiring | `test_trim_path_rejects_shape_parent` |
| `validator/mod.rs` | Binary parsing, property deserialization | `test_parse_riv` |

**Convention**: when adding a new object type, add at minimum:

- `test_{type}_type_key` — verify type key matches C++ constant
- `test_{type}_default_properties` — verify only non-default properties emitted
- `test_{type}_properties` — verify property keys and values

### 2. End-to-End Tests (`tests/e2e.rs`)

CLI subprocess tests that exercise the full pipeline:

```text
JSON fixture → cargo run -- generate → .riv file → cargo run -- validate → exit 0
                                                  → cargo run -- inspect  → verify output
```

Each fixture in `tests/fixtures/` gets at least one e2e test:

- `test_generate_{fixture}` — generates .riv, validates it (required for every valid fixture)
- `test_inspect_{fixture}` — optional, for select fixtures to verify inspect output structure
- `test_generate_invalid_{fixture}` — required for every `invalid_*` fixture

**Convention**: every new fixture added to `tests/fixtures/` must have a corresponding e2e test in `tests/e2e.rs`.

**Convention**: a negative test for an `invalid_*` fixture must assert a specific stderr substring, not merely a non-zero exit. An exit-code-only assertion passes for any failure, including failures unrelated to what the fixture is named for.

### 3. Playwright Runtime Regression (`tests/playwright/regression.js`)

Loads generated `.riv` files in the official `@rive-app/canvas` WASM runtime via a browser harness. Catches issues that pass structural validation but fail at runtime (wrong object hierarchy, missing properties, encoding quirks).

**How it works**:

1. Generates `.riv` from each fixture JSON via `cargo run -- generate`
2. Starts a local HTTP server serving the harness and `.riv` files
3. Launches headless Chromium via Playwright
4. Loads each `.riv` in the Rive canvas runtime
5. Fails on runtime errors, load failures, or console errors, except names in `KNOWN_RUNTIME_GAPS`

The harness explicitly sets the canvas backing-store dimensions before capture. This matters: the old harness left the backing store at the browser default of 300×150 and merely upscaled it to the 1024×1024 screenshot.

**Fixture lists** (`tests/playwright/shared.js`): `FIXTURES` is every fixture enrolled in runtime regression. `RUNTIME_ONLY_FIXTURES` have no committed PNG baseline; `VISUAL_FIXTURES` is the complement. `KNOWN_RUNTIME_GAPS` are documented in `docs/parity.md`.

**Runtime version**: both consumers use `@rive-app/canvas` 2.39.1, and the vendored JS/WASM pair must come from the same package version.

### 4. Golden-Frame Visual Regression (`tests/playwright/visual-regression.js`)

Pixel-level comparison of rendered frames against committed baseline PNGs.

**How it works**:

1. Loads each fixture in a controlled Rive canvas with autoplay disabled
2. Seeks each planned frame with `Rive.scrub([animationName], seconds)`
3. Resolves the runtime animation frame, then captures the correctly sized canvas
4. Compares against baselines in `tests/playwright/baselines/`

The previous claim that frame capture was deterministic was false: `scrub(undefined, t)` is a silent no-op in `@rive-app/canvas` 2.39.1 because the runtime maps an undefined animation list to an empty list. The harness now passes the mounted animation name explicitly. The previous 300×150 backing-store bug was fixed as described above. Baselines were regenerated after both corrections.

**Frame capture plan** (`shotPlanForFixture()`):

| Fixture category | Frames captured | Why |
|------------------|-----------------|-----|
| Static | f0 only | No animation |
| Linear animation | f0, f30, f60 | Start, midpoint, end |
| Cubic easing | f0, f15, f30, f45, f60 | Easing curve shape |
| Multi-artboard | f0, f30 | Opacity fade and X slide |
| Nested artboard | f0 only | Static embedding |
| State machine | f0 only | Static initial state |

**Updating baselines**:

```bash
npx -y -p playwright node tests/playwright/visual-regression.js --update
```

**Resolution**: 512×512 logical viewport with `deviceScaleFactor: 2`, producing 1024×1024 PNGs.

**Diff threshold**: 1.0% of pixels by default, configurable with `VISUAL_DIFF_THRESHOLD`.

### 5. CLI Render Verification (`src/render/`)

`rive-cli render` exercises the same embedded Rive JS/WASM runtime through headless Chromium over CDP, without Node or Playwright. It captures one or more animation or state-machine frames as PNGs and writes `manifest.json`; `--contact-sheet` writes a filmstrip, and `--preview` writes ASCII coverage maps plus dominant colour, distinct-colour count, blank status, and non-background bounds. Frame time is derived from `frame / fps`, so repeated renders of the same input are byte-identical.

Render verification is intentionally separate from Playwright visual baselines: it is a user-facing smoke test and an agent-facing inspection path, while Playwright remains the committed pixel-regression gate.


## Fixture Corpus

### Current Fixtures

| Fixture | Category | Objects | Animations | Golden frames |
|---------|----------|---------|------------|---------------|
| `minimal.json` | Static | Backboard, Artboard, Shape, Ellipse, Fill | None | f0 |
| `shapes.json` | Static | Ellipse, Rectangle, Fill, Stroke, Gradients | None | f0 |
| `path.json` | Static | Path with path_flags, Stroke | None | f0 |
| `animation.json` | Animated | Shape with X/Y position keyframes | 1 (120 frames) | f0, f30, f60 |
| `cubic_easing.json` | Animated | Shape with CubicEaseInterpolator width keyframes | 1 (120 frames) | f0, f15, f30, f45, f60 |
| `trim_path.json` | Static | Stroke with TrimPath (75% sequential trim) | None | f0 |
| `state_machine.json` | Interactive | States, transitions, bool/trigger inputs | 1 SM | f0 |
| `multi_artboard.json` | Multi/Animated | 2 artboards, opacity fade + X slide animations | 2 | f0, f30 |
| `nested_artboard.json` | Multi/Static | Main embeds Component via NestedArtboard | None | f0 |
| `artboard_preset.json` | Static | Mobile preset (390×844), empty artboard | None | f0 |
| `gradients.json` | Static | LinearGradient and RadialGradient with gradient stops | None | f0 |
| `color_animation.json` | Animated | Solid color paint with KeyFrameColor progression | 1 (120 frames) | f0, f30, f60 |
| `loop_animation.json` | Animated | Loop-type linear animation with speed override | 1 (60 frames) | f0, f30 |
| `stroke_styles.json` | Static | Stroke cap/join/thickness combinations with fill overlay | None | f0 |
| `empty_artboard.json` | Edge/Static | Artboard without drawable children | None | f0 |

### Growth Plan

Add fixtures in these categories as new features land:

| Category | Target fixtures | Triggers |
|----------|----------------|----------|
| **Static drawing** | 5+ | New shape types, new paint types, nested transforms |
| **Animation** | 5+ | New keyframe types (bool, path vertex), new interpolators, work areas |
| **State machine** | 3+ | Number/trigger inputs, multi-layer SMs, transition conditions |
| **Multi-artboard** | 2+ | When #29 lands — different sizes, shared components |
| **Rigging** | 3+ | When #12 lands — bones, skins, constraints |
| **Text/assets** | 2+ | When #13 lands — text runs, image asset refs |
| **Edge cases** | 1 per bug | Every bug fix gets a regression fixture |

**Target**: 15 fixtures by Phase 6, 25+ by Phase 7.

### Fixture Design Principles

1. **High contrast**: bright colors on dark backgrounds for unambiguous screenshots
2. **Centered composition**: main subject in artboard center for consistent framing
3. **Minimal complexity**: each fixture tests one feature, not a combination
4. **Deterministic**: same input always produces byte-identical `.riv` output
5. **Named objects**: every object has a meaningful name for inspect output readability

## Adding a New Object Type (Testing Checklist)

When adding a new Rive object type to the codebase:

- [ ] Verify type_key and property_keys against C++ `*_base.hpp` headers
- [ ] Add unit tests in the object's source file (type_key, properties, defaults)
- [ ] Add builder support in `scene.rs` with validation
- [ ] Add validation tests (valid input, invalid input, edge cases)
- [ ] Create a fixture JSON in `tests/fixtures/`
- [ ] Add e2e test in `tests/e2e.rs`
- [ ] Add fixture to `FIXTURES` in `tests/playwright/shared.js` (and to `RUNTIME_ONLY_FIXTURES` if it should not get PNG baselines)
- [ ] Run Playwright regression — fixture loads without runtime errors
- [ ] Capture golden-frame baseline: `npx -y -p playwright node tests/playwright/visual-regression.js --update`
- [ ] Commit baseline PNGs
- [ ] Regenerate the scene schema: `UPDATE_SCENE_SCHEMA=1 cargo test scene_schema_file_is_in_sync` (it is derived from `src/builder/spec.rs`, never hand-edited)
- [ ] Update `AGENTS.md` with the new type's location and conventions

## Adding an AuthoringSpec Field (Testing Checklist)

The object-type checklist above covers `SceneSpec`. A field on the high-level
`AuthoringSpec` frontend takes a different path, because it is validated by
lowering rather than by encoding:

- [ ] Write the contract test first in `tests/authoring_{feature}_contract.rs` — assert the lowered SceneSpec, the `authored_path`/`scene_path` pair in the source map, and the diagnostic code and authored path for every rejection. `tests/authoring_stacking_contract.rs` is the shortest example at four tests
- [ ] Give the field a `#[serde(default)]` in `src/authoring/spec.rs` whose default reproduces the previous output, so `authoring_format_version` stays 0 and the committed showcase artifacts do not drift
- [ ] Regenerate the published schema: `cargo test --test authoring_contract regenerate_published_authoring_schema -- --ignored`. It is derived from `authoring_schema()` and never hand-edited; `published_authoring_schema_matches_generated_contract` fails until `docs/authoring.schema.v0.json` matches
- [ ] Add official-runtime evidence when the field changes rendered output — a `tests/authoring_{feature}_runtime.rs` that lowers a fixture from `examples/authoring/`, renders through `render()`, and measures pixels rather than object counts. `tests/authoring_stacking_runtime.rs` renders frame 0 at 128×128 and asserts which rectangle owns the centre pixel
- [ ] Add the fixture to `examples/authoring/` with a row and a paragraph in `examples/authoring/README.md`, naming the tests that gate it
- [ ] Register a new showcase in `FIXTURES` in `tests/playwright/shared.js`, and add it to `RUNTIME_ONLY_FIXTURES` when its visible state comes from state-machine inputs rather than a fixed timeline, since `visual-regression.js` cannot pixel-compare a frame that depends on an input
- [ ] Add the showcase name to `SHOWCASES` in `tests/showcase_artifact.rs` when a compiled `.riv` is committed beside the document, and regenerate that artifact with `authoring compile` whenever the document or the compiler changes

## Fuzz and Property Testing

Implemented in PR #39:

- **cargo-fuzz**: `fuzz_parse_riv` feeds random bytes into the parser. Requires nightly and uses the seed corpus in `fuzz/corpus/`.
- **Roundtrip property tests**: proptest-based encode→decode roundtrips for varuint, float, string, color, and bool in `encoder/binary_writer.rs`.
- **LEB128 boundary tests**: edge coverage at 2^7, 2^14, 2^21, 2^28 boundaries plus `u64::MAX`.

## Animation Frame Diffing (Multi-Frame)

Implemented coverage captures multi-frame baselines for:

- `animation.json`: f0, f30, f60
- `cubic_easing.json`: f0, f15, f30, f45, f60
- `multi_artboard.json`: f0, f30
- `color_animation.json`: f0, f30, f60
- `loop_animation.json`: f0, f30

Static fixtures continue to capture f0.

## Commands

```bash
# Run all tests
cargo test

# Lint
cargo clippy -- -D warnings

# Format check
cargo fmt --check

# Runtime regression (loads .riv in Rive WASM)
npx -y -p playwright node tests/playwright/regression.js

# Visual regression (pixel comparison against baselines)
npx -y -p playwright node tests/playwright/visual-regression.js

# Update golden-frame baselines (after visual review)
npx -y -p playwright node tests/playwright/visual-regression.js --update
```
