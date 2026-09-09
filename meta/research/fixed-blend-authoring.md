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

## Observed complete verification

Run `34358874270` verified source
`a99e6bf9df2bd4ab14ea418255c2f0502142023c`: formatting, all-target/all-feature
Clippy, 1,125 Rust tests passed, zero failed, and one existing regeneration helper
ignored. Cairn scan and lint passed with no errors; 97 warnings and 11 informational
findings remain visible. The full-suite schema guard was extended from two to three
strict sources, not removed. An earlier workbench Cairn invocation used unsupported
`--format json`; the successful run used the repository's normal `--json` flags.

Artifact `10107122169` (`fixed-blend-source`) retains exact source and logs, public
CLI compilation reports, binaries, runtime positions and frames. Archive SHA-256:
`21906ff777619fc6f2410fd5776b7af6e3e37985cd4cdbadb6d68a1ea1fb1ac0`.

All five shared runtime modes passed, with 62 measured samples: 17 fixed/input and
pure-constant cases, 11 fixed/model cases, plus the unchanged input (9), model (11)
and model-1D (14) modes. Pure-constant variants expose only reset/resume triggers;
no numeric inputs exist. Cases cover zero, fractional, half and full weights,
reversed contributions, rest-last ordering, and reset/resume transitions. Model
cases retain the input-only non-effect check.

For 25.5/75.5-percent contributions, the expected marker centers are 80.8/160.8 px;
observed raster centers are 80/160 px, within the existing one-pixel tolerance.
Reset returns both markers to 39.5 px and resume restores 80/160 px. Putting the
100-percent rest motion last returns both to 39.5 px, proving authored order rather
than normalized shares. The retained fractional PNG was visually inspected.

Runtime JavaScript SHA-256:
`9bfa2546433e72e7fb6e1cb63d863febe24563ba8644ddbb095518f2ef29b4a7`.
WASM SHA-256:
`0e018bfd0826a276c4fbefae4d3dd0fe1be127eed10bedce4268f3890e54d47b`.
Chromium: `151.0.7922.34`.

## Delivery gate and remaining scope

The permanent CI workflow adds both fixed modes without removing existing gates or
changing tolerances. After temporary workbench removal, exact delivery-head normal
CI, Rust 1.88 and separate Standards/Spec self-review are recorded on the PR before
merge. The behavior parent remains open; additive states, broader timing, other
model property kinds and conversions are not claimed by this slice.
