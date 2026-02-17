# rive-cli Implementation Plan

## Goal

Deliver a production-grade Rust CLI that can generate valid Rive (`.riv`) files from:

- Deterministic structured input (JSON/YAML)
- Incremental CLI commands
- AI-generated scene plans (natural language -> validated scene spec -> `.riv`)

## Product Outcomes

1. Generate `.riv` files that load in official Rive runtimes.
2. Support a large enough object surface to build real interactive animations.
3. Provide a stable intermediate scene format that AI agents can produce safely.
4. Make failures diagnosable with inspect/validate tooling.

## Non-Goals (Initial)

- Full visual parity with every new runtime feature on day one.
- Training or hosting an LLM model inside the CLI.
- Replacing the Rive editor UX.

## System Architecture

```text
Prompt/JSON -> Scene Spec -> Builder Graph -> Binary Encoder -> .riv
                               |                |
                               +-> Validator <--+
```

### Workstreams

1. `encoder`: Binary writer, header, ToC, object serialization, versioning.
2. `objects`: Typed object model + generated type/property metadata from Rive runtime definitions.
3. `builder`: High-level scene construction with deterministic IDs and parent ordering.
4. `cli`: `generate`, `validate`, `inspect`, `schema`, `init`, and AI-oriented commands.
5. `validator`: Structural and semantic checks + helpful diagnostics.
6. `ai`: Prompt-to-scene pipeline, guardrails, deterministic transforms, repair loop.
7. `qa`: Golden fixtures, runtime compatibility matrix, fuzz/property tests.

## Delivery Phases

## Phase 0: Foundation

### Scope

- CLI skeleton with `clap`
- Error model (`thiserror`, rich context)
- Logging/tracing
- Project layout matching architecture
- CI (fmt, clippy, tests)

### Exit Criteria

- `cargo test`, `cargo clippy -- -D warnings`, and `cargo fmt --check` pass in CI.

## Phase 1: Binary Core (Minimal Valid `.riv`)

### Scope

- Primitive writers (LEB128 varuint, float32 LE, string, color)
- Header encoding (`RIVE`, major/minor, file id)
- ToC encoding and backing type map
- Object serialization contract + terminator handling
- Write minimal file with one artboard

### Exit Criteria

- Generated sample passes internal validator and opens in at least one official runtime.

## Phase 2: Typed Object Model + Codegen

### Scope

- Parse/ingest runtime-generated definitions (`type key`, `property id`, backing type)
- Generate Rust metadata and type stubs
- Core traits for serializable objects
- Registry for object type lookup and reflection

### Exit Criteria

- No manually guessed property IDs for supported types.
- Generation process reproducible from pinned upstream revision.

## Phase 3: Scene DSL + Builder

### Scope

- Stable `SceneSpec` schema (serde)
- Deterministic object ID assignment and topological ordering
- Parent-child and cross-reference validation
- CLI `generate` from file/stdin + `schema` export

### Exit Criteria

- Same input spec produces byte-identical output (except optional file id variance if configured).

## Phase 4: Drawing Coverage

### Scope

- Artboard, Node/Transform, Shape container
- Geometry: path, ellipse, rectangle, polygon basics
- Styling: solid fill, stroke, gradients (linear/radial)
- Paint ordering and draw rules

### Exit Criteria

- Golden scenes render correctly in runtime snapshot tests.

## Phase 5: Animation Timeline

### Scope

- Linear animation objects
- Keyframes for double, color, path vertices, bool
- Interpolators (hold, linear, cubic)
- CLI helpers for common transform animation patterns

### Exit Criteria

- Timeline playback behaves as expected in compatibility fixtures.

## Phase 6: Interactivity (State Machines)

### Scope

- State machine core (layers, states, transitions)
- Conditions (bool/number/trigger)
- Listeners/events and transition evaluation metadata

### Exit Criteria

- Interactive sample artboards respond correctly to inputs in runtime tests.

## Phase 7: Rigging, Text, Assets, Layout, Data Binding

### Scope

- Bones, skins, weights, constraints
- Text runs/styles and related properties
- Image/font/audio assets (metadata + linkage)
- Layout components
- ViewModel/data binding objects

### Exit Criteria

- Representative fixtures for each subsystem load and validate.

## Phase 8: AI Authoring Workflow

### Scope

- AI-safe prompt flow:
  - Prompt -> constrained plan JSON
  - JSON -> validated `SceneSpec`
  - `SceneSpec` -> `.riv`
- Deterministic "repair" loop from validator diagnostics
- Template library for common animation intents (loader, bounce, morph, hover, button states)
- Optional provider abstraction (OpenAI/local) via config

### Exit Criteria

- End-to-end command can produce useful animations from plain-English prompts with bounded retries.

## Phase 9: Tooling, Reliability, Release

### Scope

- `inspect` command (human and JSON output)
- Advanced validator diagnostics (property/type mismatch, missing refs)
- Fuzz/property tests for parser/validator
- Cross-platform packaging + release automation
- Comprehensive docs and cookbook

### Exit Criteria

- Versioned release with install instructions and reproducible examples.

## Cross-Cutting Standards

1. Upstream alignment: pin and track Rive runtime schema version.
2. Determinism: reproducible output for same input and config.
3. Compatibility: test output against at least one official runtime continuously.
4. Explainability: every validator error includes object path + fix hint.
5. Safety: AI-generated plans must pass schema + semantic guards before encode.

## Issue Strategy

- One tracking epic issue references all execution issues.
- Issues are dependency-ordered and each has explicit acceptance criteria.
- Labels:
  - `epic`
  - `area:encoder`
  - `area:objects`
  - `area:builder`
  - `area:cli`
  - `area:validator`
  - `area:ai`
  - `area:qa`
  - `priority:p0` / `priority:p1` / `priority:p2`

## Recommended Execution Order

1. Foundation + CI
2. Binary core + minimal validator
3. Object metadata codegen
4. Scene DSL + builder
5. Drawing
6. Animation
7. State machines
8. Rigging/text/assets/layout/data binding
9. AI authoring workflow
10. Reliability and release
