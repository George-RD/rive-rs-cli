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
- Route frontend lowering through one internal `AuthoringCompiler` state.
  Assets and visuals lower once into a resumable draft. For tracked documents,
  the provisional visual SceneSpec is canonically validated before motion so the
  established visual-diagnostic precedence is preserved. Typed motion is then
  emitted into the same final animation list before raw escapes, and the final
  assembled SceneSpec is canonically validated after raw content is appended.
- `AuthoringCompiler` retains the borrowed authored document, while its compiler
  state owns the resumable scene draft, source-map state, runtime-name registry,
  checked runtime bindings, and motion-target index while preserving public
  schema, source maps, diagnostics, and ordering.
- Typed track entries receive their final animation paths directly. Raw animation
  paths are created at their final typed-prefix offset while they are appended.
  The cloned/cleared second `AuthoringSpec`, typed-motion `RawSceneFragment`
  bridge, second full visual lower, and bridge-specific diagnostic/source-path
  repair are removed. General authored-path rewriting for expanded visual and
  component diagnostics remains.
- Raw animation IDs are validated at the final typed/raw animation merge so typed
  and raw animations share the same authored-ID namespace without moving that
  diagnostic ahead of typed motion validation. On no-track documents, raw content
  is finalized before unused pose semantics are checked so the previous raw-error
  precedence remains stable.
- Do not begin typed behavior/statechart lowering until the delivery roadmap's
  animated Authoring exit gate is complete; behavior must reuse this compiler
  state rather than introducing a second lowering graph.

## Why

The mixed typed/raw characterization in PR #169 fixes ordering, cross-reference,
source-map identity, canonical-builder acceptance, and authored diagnostic paths.
Introducing a state boundary before moving scene ownership kept those contracts
stable while giving the one-pass migration one explicit home. Compiler-owned
output state and runtime-name validation removed one duplicate owner in PR #171;
compiler-owned checked bindings and target indexing removed the remaining
source-map scan from `motion.rs` in PR #172; PR #173 moved motion source-map path
normalization and easing source-entry construction under the same state boundary.
A review regression proved that source-map normalization had to happen before
runtime-name registry construction because the registry retains authored paths in
collision diagnostics.

PR #192 completes that migration boundary. The visual/component graph is lowered
once, typed motion produces final animation objects and source entries without
converting back to raw fragments, and raw escapes append after the typed prefix at
their final scene indices. Review hardening showed that deleting the second
lowering pass must not delete the observable validation boundaries it used to
provide: tracked documents still need visual-only canonical validation before
motion, typed/raw authored IDs still form one namespace at their merge, and
no-track raw-fragment failures still precede unused pose semantics. These checks
now operate on the same compiler-owned draft instead of recreating the authored
graph.

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
- Final pre-review code head `b25118b7e62c72f8865d91fe52ef5a570b46e187`
  updates appended runtime names incrementally instead of rebuilding the full
  registry and an unused post-motion target index. Exact-head minimum-Rust run
  `32463482887` and complete CI run `32463482955` both passed.
- After PR #173 was marked ready, Codex review found that the post-motion registry
  captured a raw-animation collision path before typed-prefix normalization. RED
  head `702fb2c39958a27f8c66d53d2fbb9c6522b688f4` added the focused regression:
  minimum-Rust run `32831632752` passed and CI run `32831632768` failed exactly
  one new Rust test, with 625 tests passing and the collision reported at
  `$.motion.raw_animations[1].value` instead of authored
  `$.motion.raw_animations[0].value`.
- Review fix `bff756f389e6801b212d372270b60d73e34019fd` normalizes the
  post-motion source map before constructing source-map-derived compiler state,
  then appends new motion source entries incrementally. Minimum-Rust run
  `32831885096` passed; CI run `32831885008` stopped only on one rustfmt line wrap,
  so no behavioral conclusion was drawn from that stable run.
- Exact formatted review-fix head `da535aae3665c0a6e88b73f927cad3fc1bf5d94c`
  passed minimum-Rust run `32832102805` and complete CI run `32832102823`:
  formatting, Clippy, all Rust tests, browser contracts, Cairn architecture
  validation, official-runtime evidence, demo, site, Playwright, and visual
  regression.
- PR #192 implementation head `f5c7ae682f2255b30f16338cc5e6171d78975f5f`
  passed minimum-Rust run `32866478672` and complete CI run `32866478752`:
  formatting, Clippy, all Rust tests including the unchanged compiler and motion
  characterization contracts, browser contracts, Cairn architecture validation,
  official-runtime evidence, demo, site, Playwright, and visual regression.
- Codex review on `8c0f0298e0f1d89dde8642d7ec5be255f8f82e37` found that typed/raw
  authored animation IDs no longer shared one namespace and that no-track raw
  fragment failures had moved behind pose validation. Review regressions now pin
  both contracts plus raw/raw duplicate-ID timing; hardened head
  `aa0524bca31651115fb2cd463f019a15749aeb31` passed minimum-Rust run
  `32871336107` and complete CI run `32871336092`.
- Final Codex re-review on `aa0524bca31651115fb2cd463f019a15749aeb31`
  found that tracked canonical visual validation had moved behind motion semantic
  errors. `tests/authoring_compiler_review_regressions.rs` now pins that ordering,
  and `PartialLowering` validates its provisional visual scene before motion while
  retaining the same authored draft. Review-fix head
  `6fbf9188a52d3151772b416562be9c58f4fd99bd` passed minimum-Rust run
  `32872297769` and complete CI run `32872297773` across formatting, Clippy, all
  Rust tests, browser contracts, Cairn architecture validation, official-runtime
  evidence, demo, site, Playwright, and visual regression.

## Alternatives considered

- **Delete the second lowering pass immediately.** Reaches the target sooner but
  mixes state design, scene ownership, path repair, and behavioral migration.
- **Keep target indexing in `motion.rs`.** Avoids owned compiler bindings now but
  leaves source-map validation and target discovery outside their long-term owner.
- **Keep motion source-map mutation in `motion.rs`.** Avoids a small structured
  return type but leaves two owners for source-map paths and appended entries just
  before direct scene mutation.
- **Patch stored collision diagnostics after path normalization.** Avoids changing
  state-construction order but creates a second path-mutation owner and risks
  leaving other source-map-derived state stale.
- **Use self-referential borrowed bindings.** Avoids small string clones but makes
  compiler state self-referential and harder to mutate safely.
- **Give behavior a separate compiler.** Reduces short-term coupling but creates
  competing symbol, runtime-name, and source-map ownership.
- **Skip provisional canonical validation.** Avoids one builder preflight on
  tracked documents but changes the established visual-vs-motion diagnostic
  precedence. The one-pass goal removes duplicate authored lowering, not required
  validation of an already-lowered draft.

## Trade-offs

The resumable `PartialLowering` remains a JSON-oriented draft rather than a typed
incremental `SceneSpec` builder API. Motion-target discovery therefore materializes
a provisional scene `Value` from the already-lowered visual graph. Tracked
documents canonically validate that provisional visual scene before motion and
validate the final assembled scene after motion/raw append. This intentionally
adds a second canonical validation pass while still performing authored visual and
component lowering only once; preserving public diagnostic precedence takes
priority over avoiding validation work. The target index owns runtime names and
object-type strings to avoid self-referential state, and `motion.rs` still creates
a small property-resolution adapter vector. Binding-index failures remain retained
until easing resolution to preserve characterized diagnostic precedence, while
runtime-name collision reporting remains delayed until compiler finalization.

In return, the cloned second `AuthoringSpec`, second full visual lower,
typed-motion `RawSceneFragment` bridge, typed-prefix diagnostic/source-map repair,
and duplicate post-motion compiler-state rebuild are gone. Typed motion and raw
escapes now share one final SceneSpec assembly path. The next delivery slice is
#177's shared SceneSpec-to-`.riv` compilation seam; typed behavior remains
downstream of the animated Authoring exit gate and must reuse this compiler state.
