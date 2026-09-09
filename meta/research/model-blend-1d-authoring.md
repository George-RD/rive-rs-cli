---
id: res.model-blend-1d-authoring
nodes:
  - rive-cli.intelligence.authoring
  - rive-cli.core.builder
  - rive-cli.verification.rust
  - rive-cli.verification.browser
date: 2026-09-09
method: primary
---

# Numeric model-bound one-dimensional blends (#243)

## Reconciliation and native semantics

Main `74a88084b456318a1192e762751e094766c3e67a` contained merged PR #242 and no
open PRs. The remaining behavior todo explicitly listed model-bound one-dimensional
blends. #243 continues that acceptance after numeric properties/conditions and
model-bound direct weights, not a new roadmap track or low-level coverage programme.

Primary runtime source `rive-app/rive-runtime/src/animation/blend_state_1d_viewmodel.cpp`
imports the most recent `BindablePropertyImporter` and owns that property. Each
consumer therefore receives its own preceding bindable-number/context pair before
the native `BlendState1DViewModel`; reusing the synthetic input is insufficient.
Model/property path resolution and bindable emission reuse the existing canonical
builder. Authoring retains the existing named-input SceneSpec shape and strict
source union. Binding discovery and numeric source validation are shared with direct
blends. Hosts create, initialize and bind model instances; there is no input mirror.

## Observed test-first evidence

- Test-only commit `2f442f9540ee62a3f7359b4267a376660dfb12a7`, run `34306951457`,
  job `102325514248`, compiled and failed because `blend.binding` was unknown.
  The initial canonical-index assertion was corrected to the existing named-input
  representation before the green; the observed schema failure was unchanged.
- Run `34307286816`, job `102326512108`, passed the lowering contract on exported
  source `6e56dfa4a22e9147502afbc65f7a0d3a68152206`.
- The native-object contract first had a test-only byte-inspection type error. After
  correcting it to the public parser's byte-length representation, run `34307461860`,
  job `102327066340`, compiled and failed because no native view-model state existed.
  Lowering alone passed, proving that accepting the new source was insufficient.
- Run `34307566947` passed both public contracts after native-state emission and
  shared direct/one-dimensional bindable-number construction. Workflow triggering
  heads differ from the bot-published tested source; artifacts retain source heads.

Local cloning and Cargo execution were unavailable. Scoped GitHub Actions execute
Rust, schema generation and bundled-runtime proofs; exported sources and artifacts
support local inspection. A workbench push of `ci.yml` failed at the normal workflow
permission boundary, so the CI addition was made through the authorized connector.
No CI gates were removed or relaxed. Temporary workbench files are not delivery code.

## Verification contract

Thirteen public tests cover exclusive sources, authored reference/kind errors,
scalar parameters, stop bounds/f32 order, region-only binding discovery, shared
one-dimensional/direct/transition consumers, multiple charts with different input
offsets, nonzero model/multibyte property paths, native object ordering, legacy
input encoding, deterministic example output and atomic operation rollback.

The three-stop panel uses thresholds 0, 50 and 100 at x=40,120,200. The existing
browser harness adds a separate model-bound one-dimensional mode, preserving both
direct modes. It tests exact stops and interior ranges, not a false arithmetic-
midpoint promise for sequential neighboring-animation mixing. It also tests model-
only mutation, input-only non-effects, independence, clamping, reversal, timed
exit/resume and updates while inactive. JSON, binary, PNGs, positions and runtime/
WASM hashes are retained in the typed-behavior CI artifact. Final exact-head CI/MSRV
and separate Standards/Spec self-review are recorded on the delivery PR before merge.
