# rive-cli Cookbook

This cookbook shows deterministic, end-to-end flows from prompt intent to `SceneSpec` JSON to a generated `.riv` artifact.

## Quick Run Pattern

```bash
# Generate a .riv file from a SceneSpec JSON file
cargo run -- generate tests/fixtures/minimal.json -o out.riv

# Validate structure and encoding
cargo run -- validate out.riv

# Inspect decoded object tree
cargo run -- inspect out.riv
```

## 1) Static Icon Card

- Prompt intent: "Create a simple icon card with a circle and clean fill."
- SceneSpec: `tests/fixtures/minimal.json`

```bash
cargo run -- generate tests/fixtures/minimal.json -o minimal.riv
cargo run -- validate minimal.riv
cargo run -- inspect minimal.riv --json
```

## 2) Bouncing Loader Animation

- Prompt intent: "Animate a loader ball that bounces over time at 60fps."
- SceneSpec: `tests/fixtures/animation.json`

```bash
cargo run -- generate tests/fixtures/animation.json -o animation.riv
cargo run -- validate animation.riv
cargo run -- inspect animation.riv --type-key 31
```

## 3) Interactive Toggle State Machine

- Prompt intent: "Build a button-like object with idle/active states controlled by a bool input."
- SceneSpec: `tests/fixtures/state_machine.json`

```bash
cargo run -- generate tests/fixtures/state_machine.json -o state-machine.riv
cargo run -- validate state-machine.riv
cargo run -- inspect state-machine.riv --type-key 53
```

## 4) Multi-Artboard Component Reuse

- Prompt intent: "Create a reusable component artboard and embed it in a main artboard."
- SceneSpec: `tests/fixtures/multi_artboard.json`

```bash
cargo run -- generate tests/fixtures/multi_artboard.json -o multi-artboard.riv
cargo run -- validate multi-artboard.riv
cargo run -- inspect multi-artboard.riv --type-key 1
```

## 5) Character Rigging with Constraints

- Prompt intent: "Model a rigged arm with IK-style behavior and explicit constraints."
- SceneSpec: `tests/fixtures/constraints.json`

```bash
cargo run -- generate tests/fixtures/constraints.json -o constraints.riv
cargo run -- validate constraints.riv
cargo run -- inspect constraints.riv --type-key 81
```

## 6) Text + Layout + Data Binding + Assets

- Prompt intent: "Render text in a layout system, bind values through a view model, and attach external assets."
- SceneSpec files:
  - `tests/fixtures/text.json`
  - `tests/fixtures/layout.json`
  - `tests/fixtures/data_binding.json`
  - `tests/fixtures/assets.json`

```bash
cargo run -- generate tests/fixtures/text.json -o text.riv
cargo run -- generate tests/fixtures/layout.json -o layout.riv
cargo run -- generate tests/fixtures/data_binding.json -o data-binding.riv
cargo run -- generate tests/fixtures/assets.json -o assets.riv

cargo run -- validate text.riv
cargo run -- validate layout.riv
cargo run -- validate data-binding.riv
cargo run -- validate assets.riv
```

## Authoring Checklist

- Keep `scene_format_version` at `1`.
- Use `cargo run -- validate` after every generation.
- Use `cargo run -- inspect --json` when debugging object keys and properties.
- Start from an existing fixture and evolve incrementally.
