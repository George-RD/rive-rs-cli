---
id: res.model-blend-authoring
nodes:
  - rive-cli.intelligence.authoring
  - rive-cli.core.builder
  - rive-cli.verification.rust
  - rive-cli.verification.browser
date: 2026-09-09
method: primary
---

# Model-bound direct weights (#241)

## Reconciliation and native semantics

Main `40d9a5ee4e1b50819a44c3ff5bc49d611fdf44c8` contained merged #240 and no open
PRs. The remaining behavior todo explicitly called for model-bound direct blends.
#241 continues #175 after the completed number-binding and input-direct slices;
it does not reopen the independent low-level coverage programme.

The runtime header `rive-app/rive-runtime/include/rive/animation/blend_animation_direct.hpp`
defines direct sources input=0, fixed mix=1 and data bind=2. Its
`src/animation/blend_animation_direct.cpp` imports source 2 from the most recent
`BindablePropertyImporter`. These primary files were inspected through GitHub.
The implementation uses existing numeric bindable/context objects before the direct
child; pointing at the synthesized input alone would not bind a model weight.

The canonical number input's `view_model_binding` supplies the named model/property
path through the existing resolver. Explicit raw non-input sources are preserved.
Authoring uses an exclusive input/binding union and includes blend-only references
in used-binding discovery without a new lowering pass or index-repair mechanism.
The host initializes and binds the model instance; no input mirroring is promised.

## Observed test-first evidence

- Test-only source `21ec42762acc65a18098929dd856cff064d3812a`, run `34276198243`,
  job `102229687624`, compiled and failed because `binding` was an unknown direct
  child field. The contract also checks native encoded source 2 and bindable/context
  objects, so schema acceptance alone cannot satisfy it.
- The unchanged contract passed on source
  `b1d94ece335a3da50a5ec610ae0619e741f6e979`, run `34276456042`, together with
  all-target/all-feature Clippy.
- Expanded contracts and Clippy passed in run `34277015841`. They cover authored
  errors, shared/region-only references, scoped/multibyte indices, native ordering,
  explicit fixed-source preservation, old-byte compatibility and atomic rollback.
- Source `73214c76ff456c03536578b45fe75d3db2b46001`, run `34277179381`, passed
  all 1,101 Rust tests (one pre-existing schema helper ignored), Clippy and both
  official-runtime blend modes. Artifact `10076276538` retained source, logs and
  frames; its SHA-256 is
  `5e5f4f7440fd8c72c5dc0a627e445268eb3d3c51ab626476984d0d49d2a3c6e1`.

Local cloning and Rust execution were unavailable. Branch-scoped Actions performed
formatting, schema generation and executable verification; downloaded source/log
artifacts supported local inspection. Temporary workbench files are excluded from
the delivery diff. Final exact-head CI/MSRV and separate Standards/Spec self-review
are retained on PR #242; intermediate runs are not the final merge gate.

## Runtime evidence

`tests/playwright/authoring-direct-blend-runtime.js --model-bound` compiles the new
example through the public CLI and loads the bundled official runtime. It reuses the
existing input-driven harness and its pixel measurements instead of duplicating
that infrastructure. The model supplies independent left/right contributions after
a full-weight rest motion. Mutating synthesized inputs alone leaves both panels at
rest; model-only mutation changes their measured positions while those inputs remain
at unrelated 90/10 values.

The eleven cases cover initial binding, input-only mutation, zero/partial/full model
weights, independent contributions, reversal, out-of-range clamping, timed exit and
resume, and return to zero. Measured centres were 39.5px at zero, 119.5px at half
and 199.5px at full weight. The model-driven binary SHA-256 is
`bc107cad1542c95190a125b24ce97a6c62e05fd553972bc47f2cad3735c5f8d4`.
The downloaded artifact and all eleven PNG digests were checked locally, and
representative input-only and independent-weight frames were inspected.

Source, binary, compile report/source map, PNGs, positions, runtime/WASM hashes and
browser version are retained in `target/playwright-behavior/model-blend`, under the
normal typed-behavior CI artifact. The original input mode remains separately
verified. The parent behavior todo stays open; one-dimensional model blends,
additive states, static expression weights, other property kinds, broader timing
and conversions are not included.
