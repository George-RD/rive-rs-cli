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
  `CompilerState` owns the canonical JSON scene draft, source-map state, and
  runtime-name registry while preserving public schema, source maps,
  diagnostics, and ordering.
- Continue moving resolved symbols, checked runtime bindings, the motion-target
  index, and mutation-oriented source-map construction into that state before
  lowering typed motion directly into the same scene draft.
- Do not begin typed behavior/statechart lowering until typed motion and raw
  escapes share one compiler-owned scene draft without the cloned second lower.

## Why

The mixed typed/raw characterization in PR #169 fixes ordering, cross-reference,
source-map identity, canonical-builder acceptance, and authored diagnostic paths.
Introducing a state boundary before moving scene ownership keeps those contracts
stable while giving the one-pass migration one explicit home. Moving output state
and runtime-name ownership next removes a duplicate free-function validator and
creates the stable container required for checked bindings and direct mutation,
without combining those later deletions into the same review.

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

## Alternatives considered

- **Delete the second lowering pass immediately.** Reaches the target sooner but
  mixes state design, scene ownership, path repair, and behavioral migration.
- **Keep the existing free-function chain.** Smaller now, but leaves no stable
  owner for the scene draft and registries required by typed behavior.
- **Give behavior a separate compiler.** Reduces short-term coupling but creates
  competing symbol, runtime-name, and source-map ownership.

## Trade-offs

`CompilerState` still adapts through `LoweredAuthoring` around the existing motion
lowerer, so this slice establishes ownership without yet reducing the second-pass
runtime cost. The runtime-name registry is rebuilt after that pass rather than
mutated incrementally, and it retains the first collision until compiler
finalization so authored motion diagnostics preserve their previous precedence.
In return, duplicate validation logic is gone, the scene and source map have one
explicit owner, and the next slice can migrate checked bindings and target
indexes before deleting the characterized adapter boundary.
