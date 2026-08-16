---
id: dec.authoring-compiler-state
nodes:
  - rive-cli.intelligence.authoring
  - rive-cli.core.builder
status: accepted
date: 2026-08-08
revisit_triggers:
  - "The one-pass migration cannot preserve the characterized source-map or diagnostic contracts"
  - "Typed behavior requires a separate lowering graph rather than the shared Authoring compiler state"
  - "The canonical builder exposes a higher-level incremental construction API that replaces the compiler-owned scene draft"
---

# Route Authoring lowering through one compiler state boundary

## Decision

- Keep `AuthoringSpec -> SceneSpec -> canonical builder` as the public and
  validation boundary.
- Route frontend lowering through one internal `AuthoringCompiler` state before
  deleting the existing cloned second lowering pass.
- `AuthoringCompiler` retains the borrowed authored document, while its
  `CompilerState` owns the canonical JSON scene draft, source-map state,
  runtime-name registry, checked runtime bindings, and motion-target index while
  preserving public schema, source maps, diagnostics, and ordering.
- Continue moving resolved symbols and mutation-oriented source-map construction
  into that state before lowering typed motion directly into the same scene draft.
- Do not begin typed behavior/statechart lowering until typed motion and raw
  escapes share one compiler-owned scene draft without the cloned second lower.

## Why

The mixed typed/raw characterization in PR #169 fixes ordering, cross-reference,
source-map identity, canonical-builder acceptance, and authored diagnostic paths.
Introducing a state boundary before moving scene ownership keeps those contracts
stable while giving the one-pass migration one explicit home. Compiler-owned
output state and runtime-name validation removed one duplicate owner in PR #171;
compiler-owned checked bindings and target indexing remove the remaining
source-map scan from `motion.rs` in PR #172. This leaves direct mutation and
source-map construction as the next isolated deletion boundary rather than
combining state design, indexing, scene mutation, and path repair in one review.

## Evidence

- PR #169 established the mixed compiler characterization before production
  architecture changed.
- PR #170 initial head `3b59218633043918dd7e21198b92fe83c0505822`
  failed CI run `31263772976` and minimum-Rust run `31263773001` because
  `frontend.rs` was accidentally truncated; the failure was syntactic and no
  behavioral conclusion was drawn.
- Corrected PR #170 implementation head
  `d01cbc0fb515afab4b436fa2951868bdb147cd8f` restored the characterized
  frontend and passed minimum-Rust run `31267191127` plus complete CI run
  `31267191151`.
- PR #171 RED head `e61a5c0566cddcd66efba4fa5acacdcda6c4a14e`
  failed CI run `31620093313` and minimum-Rust run `31620093283` because the new
  state contracts referenced the not-yet-implemented `CompilerState`.
- Implementation head `2581a8102be4a3ca1731e774ff298b0b4c8a6a0d`
  introduced compiler-owned output state and removed the duplicate frontend
  runtime-name validator. Its CI run `31620607421` stopped at formatting only,
  before Clippy or Rust tests, so no behavioral conclusion was drawn.
- Formatting head `fb5ed4d5f58e55139846d22a5c8bbaaa05253445`
  exposed one ambiguous iterator result type in both stable and Rust 1.88 checks.
  Exact head `dc6b89683b4a4ba0d8400be2c0292bbdd1838cde` made the
  registry-count invariant explicit and passed minimum-Rust run `31620929117`
  plus complete CI run `31620929128`: formatting, Clippy, all Rust tests, browser
  contracts, Cairn architecture validation, official-runtime evidence, demo,
  site, Playwright, and visual regression.
- Review RED head `8c026e43f6573c5b87646858ac842c0dc6ebbd15`
  passed minimum-Rust run `31621973310`; CI run `31621973309` passed formatting,
  Clippy, browser contracts, and every pre-existing Rust test, then failed only
  because the new characterization received `runtime_name_collision` before the
  prior `unknown_motion_target` diagnostic.
- Review fix `6d979ef368325b424fb89a1c1af533a9ab5398f0` records the
  first collision during state construction but reports it only at compiler
  finalization. Exact documentation head
  `5761fe98e3863c361c27ccddf41147781beec555` passed minimum-Rust run
  `31622213725` plus complete CI run `31622213661`: formatting, Clippy, all Rust
  tests, browser contracts, Cairn architecture validation, official-runtime
  evidence, demo, site, Playwright, and visual regression.
- PR #172 exact RED head `d7807cbb5da065e6000ac24f8ff5b72a4f627581`
  added compiler-state contracts for a checked motion-target index and rejected
  unpaired source-map bindings. Stable CI run `31961431662` stopped at formatting
  before Rust tests, while minimum-Rust run `31961431674` failed exactly because
  `CompilerState::into_motion_input` did not exist.
- PR #172 implementation head
  `b7bab3149c682f8b60628f07d49a74254d9781ff` moved checked bindings and target
  indexing into compiler state and removed their duplicate ownership from
  `motion.rs`. Minimum-Rust run `31961897868` passed; stable CI run
  `31961897869` stopped only on two rustfmt line wraps.
- Exact implementation head `7faf26a31a2782a4022e54463828473109717722`
  passed minimum-Rust run `31962017692` plus complete CI run `31962017694`:
  formatting, Clippy, all Rust tests, browser contracts, Cairn architecture
  validation, official-runtime evidence, demo, site, Playwright, and visual
  regression.

## Alternatives considered

- **Delete the second lowering pass immediately.** Reaches the target sooner but
  mixes state design, scene ownership, path repair, and behavioral migration.
- **Keep target indexing in `motion.rs`.** Avoids owned compiler bindings now but
  leaves source-map validation and target discovery outside their long-term owner.
- **Use self-referential borrowed bindings.** Avoids small string clones but makes
  `CompilerState` self-referential and harder to mutate safely.
- **Give behavior a separate compiler.** Reduces short-term coupling but creates
  competing symbol, runtime-name, and source-map ownership.

## Trade-offs

`CompilerState` still adapts through `LoweredAuthoring` around the existing motion
lowerer, so this slice does not yet remove the cloned second pass. The target
index owns runtime names and object-type strings to avoid self-referential state,
and `motion.rs` temporarily creates a small property-resolution adapter vector.
Binding-index failures are retained in state and surfaced after easing resolution
to preserve characterized diagnostic precedence. The post-motion state rebuilds
an index that finalization does not consume because the previous pipeline did not
re-index after the second lower.

In return, checked source-map bindings and target discovery now have one owner,
the duplicate scan and diagnostics are gone from `motion.rs`, and the next slice
can move mutation-oriented source-map construction and typed-motion emission into
the existing scene draft before deleting the raw-fragment bridge and path repair.
