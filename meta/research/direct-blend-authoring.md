---
id: res.direct-blend-authoring
nodes:
  - rive-cli.intelligence.authoring
  - rive-cli.core.builder
  - rive-cli.verification.rust
  - rive-cli.verification.browser
date: 2026-09-08
method: primary
---

# Input-driven direct blends (#239)

## Reconciliation and scope

Main `63032288ff67181d3c23697b52d8bd3bd07df9bf` had no open pull requests. The
ordered AuthoringSpec/public-proof issues were complete. The parent behavior todo
explicitly retained direct blends, so #239 is a bounded continuation of #175 rather
than an independent roadmap programme.

A state accepts `direct_blend: {motions: [{motion, input}]}` as an alternative to
`motion` or the existing one-dimensional `blend`. Only chart-local number inputs
and typed motion tracks are exposed. Existing canonical direct-blend objects own
runtime semantics; actual emitted animation/input arrays own numeric addressing.
Authored order is retained. No new binary encoder or host-side simulation is added.

## Observed red/green

- Test-only head `5e3641a8d22c2c7162b85392f9ac7c1fb6a20469`, Actions run
  `34258204739`, compiled then failed because `direct_blend` was unknown.
- Run `34258438290` generated source head
  `8283b4a6b0dbd2aaec1b44963ff2ff20c2a6c85a`; the unchanged lowering/index/source-map
  test passed, and all-target/all-feature Clippy passed.
- Run `34259653669` generated source head
  `717d10d5a8a8b127601dbdffa87e72bdf76a0efb`; the expanded public contracts,
  locked all-feature Rust suite and all-target/all-feature Clippy passed.

Local network cloning and Cargo execution were unavailable. Source editing was
performed through the connected GitHub API, with branch-scoped temporary Actions
for Rust execution, formatting and generated schema. Downloaded source snapshots
were used for local inspection and review. The workbench files are removed from
the delivery diff; the runtime contract is part of normal CI. See delivery PR #240
for observed runtime measurements, retained artifacts, final exact-head CI/MSRV
and separate Standards/Spec self-review. Intermediate runs do not replace that
final merge gate.

## Runtime contract

`examples/authoring/direct-blend-panel.v0.json` supplies a full-weight baseline
motion first, then separate left/right motion contributions. The public CLI compiles
the source; the bundled official runtime receives the named number and trigger
inputs. Pixel positions, not host-side animation calculations, determine success.

`tests/playwright/authoring-direct-blend-runtime.js` retains source, binary, compile
report, representative PNGs, measured positions and hashes in
`target/playwright-behavior/direct-blend`. It checks zero/partial/full weights,
independence, clamping, reversal, leaving/resuming the state and return to zero.
The normal `typed-behavior-runtime-evidence` CI artifact retains this evidence.

## Remaining scope

The parent behavior todo stays open. Static weight expressions, additive states,
model-bound direct blends, advanced exit timing, other property kinds and binding
conversions are not implemented by this slice.
