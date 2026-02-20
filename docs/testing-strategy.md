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

- `test_generate_{fixture}` — generates .riv, validates it (required for every fixture)
- `test_inspect_{fixture}` — optional, for select fixtures to verify inspect output structure

**Convention**: every new fixture added to `tests/fixtures/` must have a corresponding e2e test in `tests/e2e.rs`.

### 3. Playwright Runtime Regression (`tests/playwright/regression.js`)

Loads generated `.riv` files in the official `@rive-app/canvas` WASM runtime via a browser harness. Catches issues that pass structural validation but fail at runtime (wrong object hierarchy, missing properties, encoding quirks).

**How it works**:

1. Generates `.riv` from each fixture JSON via `cargo run -- generate`
2. Starts a local HTTP server serving `harness.html` + the `.riv` files
3. Launches headless Chromium via Playwright
4. Loads each `.riv` in the Rive canvas runtime
5. Fails if any runtime error, load failure, or console error occurs

**Runtime version**: `@rive-app/canvas@2.35.0` (pinned in `harness.html` CDN link).

**Convention**: bump the runtime version deliberately. When bumping, re-run all fixtures and update baselines if rendering changes.

### 4. Golden-Frame Visual Regression (`tests/playwright/visual-regression.js`)

Pixel-level comparison of rendered frames against committed baseline PNGs.

**How it works**:

1. Loads each fixture in a controlled Rive canvas (manual frame advance, no autoplay)
2. Captures screenshots at specific frame points (see frame plan below)
3. Compares against baselines in `tests/playwright/baselines/` using pixel diff
4. Fails if any fixture exceeds the diff threshold (default 0.1%)

**Frame capture plan** (`shotPlanForFixture()`):

| Fixture category | Frames captured | Why |
|------------------|-----------------|-----|
| Static (minimal, shapes, path, trim_path, artboard_preset) | f0 only | No animation — single frame is sufficient |
| Linear animation | f0, f30, f60 | Start, midpoint, end — catches timing and interpolation errors |
| Cubic easing | f0, f15, f30, f45, f60 | Dense 5-frame sampling catches easing curve shape and control point errors |
| Multi-artboard | f0, f30 | Captures opacity fade-in animation between frames |
| Nested artboard | f0 only | Static embedding — expand when nested animations are supported |
| State machine | f0 only | Animations have no keyframes; expand when simulated input triggers are added |

**Updating baselines**:

```bash
UPDATE_BASELINES=1 npx -y -p playwright node tests/playwright/visual-regression.js
```

This overwrites `tests/playwright/baselines/*.png` with current renders. Commit the updated baselines after visual review.

**Resolution**: 512×512 logical viewport with `deviceScaleFactor: 2`, producing 1024×1024 pixel screenshots. This ensures crisp rendering on Retina/HiDPI displays and catches sub-pixel anti-aliasing issues.

**Diff threshold**: 1.0% of pixels (configurable via `VISUAL_DIFF_THRESHOLD` env var). Static frames (f0 of non-animated fixtures) consistently diff at 0.0000%. Animated frames show 0.2-0.5% jitter due to `requestAnimationFrame` timing non-determinism — the Rive runtime advances based on real-time deltas, so captured animation positions shift by 1-2 pixels between runs. The 1.0% threshold accommodates this while still catching real regressions (a broken animation would show 30%+ diff).

## Fixture Corpus

### Current Fixtures (Post-PR #36)

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
- [ ] Add fixture to `FIXTURES` array in `regression.js` and `visual-regression.js`
- [ ] Run Playwright regression — fixture loads without runtime errors
- [ ] Capture golden-frame baseline: `UPDATE_BASELINES=1 npx -y -p playwright node tests/playwright/visual-regression.js`
- [ ] Commit baseline PNGs
- [ ] Update `docs/scene.schema.v1.json` if new JSON fields added
- [ ] Update `AGENTS.md` with the new type's location and conventions

## Future: Fuzz and Property Testing

Planned but not yet implemented (tracked in #17):

- **cargo-fuzz**: random byte sequences → `validator/mod.rs` parser. Catches panics, OOMs, infinite loops in the binary reader.
- **Roundtrip property tests**: generate random valid `SceneSpec` → `build_scene()` → `encode_riv()` → `parse_riv()` → compare object tree. Catches encode/decode asymmetry.
- **LEB128 boundary tests**: property-based tests for varuint encoding at u32/u64 boundaries.

## Future: Animation Frame Diffing (Multi-Frame)

Current state: `animation.json` captures 3 frames (f0, f30, f60). Other animated fixtures capture only f0.

**Expansion plan**:

1. Update `shotPlanForFixture()` to return multi-frame plans for all animated fixtures
2. Standard plan: `f0` (start), `f_mid` (duration/2), `f_end` (last frame)
3. Cubic easing plan: 5 frames — f0, f15, f30, f45, f60 — to catch easing curve shape
4. State machine plan: f0 (initial state) + capture after simulated input trigger
5. Store all frame baselines: `{fixture}-f{N}.png`

This catches:
- Interpolation errors (wrong easing curve shape between keyframes)
- Timing errors (animation plays too fast/slow)
- State transition rendering (wrong state displayed after input)

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
UPDATE_BASELINES=1 npx -y -p playwright node tests/playwright/visual-regression.js
```
