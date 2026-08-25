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
- Motion source-path normalization and appended motion source entries are owned by
  that compiler state. Lower typed motion directly into the same scene draft next,
  then delete the raw-fragment bridge, cloned `AuthoringSpec`, and second full lower.
- Do not begin typed behavior/statechart lowering until typed motion and raw
  escapes share one compiler-owned scene draft without the cloned second lower.

## Why

The mixed typed/raw characterization in PR #169 fixes ordering, cross-reference,
source-map identity, canonical-builder acceptance, and authored diagnostic paths.
Introducing a state boundary before moving scene ownership keeps those contracts
stable while giving the one-pass migration one explicit home. Compiler-owned
output state and runtime-name validation removed one duplicate owner in PR #171;
compiler-owned checked bindings and target indexing removed the remaining
source-map scan from `motion.rs` in PR #172; PR #173 moves motion source-map path
normalization and easing source-entry construction under the same state boundary.
This leaves direct typed-motion scene mutation as the next isolated deletion
boundary rather than combining state design, indexing, source-map ownership, and
scene mutation in one review.

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
- PR #173 RED head `6d456927aa7007f5646181230d589289df2fb918`
  added private compiler-state contracts for motion source-path normalization and
  appended source-entry registration. CI run `32462071558` stopped at formatting,
  so no behavioral conclusion was drawn from that first head.
- Formatted RED head `252bc90395e933ebff65aed8c7d2c869d377779d`
  then failed minimum-Rust run `32462193068` exactly because
  `CompilerState::apply_motion_source_map` did not yet exist.
- Implementation head `5e1355dc5a12b4f4639705794894bf0e36cbe684`
  moved motion source-path normalization and easing source-entry construction into
  compiler state, replaced the mutating easing helper with a pure source-entry
  producer, and consolidated motion path rewriting so diagnostics and source maps
  share one helper.
- Stable CI run `32462491648` then exposed unrelated toolchain drift: Rust 1.98
  introduced `chunks_exact_to_as_chunks` against pre-existing `render/image.rs`;
  minimum-Rust run `32462491630` passed the authoring implementation. Commits
  `c2d10ce0879725f843330e727a2981cc71166ab6` and
  `166e831fd4c00c7231cfcc027f5a817d9ee49fc2` mechanically adopted the new
  constant-chunk APIs while preserving Rust 1.88 comparison semantics.
- Compatibility-fix head `166e831fd4c00c7231cfcc027f5a817d9ee49fc2`
  passed minimum-Rust run `32462871969` plus complete CI run `32462872156`:
  formatting, Clippy, all Rust tests, browser contracts, Cairn architecture
  validation, official-runtime evidence, demo, site, Playwright, and visual
  regression.
- Final code head `b25118b7e62c72f8865d91fe52ef5a570b46e187`
  updates appended runtime names incrementally instead of rebuilding the full
  registry and an unused post-motion target index. Exact-head minimum-Rust run
  `32463482887` and complete CI run `32463482955` both passed.

## Alternatives considered

- **Delete the second lowering pass immediately.** Reaches the target sooner but
  mixes state design, scene ownership, path repair, and behavioral migration.
- **Keep target indexing in `motion.rs`.** Avoids owned compiler bindings now but
  leaves source-map validation and target discovery outside their long-term owner.
- **Keep motion source-map mutation in `motion.rs`.** Avoids a small structured
  return type but leaves two owners for source-map paths and appended entries just
  before direct scene mutation.
- **Use self-referential borrowed bindings.** Avoids small string clones but makes
  `CompilerState` self-referential and harder to mutate safely.
- **Give behavior a separate compiler.** Reduces short-term coupling but creates
  competing symbol, runtime-name, and source-map ownership.

## Trade-offs

`CompilerState` still adapts through `LoweredAuthoring` around the existing motion
lowerer, so PR #173 does not yet remove the cloned second pass. The target index
owns runtime names and object-type strings to avoid self-referential state, and
`motion.rs` temporarily creates a small property-resolution adapter vector.
Binding-index failures are retained in state and surfaced after easing resolution
to preserve characterized diagnostic precedence. Appended source entries update
the runtime-name registry incrementally; the motion-target index is intentionally
not rebuilt because finalization does not consume it and the previous pipeline did
not re-index after the second lower.

In return, checked source-map bindings, target discovery, motion source-path
normalization, appended motion source entries, and runtime-name registration now
have one compiler-state owner. The next slice can lower typed motion directly into
the existing scene draft, append raw escapes afterward, and delete the clone,
second full visual lower, raw-fragment bridge, and remaining path-repair adapter.