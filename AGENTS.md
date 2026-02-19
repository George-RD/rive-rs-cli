# PROJECT KNOWLEDGE BASE

**Generated:** 2026-02-19
**Commit:** 1dd84ba
**Branch:** feat/wave-6-e2e-tests

## OVERVIEW

Rust CLI that generates Rive `.riv` binary animation files from JSON input. Implements the **write side** of the Rive format (the read side is open source). Three commands: `generate`, `validate`, `inspect`.

## STRUCTURE

```
src/
├── main.rs              # CLI dispatch (generate/validate/inspect)
├── cli/mod.rs           # clap argument parsing (28 LOC, thin)
├── encoder/             # .riv binary format writer
│   ├── mod.rs           # encode_riv(), encode_object() orchestration
│   ├── binary_writer.rs # LEB128 varuint, float, string, color
│   ├── header.rs        # RIVE magic + version + file_id
│   └── toc.rs           # Table of contents (2-bit backing types)
├── objects/             # Rive object type definitions (see objects/AGENTS.md)
│   ├── core.rs          # RiveObject trait, type_keys, property_keys
│   ├── artboard.rs      # Artboard, Backboard
│   ├── shapes.rs        # Shape, Ellipse, Rectangle, Fill, Stroke, Gradients
│   ├── animation.rs     # LinearAnimation, KeyFrame types
│   └── state_machine.rs # StateMachine, Layer, States, Transitions, Conditions
├── builder/
│   └── scene.rs         # JSON → Result<Vec<Box<dyn RiveObject>>, String> via SceneSpec
└── validator/
    └── mod.rs           # BinaryReader, parse_riv, validate_riv, inspect_riv
tests/
├── e2e.rs               # Integration tests (cargo run subprocess)
├── fixtures/            # JSON fixtures (minimal, shapes, animation, state_machine, path)
└── playwright/          # Runtime load harness + regression script
```

## CURRENT WORKTREE SNAPSHOT

- Branch: `feat/wave-6-e2e-tests`
- HEAD: `1dd84ba`
- Uncommitted implementation updates are active in:
  - `src/builder/scene.rs` (scene graph index mapping and state-machine wiring)
  - `src/encoder/mod.rs` + `src/encoder/toc.rs` (object emission + ToC backing bit packing)
  - `src/objects/animation.rs` + `src/objects/artboard.rs` + `src/objects/state_machine.rs` (property slimming/order/default handling)
  - `src/validator/mod.rs` (reader/parser/inspect alignment with writer behavior)

## WHERE TO LOOK

| Task | Location | Notes |
|------|----------|-------|
| Add new object type | `objects/core.rs` + `objects/{category}.rs` | Add type_key const, property_key consts, update `property_backing_type()`, implement `RiveObject` trait |
| Add JSON support for new type | `builder/scene.rs` | Extend `ObjectSpec` enum + `append_object()` match arm |
| Fix binary encoding | `encoder/binary_writer.rs` | LEB128, float, string, color primitives |
| Fix ToC encoding | `encoder/toc.rs` | 2-bit backing types, 16 properties per uint32 |
| Fix header | `encoder/header.rs` | RIVE magic, version 7.0, file_id |
| Debug .riv loading | `validator/mod.rs` | `inspect_riv()` dumps full object tree |
| Add CLI flag | `cli/mod.rs` | clap derive macros |
| Add test fixture | `tests/fixtures/` + `tests/e2e.rs` | JSON file + e2e test case |
| Look up type/property keys | `objects/core.rs` | `type_keys::*`, `property_keys::*` |
| Cross-ref with C++ runtime | `/tmp/rive-runtime/` | Cloned repo for generated headers |

## DATA FLOW

```
JSON (tests/fixtures/*.json)
  → serde → SceneSpec (builder/scene.rs)
  → build_scene() → Vec<Box<dyn RiveObject>>
  → encode_riv() → Vec<u8>
  → .riv file
  → validate_riv() / inspect_riv() (validator/mod.rs)
```

## DEPENDENCY GRAPH

```
objects/core.rs (foundation: RiveObject trait, type/property keys)
  ↑
objects/{shapes,animation,state_machine,artboard}.rs (implement RiveObject)
  ↑                    ↑
encoder/mod.rs      builder/scene.rs (consumes all object types)
  ↑                    ↑
main.rs ←──────── validator/mod.rs (independent reader)
  ↑
cli/mod.rs
```

## CONVENTIONS

- **No comments or docstrings** in code — code must be self-documenting
- **No magic numbers** — use `type_keys::*` and `property_keys::*` constants
- **No `unwrap()` in library code** — use `Result` + `?` operator
- **CLI errors**: `eprintln!()` + `std::process::exit(1)`
- **Tests**: unit tests inline (`#[cfg(test)] mod tests`), e2e tests in `tests/e2e.rs`
- **Commits**: `feat:` / `fix:` prefix, only when user requests
- **Edition 2024** — requires Rust 1.84+

## ANTI-PATTERNS (THIS PROJECT)

- **NEVER guess property IDs or type keys** — always cross-reference with C++ `core_registry.hpp` and `*_base.hpp` files
- **NEVER write CoreBoolType as varuint** — booleans encode as single raw byte, not LEB128
- **NEVER include baseline properties (name=4, parentId=5, width=7, height=8) in ToC** — causes WASM runtime import failures
- **NEVER write Artboard parentId** — Artboard is root, no parent reference
- **NEVER write default-valued properties for LinearAnimation** — only name/fps/duration are always written; speed/loop/workStart/workEnd only when non-default
- **Artboard property order**: width(7) → height(8) → name(4) — no parentId

## COMMANDS

```bash
cargo build
cargo test                                    # 131 tests (121 unit + 10 e2e)
cargo run -- generate input.json -o out.riv   # JSON → .riv
cargo run -- validate out.riv                 # structural check
cargo run -- inspect out.riv                  # dump object tree
cargo run -- inspect out.riv --json           # dump as JSON
cargo clippy -- -D warnings                   # lint
cargo fmt --check                             # format check
```

## TEST INFRASTRUCTURE

- `cargo test` currently runs **131 tests total**: **121 unit tests** + **10 e2e tests**
- E2E coverage lives in `tests/e2e.rs` and executes CLI subprocesses for `generate`, `validate`, and `inspect`
- Fixtures for e2e live in `tests/fixtures/` (`minimal.json`, `shapes.json`, `animation.json`, `state_machine.json`, `path.json`)
- Playwright runtime regression checks live in `tests/playwright/` and run via `npx -y -p playwright node tests/playwright/regression.js`

## BINARY FORMAT QUICK REF

- **Header**: `RIVE` (4B) + major(varuint=7) + minor(varuint=0) + fileId(varuint) + ToC
- **ToC**: property keys (varuint, 0-terminated) + backing bits (2-bit per key, 16 per uint32 LE)
- **Backing types**: uint/bool=0, string=1, float=2, color=3
- **Object**: typeKey(varuint) + [propKey(varuint) + value]* + 0 terminator
- **Hierarchy**: sequential order, parent-child via parentId (artboard-local index, 0-based excluding Backboard)

## NOTES

- `#![allow(dead_code, unused_imports)]` in main.rs is intentional (incremental build)
- `.gitignore` excludes `*.riv` — generated files not committed
- `.claudeignore` excludes `README.md` from context
- C++ runtime source cloned to `/tmp/rive-runtime/` for reference
- Visual test harness at `/tmp/rive-visual-test/` with Playwright automation
- Scene input is versioned with `scene_format_version` (current value `1`) and JSON schema lives at `docs/scene.schema.v1.json`
- **Active bug**: StateMachineLayer (type 57) causes WASM runtime "may be corrupt" error — under investigation
- No CI/CD pipeline yet — no `.github/workflows/`
- `thiserror` v2 is a dependency but not yet used (validator uses `String` errors)
