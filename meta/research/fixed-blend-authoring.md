---
id: res.fixed-blend-authoring
nodes:
  - rive-cli.intelligence.authoring
  - rive-cli.verification.rust
  - rive-cli.verification.browser
date: 2026-09-09
method: primary
---

# Fixed direct-blend weights (#245)

## Gap and native semantics

Main `9835234e5f495b6fece43b7658c825abc20b63a6` contained merged PR #244 and no
open PRs. The behavior todo remained open; #240 had explicitly excluded static
weights. The direct panel's `foundation` number input existed only to hold a rest
motion at 100 percent. #245 closes that bounded authoring gap under #175, rather
than adding another roadmap track or a new binary type.

Primary source `rive-app/rive-runtime/include/rive/animation/blend_animation_direct.hpp`
defines `DirectBlendSource::mixValue = 1`, beside input source 0 and data-bind source
2. The canonical builder already preserves explicit fixed sources. The frontend
therefore emits existing `blend_source: 1` and `mix_value`, with no synthetic input,
model context, encoder change or second lowering pass. Each authored child selects
exactly one input, binding or scalar-expression weight. Constants are checked in
f64 before narrowing, preventing values just above 100 from rounding into validity.

## Observed test-first evidence

- Run `34356229933`, job `102481491328`, compiled the initial public contract and
  failed because `BehaviorDirectBlendMotionSpec` did not accept a fixed weight.
  Exported test-only source was `e28f476e04700c969c96fb33da5b71736b70bb77`.
- Run `34356566182` passed native lowering/encoding and no-synthetic-input checks
  after the minimal typed source and compiler implementation.
- Run `34356975522`, job `102484022169`, then compiled and failed the range contract:
  -0.001 incorrectly lowered to a fixed runtime value. The original native-object
  test still passed. The implementation adds one shared checked evaluator.
- Run `34357601961` passed all eleven expanded public contracts, including strict
  forms, parameters/fractions, expression diagnostics, scoped regions, mixed
  fixed/input/model offsets, authored ordering, rollback and legacy model bytes.

Local Cargo and outbound cloning were unavailable. Scoped Actions executed Rust and
retained the tested source and logs. Workflow trigger SHAs may precede the formatted
bot-published tested source; exported artifacts record that source explicitly.
Temporary workflow/patch files are removed from the delivered change.

## Runtime and merge gate

The shared direct-blend harness preserves its input, model and model-1D modes and
adds fixed/input and fixed/model modes. The checked example removes `foundation`;
pure-constant variants use scalar parameters and expose only two trigger inputs.
Pixel assertions cover zero, 25.5/75.5, half, full, reversed contributions, rest-last
ordering and timed reset/resume. Fixed/model checks retain input-only non-effects.
Sources, compile reports, binaries, PNGs, positions and JS/WASM hashes are retained.
Final exact-head CI, Rust 1.88 and the separate Standards/Spec self-review are
recorded on the delivery PR; structural lowering alone is not the runtime gate.
