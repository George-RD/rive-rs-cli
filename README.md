# rive-cli

A feature-complete CLI for programmatic creation and manipulation of Rive (`.riv`) animation files, written in Rust.

## Vision

Generate production-ready `.riv` files from structured input (JSON, CLI commands, or piped instructions) without requiring the Rive editor. Output files are loadable by any Rive runtime (iOS, Android, Web, Flutter, etc.).

This tool is designed to be composable — usable standalone, scriptable in CI pipelines, and connectable to AI agents via MCP skills/plugins.

## Why This Exists

The Rive `.riv` binary format is [well-documented](https://rive.app/docs/runtimes/advanced-topic/format) and the runtimes are [MIT-licensed open source](https://github.com/rive-app/rive-runtime). The editor is proprietary. This project implements the **write side** of the format — creating `.riv` files programmatically.

## Architecture

```
Input (JSON/CLI) → Builder API → Binary Encoder → .riv file
                                                      ↓
                                              Rive Runtime (validate)
```

### Core Layers

1. **Binary Encoder** (`src/encoder/`) — Low-level .riv binary format writer. Handles the RIVE header, table of contents, LEB128 varuint encoding, object serialization. Direct port of concepts from `rive-runtime/include/rive/core/binary_writer.hpp`.

2. **Object Model** (`src/objects/`) — Rust types for every .riv object (Artboard, Shape, Path, Fill, Animation, StateMachine, Bone, etc.). Auto-generated from Rive's open-source core definitions where possible.

3. **Builder API** (`src/builder/`) — Ergonomic API for constructing animation scenes. Handles parent-child relationships, property defaults, ID assignment, and serialization order.

4. **CLI Interface** (`src/cli/`) — Command-line interface accepting JSON scene descriptions or subcommands for building files incrementally.

5. **Validator** (`src/validator/`) — Reads back generated `.riv` files using the format spec to verify structural correctness before runtime loading.

## .riv Format Reference

- **Spec**: https://rive.app/docs/runtimes/advanced-topic/format
- **Binary format**: Little-endian, LEB128 varuints, 4 backing types (uint, string, float, color)
- **Header**: `RIVE` fingerprint (4 bytes) + major version (varuint, currently 7) + minor version + file ID + ToC
- **Objects**: Sequential list, each = type key (varuint) + property key-value pairs + zero terminator
- **Hierarchy**: Implicit via read order; parent references via unsigned integer indices

## Object Type Coverage

Target: all object types from the Rive runtime's generated definitions (~150-200 types across these categories):

| Category | Examples | Source |
|----------|----------|--------|
| **Drawing** | Artboard, Shape, Ellipse, Rectangle, Path, Fill, Stroke, RadialGradient | `generated/shapes/` |
| **Animation** | LinearAnimation, KeyFrameDouble, KeyFrameColor, CubicInterpolator | `generated/animation/` |
| **State Machines** | StateMachine, Layer, EntryState, AnimationState, Transition, Conditions, Listeners | `generated/animation/` |
| **Hierarchy** | Node, TransformComponent, Drawable, ContainerComponent | root generated |
| **Bones** | Bone, RootBone, Skin, Weight, Tendon | `generated/bones/` |
| **Constraints** | IKConstraint, DistanceConstraint, TransformConstraint | `generated/constraints/` |
| **Text** | Text, TextRun, TextStyle, TextValueRun | `generated/text/` |
| **Assets** | ImageAsset, FontAsset, AudioAsset, FileAssetContents | `generated/assets/` |
| **Layout** | LayoutComponent, various layout types | `generated/layout/` |
| **Data Binding** | ViewModel, ViewModelProperty types, DataBind | `generated/data_bind/`, `generated/viewmodel/` |

## Development Approach

Build incrementally, driven by real output at each step:

1. **Binary foundation** — Encoder writes valid .riv headers and empty artboards. Validate by loading in Rive runtime.
2. **Static drawing** — Shapes, paths, fills, strokes, colors. Produce visible artwork.
3. **Animation** — Linear animations with keyframes. Things move.
4. **State machines** — Interactive states with transitions and listeners. Things respond.
5. **Rigging** — Bones, skinning, constraints. Characters articulate.
6. **Text & assets** — Embedded text, images, fonts, audio references.
7. **Advanced** — Data binding, view models, nested artboards, layout system.

Each step produces testable `.riv` output. No layer is "done" until its output loads correctly in a Rive runtime.

## Usage (Planned)

```bash
# Generate from JSON scene description
rive-cli generate scene.json -o output.riv

# Validate a .riv file
rive-cli validate output.riv

# Inspect a .riv file (dump object tree)
rive-cli inspect existing.riv

# Pipe from stdin
echo '{"artboard": {"width": 500, "height": 500, "children": [...]}}' | rive-cli generate -o output.riv
```

## Reference Material

- [.riv format specification](https://rive.app/docs/runtimes/advanced-topic/format)
- [rive-runtime C++ (MIT)](https://github.com/rive-app/rive-runtime) — binary reader/writer, generated type definitions
- [rive-rs Rust runtime](https://github.com/rive-app/rive-rs) — Rust bindings (read-only)
- [Generated core definitions](https://github.com/rive-app/rive-runtime/tree/main/include/rive/generated) — all object types and property IDs

## License

MIT
