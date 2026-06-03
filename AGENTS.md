# Repository Guidelines

## Project Overview

`rive-cli` is a Rust CLI and library that generates Rive `.riv` binary animation files from JSON scene specifications. It implements the **write side** of the Rive binary format; the read side is open-source. The tool supports programmatic creation of vector animations, state machines, and interactive graphics without the Rive editor.

**Primary commands**: `generate`, `validate`, `inspect`, `decompile`, `ai generate`, `ai lab`.

## Architecture & Data Flow

The codebase is organized around a linear pipeline from JSON input to binary output, with a validator that reads it back:

```
JSON SceneSpec (tests/fixtures/*.json)
  → serde → SceneSpec (src/builder/spec.rs)
  → build_scene() → Vec<Box<dyn RiveObject>>
  → encode_riv() → Vec<u8>
  → .riv file
  → validate_riv() / inspect_riv() (src/validator/mod.rs)
```

**Key layers**:

1. **Object Model** (`src/objects/`): Flat struct hierarchy implementing the `RiveObject` trait. Each struct exposes a `u16` `type_key` and a `properties()` list. Type keys and property keys are constants matching the C++ rive-runtime exactly.
2. **Builder** (`src/builder/`): JSON deserialization → validation → object tree construction. `scene.rs` orchestrates; `objects.rs` is a ~4200-line dispatch for all ~200 object types.
3. **Encoder** (`src/encoder/`): Serializes the flat object list into the `.riv` binary format. Writes header (RIVE magic + version), Table of Contents (2-bit backing types), and property stream (LEB128 varuint, f32 LE, raw bool bytes).
4. **Validator** (`src/validator/`): Reads `.riv` bytes back into a `ParsedRiv` tree. Used for structural checks, inspection, and decompilation.
5. **AI Subsystem** (`src/ai/`): LLM provider abstraction, auto-repair engine, evaluation suites, and built-in JSON templates.
6. **MCP Server** (`src/mcp/`): Optional feature-gated Model Context Protocol server exposing generate/validate/inspect/decompile as stdio JSON-RPC tools.

**Hierarchy model**: Objects are stored in a flat `Vec<Box<dyn RiveObject>>`. Parent-child relationships are tracked via `parent_id` (artboard-local index, 0-based, excluding Backboard). Each artboard has independent object/animation/interpolator index scopes.

## Key Directories

| Directory | Purpose |
|-----------|---------|
| `src/` | Main source code |
| `src/objects/` | Rive object type definitions (~18 files, ~800KB). Core trait, type keys, property keys, and all domain implementations |
| `src/builder/` | JSON → `Vec<Box<dyn RiveObject>>` pipeline |
| `src/encoder/` | `.riv` binary format writer |
| `src/validator/` | Binary reader, parser, inspector, decompiler |
| `src/ai/` | AI-assisted generation, repair, and evaluation |
| `src/mcp/` | Optional MCP server (`mcp` feature) |
| `tests/` | E2E CLI tests, JSON fixtures, Playwright runtime regression |
| `tests/fixtures/` | ~55 JSON scene specs + 3 intentionally invalid fixtures |
| `tests/playwright/` | Browser-based runtime and visual regression harness |
| `docs/` | Format spec, JSON schema, install/release docs, testing strategy, cookbook |
| `fuzz/` | `cargo-fuzz` target for adversarial parser testing |
| `specs/` | Domain specs for missing Rive object types (now all implemented) |
| `scripts/` | Multi-model AI race harness (`run_race.sh`) |
| `demo/` | Node.js demo server and interactive HTML viewer |

## Development Commands

```bash
# Build
cargo build
cargo build --release
cargo build --features mcp          # enable MCP server

# Test
cargo test                          # unit + integration tests
cargo test --test e2e               # e2e tests only
cargo test test_name                # single test by name

# Lint / Format
cargo clippy -- -D warnings
cargo fmt --check

# CLI usage
cargo run -- generate input.json -o out.riv
cargo run -- validate out.riv
cargo run -- inspect out.riv
cargo run -- inspect out.riv --json
cargo run -- inspect out.riv --type-name Shape
cargo run -- decompile out.riv
cargo run -- ai generate --prompt "bouncing ball" -o out.riv
cargo run -- ai lab --suite evals/suites/<suite>.json
cargo run -- --list-presets         # artboard size presets

# Playwright regression (requires Node.js)
npx -y -p playwright node tests/playwright/regression.js
npx -y -p playwright node tests/playwright/visual-regression.js
npx -y -p playwright node tests/playwright/visual-regression.js --update

# Fuzzing (requires nightly)
cd fuzz && cargo +nightly fuzz run fuzz_parse_riv
```

## Code Conventions & Common Patterns

### Style
- **Edition 2024** (requires Rust 1.84+)
- **No comments or docstrings** — code must be self-documenting
- **No magic numbers** — use `type_keys::*` and `property_keys::*` constants
- `#![allow(dead_code, unused_imports)]` in `main.rs` is intentional (incremental build)

### Error Handling
- **No `unwrap()` in library code** — use `Result` + `?` operator
- **CLI errors**: `eprintln!()` + `std::process::exit(1)`
- AI subsystem uses `thiserror` derives (`AiError` enum in `src/ai/error.rs`)

### Architecture Patterns
- **Trait-based object model**: Every Rive runtime type implements `RiveObject` (`src/objects/core.rs`). Two required methods: `type_key() -> u16` and `properties() -> Vec<Property>`.
- **Validate-first builder**: `src/builder/validation.rs` checks structural correctness (unique names, valid refs, no cycles) before object construction.
- **Flat vector with local indices**: Objects are emitted as a flat `Vec<Box<dyn RiveObject>>`. `parent_id` is artboard-local (`parent_global - artboard_start`). Never use global indices across artboards.
- **Backing type registry**: `property_backing_type()` in `src/objects/core.rs` maps property keys to `UInt`/`String`/`Float`/`Color`. The encoder uses this for ToC generation and value serialization. Only unknown properties go into the ToC.
- **Feature-gated MCP**: `src/mcp/` compiles only with `--features mcp`, adding `rmcp` + `tokio` + `schemars`.

### Testing Patterns
- Inline unit tests in `#[cfg(test)] mod tests` blocks within source files
- E2E tests in `tests/e2e.rs` invoke the compiled binary via `std::process::Command`
- Property-based roundtrip tests in `src/encoder/binary_writer.rs` using `proptest`

## Important Files

| File | Role |
|------|------|
| `src/main.rs` | Binary entry point; CLI dispatch |
| `src/lib.rs` | Library root; exports `objects`, `builder`, `encoder`, `validator`, `ai`, `mcp` |
| `src/cli/mod.rs` | clap derive-based argument parsing |
| `src/objects/core.rs` | `RiveObject` trait, `PropertyValue`, `BackingType`, type_keys/property_keys constants, backing type lookup |
| `src/objects/mod.rs` | Re-exports all 18 object submodules |
| `src/builder/scene.rs` | `build_scene()` orchestrator; artboard presets |
| `src/builder/objects.rs` | ~4200-line dispatch creating each object type from `ObjectSpec` |
| `src/builder/spec.rs` | `SceneSpec` and all sub-spec types (JSON input schema) |
| `src/encoder/mod.rs` | `encode_riv()`, `encode_object()` |
| `src/encoder/binary_writer.rs` | LEB128 varuint, f32 LE, string, color, bool primitives |
| `src/validator/mod.rs` | `validate_riv()`, `inspect_riv()`, `decompile()` entry points |
| `src/ai/provider.rs` | `AiProvider` trait + factory |
| `src/ai/repair.rs` | `RepairEngine` multi-pass auto-fix for generated scenes |
| `tests/e2e.rs` | ~4020-line integration test suite (100+ tests) |
| `tests/fixtures/` | JSON scene fixtures for all test layers |
| `Cargo.toml` | Root manifest: single crate, edition 2024, optional `mcp` feature |
| `docs/scene.schema.v1.json` | JSON Schema v1 for SceneSpec input |
| `docs/format-spec.md` | Binary encoding reference for the `.riv` format |

## Runtime/Tooling Preferences

- **Language**: Rust (stable, edition 2024)
- **Package manager**: Cargo
- **Node.js**: Required for Playwright regression tests and demo server (Node 20+ recommended)
- **Shell tools**: `jq`, `sqlite3`, `python3` used by `scripts/run_race.sh`
- **CI/CD**: GitHub Actions (`.github/workflows/ci.yml`, `.github/workflows/release.yml`)
- **Fuzzing**: `cargo-fuzz` + nightly Rust
- **No pinned Rust toolchain** — uses `dtolnay/rust-toolchain@stable` in CI
- **No `Makefile`/`justfile`** — pure Cargo workflow
- **Release profile**: `opt-level=3`, `lto=true`, `strip=true`

## Testing & QA

### Layers
1. **Unit tests**: 31 inline `#[cfg(test)]` blocks across `src/`. Cover object type keys, property emission, encoder primitives, validator parsing, and builder deserialization.
2. **Property tests**: `proptest` roundtrips in `src/encoder/binary_writer.rs` (varuint, float, string, color, bool).
3. **E2E CLI tests**: `tests/e2e.rs` (~4020 lines, 100+ tests). Spawns `cargo run --` subprocesses for `generate`, `validate`, `inspect`, `decompile` against all 55+ fixtures. Tests magic bytes, exit codes, filter flags, error paths, version warnings, and roundtrips.
4. **Runtime regression**: Playwright loads each generated `.riv` in `@rive-app/canvas` WASM via headless Chromium. Catches structural issues invisible to the Rust validator.
5. **Visual regression**: Pixel-level diff against committed PNG baselines (`tests/playwright/baselines/`). 1024×1024 captures, configurable thresholds per fixture/frame.
6. **Fuzzing**: `fuzz/fuzz_targets/fuzz_parse_riv.rs` feeds random bytes into the parser.

### Running Tests
```bash
cargo test                          # all Rust tests
cargo test --test e2e               # CLI integration tests only
npx -y -p playwright node tests/playwright/regression.js
cd fuzz && cargo +nightly fuzz run fuzz_parse_riv
```

### CI Pipeline
- `cargo fmt --check`
- `cargo clippy -- -D warnings` (with `RUSTFLAGS="-D warnings"`)
- `cargo test`
- Playwright runtime regression (depends on `check` job)
- Demo console error validation (depends on `check` job)

## Development Workflow

### 1. Feature Development Flow (Adding New Rive Object Types)

Entry: Look up the C++ runtime source in the rive-runtime repository. Extract `type_key` and `property_keys` from the corresponding `*_base.hpp` file.

Steps:
1. Add constants: define the new `type_key` and each `property_key` as `u16` constants in `src/objects/core.rs` or the appropriate module.
2. Add struct: create a Rust struct with fields matching the C++ base class properties.
3. Implement `RiveObject`: implement `type_key() -> u16` and `properties() -> Vec<Property>` for the struct. Map each field to its property key and backing type.
4. Add builder dispatch: wire the new type into `src/builder.rs` so the JSON deserializer can construct it from scene specifications.
5. Add unit tests: add a `#[cfg(test)] mod tests` block in the same file. Cover `type_key()` correctness, `properties()` length and ordering, and round-trip through the encoder.
6. Add fixture: create a minimal JSON fixture in `tests/fixtures/` that exercises the new object type. Register it in `tests/e2e.rs`.
7. Run full validation:
   ```bash
   cargo test
   cargo run -- generate tests/fixtures/your_new_fixture.json -o /tmp/out.riv
   cargo run -- validate /tmp/out.riv
   cargo run -- decompile /tmp/out.riv
   ```

Exit gates:
- `cargo test` passes with no warnings (`RUSTFLAGS="-D warnings"`).
- Demo loads with 0 console errors when served via `demo/serve.js`.
- Fixture renders in the Rive WASM runtime with no runtime rejection.

### 2. Comparison/Parity Flow (Achieving Parity with Official Rive Files)

Entry: Identify an official `.riv` file from the rive-runtime repository or Rive's public asset collection.

Steps:
1. Acquire reference: download or copy the official `.riv` file into `tests/fixtures/reference/`.
2. Decompile: run `cargo run -- decompile` on the reference to produce a human-readable structural dump.
3. Create comparison fixture: write a JSON scene specification that attempts to reproduce the same structure.
4. Generate: produce a `.riv` from the comparison fixture.
5. Compare:
   - Structural: diff the decompile output of the reference against the generated file.
   - Visual: load both in the Rive WASM runtime side-by-side.
   - Pixel: run Playwright visual regression to compute pixel difference.
6. Document gaps: if differences exceed thresholds, document them in `docs/parity.md` with a `gapType` label.

Exit gates:
- Structural match: decompile diff is below the threshold, or every delta is labeled with a `gapType`.
- Visual match: pixel diff is below the threshold, or the mismatch is documented with a `gapType`.
- All gaps use the `gapType` vocabulary: `missing-type`, `property-drift`, `encoding-difference`, `visual-mismatch`, `runtime-rejection`.

### 3. Issue Triage Flow

Entry: An issue is found during validation (console error, performance problem, corrupt file, or visual mismatch).

Steps:
1. Reproduce: create a minimal fixture or script that triggers the issue consistently.
2. Root-cause: trace the issue to a specific object type, encoder path, builder field, or runtime interaction.
3. Fix or document:
   - If fixable in scope: fix it and add a regression test.
   - If out of scope or blocked: document it as a known issue in `docs/parity.md` with a specific ticket reference and acceptance criteria.
4. Re-validate: run the full validation suite (`cargo test`, demo validation, Playwright regression) to confirm resolution.

Rule: Issues are NEVER dismissed as "pre-existing". They are either fixed or tracked with a specific ticket.

Exit gates:
- Issue is resolved and a regression test exists, OR
- Issue is documented in `docs/parity.md` with a ticket link, `gapType`, and clear acceptance criteria for closure.

### 4. Release Flow

Entry: A version bump is committed in `Cargo.toml`.

Steps:
1. Run full quality gates (see Pre-Commit Gates below).
2. Cross-compile: build `--release --locked` for all four targets:
   ```bash
   cargo build --release --locked --target x86_64-unknown-linux-gnu
   cargo build --release --locked --target x86_64-pc-windows-gnu
   cargo build --release --locked --target x86_64-apple-darwin
   cargo build --release --locked --target aarch64-apple-darwin
   ```
3. Tag: create an annotated Git tag matching the `Cargo.toml` version (`git tag -a vX.Y.Z`).
4. GitHub Release: push the tag to trigger the release workflow, which packages binaries with SHA256 checksums.

Exit gates:
- All CI jobs are green.
- All comparison fixtures are at acceptable parity (no unlabeled gaps).
- `CHANGELOG.md` is updated with the version's changes.

### 5. Pre-Commit Gates (Must Pass Before Any PR)

The following commands must pass locally before opening a pull request:

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
```

Additional validation:
- Demo validation: load `demo/index.html` via `demo/serve.js` and confirm 0 console errors.
- Playwright regression: confirm all fixtures load and render correctly in the Rive WASM runtime.

### 6. CI Pipeline Overview

The CI pipeline consists of three jobs:

- **check** job:
  - `cargo fmt --check`
  - `cargo clippy -- -D warnings` (with `RUSTFLAGS="-D warnings"`)
  - `cargo test`

- **playwright** job (depends on `check`):
  - Runs `tests/playwright/regression.js`
  - Validates that all fixtures render correctly in the Rive WASM runtime.

- **demo** job (depends on `check`):
  - Runs `tests/playwright/demo-validation.js` against `demo/serve.js`
  - Validates that the demo web UI loads with 0 console errors.

## Binary Format Quick Reference

- **Header**: `RIVE` (4B) + major(varuint=7) + minor(varuint=0) + fileId(varuint) + ToC
- **ToC**: property keys (varuint, 0-terminated) + backing bits (2-bit per key, 16 per uint32 LE)
- **Backing types**: uint/bool=0, string=1, float=2, color=3
- **Object**: typeKey(varuint) + [propKey(varuint) + value]* + 0 terminator
- **Booleans**: single raw byte, **not** LEB128 varuint

## Anti-Patterns

- **Never guess property IDs or type keys** — cross-reference with C++ `core_registry.hpp` and `*_base.hpp` files
- **Never write CoreBoolType as varuint** — booleans encode as single raw byte
- **Never include known properties in ToC** — only unknown properties; including known ones causes WASM runtime import failures
- **Never write Artboard parentId** — Artboard is root, no parent reference
- **Artboard property order**: width(7) → height(8) → name(4)
- **Never use TrimPath mode_value 0** — valid modes are 1 (sequential) or 2 (synchronized)
- **TrimPath parent must be a ShapePaint (Stroke/Fill), NOT a Shape**
- **Never use global object indices across artboards** — parent_id must be artboard-local

## Key References

- **Format spec**: https://rive.app/docs/runtimes/advanced-topic/format
- **C++ binary writer**: https://github.com/rive-app/rive-runtime/blob/main/include/rive/core/binary_writer.hpp
- **C++ core registry**: https://github.com/rive-app/rive-runtime/blob/main/include/rive/generated/core_registry.hpp
- **Scene JSON Schema**: `docs/scene.schema.v1.json`
- **Format encoding notes**: `docs/format-spec.md`
- **Object type reference**: `src/objects/AGENTS.md`
