---
node: rive-cli.intelligence.authoring
status: in_progress
created: 2026-07-29
---

# P2 — Implement poses and compact motion tracks

Add named poses, shared easings, typed time and angle units, compact property
tracks, loops, stagger helpers, and compiler-selected lowering to valid Rive
animations and blend endpoints.

## Acceptance criteria

- Constant poses no longer require manually authored two-frame animations.
- Reused easing definitions lower once and resolve deterministically.
- Motion targets use authored IDs and typed property paths.
- Generated animations render deterministically at required evaluation frames.
- The control-panel level and button motion can be represented materially more compactly than raw SceneSpec.

## Evidence

PR #157 establishes the first cohesive motion-compiler slice:

- named transform poses target visual nodes by authored ID;
- compact pose tracks lower deterministically to canonical SceneSpec keyframe groups;
- scalar expressions provide parameter-backed integer frame timing;
- `hold` and `linear` interpolation plus `oneshot`, `loop`, and `pingpong` loop modes are schema-bounded;
- typed tracks coexist with raw animation escapes without corrupting source-map indices;
- raw state-machine escapes can reference generated track runtime names in the same document;
- `tests/authoring_motion_contract.rs` covers deterministic lowering, canonical builder acceptance, exact authored-path diagnostics, schema exposure, raw-animation offsets, and behavior-reference integration;
- the generated `docs/authoring.schema.v0.json` records the public JSON contract;
- CI run 688 passed formatting, Clippy, Rust, browser, official-runtime, Cairn, visual-regression, demo, and site gates.

## Remaining

- Shared easing definitions and deterministic easing reuse.
- Semantic entrance, exit, stagger, spring, bounce, and similar motion helpers.
- Color and other non-transform property tracks.
- A complex animated showcase with retained official-runtime frame evidence.

## Dependency

Depends on the visual slice and AuthoringSpec source mapping.
