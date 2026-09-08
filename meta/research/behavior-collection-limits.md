---
id: res.behavior-collection-limits
nodes:
  - rive-cli.intelligence.authoring
  - rive-cli.verification.rust
date: 2026-09-08
method: primary
---

# Typed behavior collection limits

Bounded correctness slice of [the existing behavior compiler todo](../todos/todo.behavior-authoring-compiler.md)
and [#175](https://github.com/George-RD/rive-rs-cli/issues/175), tracked in
[#235](https://github.com/George-RD/rive-rs-cli/issues/235) and
[PR #236](https://github.com/George-RD/rive-rs-cli/pull/236). This does not complete
or create a replacement for the broader behavior roadmap item.

## Finding and contract

At baseline `f91101bb43b39f71ec8d9e3f5b8ebf7f55a69872`, `src/authoring/spec.rs`
published collection bounds through schemars but the public typed lowering path did
not enforce them. Serde parsing also accepted these out-of-schema collection sizes.

`lower_authoring` now checks behavior collection cardinality after the existing
visual expansion preflight and before frontend lowering. The JSON entry point and
incremental operations share that same boundary. The first cardinality violation
returns `invalid_behavior_collection_count` at its authored collection path, with
the actual count and inclusive accepted range. The preflight checks the three
top-level typed collections first, then model properties, then each statechart's
collections, listener actions, and regions, all in deterministic iteration order.

The published maximum of 1000 applies separately to models, bindings, statecharts,
model properties, chart inputs/events/listeners/states/transitions/regions,
listener actions, and region states/transitions. State collections require at least
one item. Other listed collections may be empty. An empty behavior section remains
valid. Two independently bounded collections may together exceed 1000 items.

Blend stops retain the existing 2..=1000 validation and `invalid_blend_stops`
diagnostics. Raw state-machine escapes keep their existing uncapped collection
contract. This is schema alignment, not a total allocation, input-byte, or CPU
budget. It does not change valid SceneSpec output, source-map identities, the
schema, the binary format, or the canonical builder/encoder ownership.

## Retained test-first evidence

- Model regression at `4f4e745d5db6b96bc961181b08614f135b69c057`:
  [CI 34245393905](https://github.com/George-RD/rive-rs-cli/actions/runs/34245393905)
  passed formatting and Clippy, then failed because 1001 otherwise-valid models
  lowered successfully. Artifact: `10063802267`.
- Minimal model guard at `7f3e646b49397e36b0c1eeac383c8f55f74b72c0`:
  [CI 34245923757](https://github.com/George-RD/rive-rs-cli/actions/runs/34245923757)
  passed the regression and full Rust suite; Rust 1.88 checking also passed.
  Rust diagnostics artifact: `10064040881`.
- Extended contracts at `f069d04bc7d47cd46a3271187e341ffe5415ce4e`:
  [CI 34246913092](https://github.com/George-RD/rive-rs-cli/actions/runs/34246913092)
  passed formatting and Clippy. Six new contracts passed, including every
  exactly-1000 boundary, raw escapes, empty optional collections, and schema
  parity. Three failed: remaining upper-bound rejection, empty-state collection
  diagnostics, and rejection of an oversized operation batch. Artifact:
  `10064412025`. This is the behavioral red before extending the guard.

Earlier formatting-only failures are not behavioral-red evidence. Execution came
from GitHub Actions because direct cloning and a local Rust toolchain were not
available in this session. Final exact-head verification and separate Standards
and Spec self-review evidence belong on PR #236; do not infer a passing final
result from the intermediate runs above.

## Regression surface

`tests/authoring_behavior_limits_contract.rs` exercises the public typed and JSON
lowering APIs, canonical building at all 13 upper boundaries, both state minima,
optional empty collections, independently bounded collections, raw escape
compatibility, schema parity, and atomic operation failure. Existing blend,
source-map identity, behavior runtime, and interactive showcase contracts remain
unchanged and must still pass.
