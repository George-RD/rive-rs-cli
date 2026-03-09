# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What This Is

A Rust CLI tool that generates Rive `.riv` animation files programmatically. This is the **write side** of the Rive format — the read side is already open source. Output files are loadable by any Rive runtime (iOS, Android, Web, Flutter, etc.).

## Build & Test

```bash
cargo build
cargo test                            # all unit + integration tests
cargo test test_name                  # run a single test by name
cargo test --test e2e                 # e2e tests only (uses cargo run under the hood)
cargo build --features mcp            # build with MCP server support
```

### CLI Commands

```bash
cargo run -- generate tests/fixtures/minimal.json -o output.riv
cargo run -- validate output.riv
cargo run -- inspect output.riv                    # human-readable object tree
cargo run -- inspect output.riv --json             # structured JSON output
cargo run -- inspect output.riv --type-name Shape  # filter by type
cargo run -- decompile output.riv                  # full JSON round-trip
cargo run -- --list-presets                         # artboard size presets
cargo run -- ai generate --prompt "bouncing ball" -o out.riv   # AI-assisted generation
cargo run -- ai lab --suite evals/suite.json                   # eval suite runner
```

### Playwright Regression Tests

```bash
npx -y -p playwright node tests/playwright/regression.js
```

Generates fixture `.riv` files, serves a local harness with a vendored Rive WASM runtime, and fails on runtime load or browser errors. Visual baselines live in `tests/playwright/baselines/`. Shared test infrastructure is in `tests/playwright/shared.js`.

### Fuzzing

```bash
cd fuzz && cargo +nightly fuzz run fuzz_parse_riv
```

## Architecture

### Data Flow

```text
JSON input → SceneSpec (serde) → build_scene() → Vec<Box<dyn RiveObject>> → encode_riv() → .riv bytes
```

### Core Layers

- **`src/encoder/`** — Low-level .riv binary writer. `binary_writer.rs` handles LEB128/varuint/float/string/color encoding. `header.rs` writes the RIVE fingerprint + version. `toc.rs` writes the table of contents (property type catalog). `encode_riv()` in `mod.rs` orchestrates header + ToC + objects.

- **`src/objects/`** — Rust structs implementing the `RiveObject` trait (defined in `core.rs`). Each struct returns its `type_key()` and `properties()` vec. `core.rs` is the authority for all type key constants, property key constants, backing type mappings, and bool-property identification. `generated_registry.rs` provides type-key-to-name and property-key-to-name lookups (auto-generated from rive-runtime defs).

- **`src/builder/scene.rs`** — Deserializes `SceneSpec` JSON, resolves artboard presets, assigns parent IDs, and emits objects in correct serialization order. This is the largest file in the project (~1500+ lines) — it maps every JSON `ObjectSpec` variant to the corresponding Rive object type.

- **`src/validator/`** — `BinaryReader` that parses `.riv` bytes back into structured data. Powers `validate`, `inspect`, and `decompile` commands. Also used in tests to verify round-trip correctness.

- **`src/ai/`** — AI-assisted generation pipeline. `provider.rs` defines the `AiProvider` trait with template and OpenAI implementations. `repair.rs` has `RepairEngine` that auto-retries failed scene builds. `eval.rs` runs evaluation suites with drift detection. `config.rs` resolves model/provider settings.

- **`src/mcp/`** — Optional MCP server (behind `mcp` feature flag) exposing `generate`, `validate`, `inspect` tools over stdio using the `rmcp` crate.

- **`src/cli/mod.rs`** — Clap-derived CLI definitions. Commands: `Generate`, `Validate`, `Inspect`, `Decompile`, `Ai { Generate, Lab }`.

### The RiveObject Trait

Every Rive object implements:

```rust
pub trait RiveObject {
    fn type_key(&self) -> u16;
    fn properties(&self) -> Vec<Property>;
}
```

Properties use `PropertyValue` enum: `UInt(u64)`, `Bool(bool)`, `String(String)`, `Float(f32)`, `Color(u32)`. The encoder uses `is_bool_property()` to decide whether to write a UInt property as a raw bool byte.

### SceneSpec JSON Format

Scene specs require `"scene_format_version": 1` and support `"artboard"` (single) or `"artboards"` (multiple). Each artboard has `children` (ObjectSpec array), optional `animations`, and optional `state_machines`. The JSON schema is at `docs/scene.schema.v1.json`.

### Object Serialization Order

Objects must be written in hierarchy order — a Backboard first, then each Artboard followed by its children depth-first. Parent-child relationships use index-based `parent_id` references. `build_scene()` handles this ordering.

## Key References

You MUST consult these before implementing any object type or format detail:

- **Format spec**: https://rive.app/docs/runtimes/advanced-topic/format
- **C++ binary writer**: https://github.com/rive-app/rive-runtime/blob/main/include/rive/core/binary_writer.hpp
- **C++ binary reader**: https://github.com/rive-app/rive-runtime/blob/main/src/core/binary_reader.cpp
- **Generated type definitions**: https://github.com/rive-app/rive-runtime/tree/main/include/rive/generated
- **Core registry**: https://github.com/rive-app/rive-runtime/blob/main/include/rive/generated/core_registry.hpp

## Binary Format Essentials

- **Byte order**: Little-endian
- **Variable integers**: LEB128 encoding
- **Header**: `RIVE` (4 bytes) + major version (varuint) + minor version (varuint) + file ID (varuint) + ToC
- **Table of Contents**: List of property keys (varuint, 0-terminated) + bit array (2 bits per property → backing type). Only includes properties NOT already known to `property_backing_type()`.
- **Backing types**: uint/bool (0), string (1), float (2), color (3)
- **Objects**: type key (varuint) + [property key (varuint) + value]* + 0 terminator

## Development Rules

- **Every object type must match the Rive runtime's generated definitions exactly** — type keys, property IDs, and backing types. Cross-reference with `core_registry.hpp` and the `*_base.hpp` files.
- **No guessing property IDs** — always look them up from the generated source. Wrong IDs produce files that load but render incorrectly or crash.
- **Build incrementally** — each step must produce valid, loadable `.riv` output before moving on: binary encoder → shapes → animations → state machines → bones → text → assets → data binding → layout → nested artboards.
- **When adding a new object type**: add its type key to `core::type_keys`, property keys to `core::property_keys`, backing types to `property_backing_type()`, bool flags to `is_bool_property()`, update name mappings in `generated_registry.rs`, struct in the appropriate `objects/` file, JSON variant in `scene.rs`, and test fixtures.
- **Test every layer by loading the output** — generate a .riv, read it back with the validator, and ideally load it in a Rive runtime.
- **Preserve format version compatibility** — target major version 7 (current).

## Test Fixtures

JSON fixtures live in `tests/fixtures/`. E2E tests in `tests/e2e.rs` run `cargo run` against these fixtures and validate the output. When adding new object types, add corresponding fixture files and e2e tests.

## Validation Strategy

1. **Internal**: Read back the generated .riv with the validator and verify object counts/types
2. **External**: Load in Rive's open-source runtimes (rive-rs, or rive-ios in the companion project)
3. **Visual**: Playwright regression tests with baseline screenshots, or open in the free Rive editor
