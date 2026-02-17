# rive-cli

## What This Is

A Rust CLI tool that generates Rive `.riv` animation files programmatically. This is the **write side** of the Rive format — the read side is already open source.

## Key References

You MUST consult these before implementing any object type or format detail:

- **Format spec**: https://rive.app/docs/runtimes/advanced-topic/format
- **C++ binary writer**: https://github.com/rive-app/rive-runtime/blob/main/include/rive/core/binary_writer.hpp
- **C++ binary reader** (for understanding what the writer must produce): https://github.com/rive-app/rive-runtime/blob/main/src/core/binary_reader.cpp
- **Generated type definitions** (object types, property IDs, backing types): https://github.com/rive-app/rive-runtime/tree/main/include/rive/generated
- **Core registry** (type key → object mapping): https://github.com/rive-app/rive-runtime/blob/main/include/rive/generated/core_registry.hpp

## Architecture

```
src/
├── main.rs              # CLI entry point
├── encoder/             # Low-level .riv binary format
│   ├── mod.rs
│   ├── binary_writer.rs # LEB128, varuint, float, string, color encoding
│   ├── header.rs        # RIVE fingerprint, version, file ID
│   └── toc.rs           # Table of contents (property type catalog)
├── objects/             # Rive object type definitions
│   ├── mod.rs
│   ├── core.rs          # Base traits (RiveObject, properties)
│   ├── artboard.rs
│   ├── shapes.rs        # Shape, Path, Ellipse, Rectangle, Fill, Stroke, etc.
│   ├── animation.rs     # LinearAnimation, KeyFrame types
│   ├── state_machine.rs # StateMachine, Layer, State, Transition, Condition
│   ├── bones.rs         # Bone, RootBone, Skin, Weight
│   ├── constraints.rs   # IK, Distance, Transform constraints
│   ├── text.rs          # Text, TextRun, TextStyle
│   ├── assets.rs        # Image, Font, Audio assets
│   ├── layout.rs        # Layout components
│   └── data_bind.rs     # ViewModel, DataBind types
├── builder/             # Ergonomic scene construction API
│   ├── mod.rs
│   └── scene.rs         # Scene builder with parent-child management
├── validator/           # Read-back validation
│   └── mod.rs
└── cli/                 # CLI argument parsing and commands
    └── mod.rs
```

## Binary Format Essentials

- **Byte order**: Little-endian
- **Variable integers**: LEB128 encoding
- **Header**: `RIVE` (4 bytes) + major version (varuint) + minor version (varuint) + file ID (varuint) + ToC
- **Table of Contents**: List of property keys (varuint, 0-terminated) + bit array (2 bits per property → backing type)
- **Backing types**: uint/bool (0), string (1), float (2), color (3)
- **Objects**: type key (varuint) + [property key (varuint) + value]* + 0 terminator
- **Hierarchy**: Objects are ordered sequentially. Parent-child via index references. A Shape always follows its Artboard.

## Development Rules

- **Every object type must match the Rive runtime's generated definitions exactly** — type keys, property IDs, and backing types. Cross-reference with `core_registry.hpp` and the `*_base.hpp` files.
- **Test every layer by loading the output** — generate a .riv, attempt to read it back with the validator, and ideally load it in a Rive runtime.
- **No guessing property IDs** — always look them up from the generated source. Wrong IDs produce files that load but render incorrectly or crash.
- **Preserve format version compatibility** — target major version 7 (current).

## Build & Test

```bash
cargo build
cargo test
cargo run -- generate test.json -o output.riv
cargo run -- validate output.riv
cargo run -- inspect output.riv
```

## Validation Strategy

1. **Internal**: Read back the generated .riv with our own reader and verify object counts/types
2. **External**: Load in Rive's open-source runtimes (rive-rs, or rive-ios in the companion project)
3. **Visual**: Open in the free Rive editor to visually confirm rendering

## Incremental Build Order

Each step must produce valid, loadable `.riv` output before proceeding:

1. Binary encoder + empty artboard
2. Shapes (ellipse, rectangle, path with vertices, fills, strokes)
3. Linear animations (keyframes on transform properties)
4. State machines (states, transitions, conditions, listeners)
5. Bones and skinning
6. Text rendering
7. Assets (images, fonts, audio)
8. Data binding and view models
9. Layout system
10. Nested artboards and advanced features
