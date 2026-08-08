---
node: rive-cli.intelligence.authoring
status: in_progress
created: 2026-07-29
---

# P2 — Implement poses and compact motion tracks

Add named poses, shared easings, typed time and angle units, compact property
tracks, loops, stagger helpers, and compiler-selected lowering to valid Rive
animations and blend endpoints.

## Acceptance criteria

- Constant poses no longer require manually authored two-frame animations.
- Reused easing definitions lower once and resolve deterministically.
- Motion targets use authored IDs and typed property paths.
- Generated animations render deterministically at required evaluation frames.
- The control-panel level and button motion can be represented materially more compactly than raw SceneSpec.

## Evidence

PR #157 establishes the first cohesive motion-compiler slice:

- named transform poses target visual nodes by authored ID;
- compact pose tracks lower deterministically to canonical SceneSpec keyframe groups;
- scalar expressions provide parameter-backed integer frame timing;
- `hold` and `linear` interpolation plus `oneshot`, `loop`, and `pingpong` loop modes are schema-bounded;
- typed tracks coexist with raw animation escapes without corrupting source-map indices;
- raw state-machine escapes can reference generated track runtime names in the same document;
- `tests/authoring_motion_contract.rs` covers deterministic lowering, canonical builder acceptance, exact authored-path diagnostics, schema exposure, raw-animation offsets, and behavior-reference integration;
- the generated `docs/authoring.schema.v0.json` records the public JSON contract.

TDD and review hardening for PR #157:

- RED `76a30fe` proved the strict frontend did not yet accept poses or tracks.
- RED `aaae4dd` proved preliminary lowering could not resolve a generated typed animation from a raw state-machine escape.
- GREEN `51ee8cc` separated visual-target discovery from final canonical validation so motion and behavior resolve together.
- CI run 688 passed the feature slice before the final unused-pose diagnostic case was added.
- RED CI run 690 then proved that the no-track fast path skipped semantic target validation for declared poses.
- Commit `a3b793f` resolves every declared pose before the no-track return, preserving the fast path while enforcing authored-target and expression diagnostics.
- Exact head `eb7216e` passed CI run 694 across formatting, Clippy, Rust, browser contracts, Cairn, official-runtime evidence, demo, site, Playwright, and visual regression.
- PR #157 merged to `main` as `7034e4f` on 2026-08-03.

The post-merge audit reconciled `ROADMAP.md`, this Cairn todo, open and merged
pull requests, retained branches, delayed reviews, and exact-head CI. It found no
new roadmap gap: the review remediation is hardening of this existing P2 todo.
PR #159 records that follow-up:

- exact-head RED `35a70b0` in CI run `30818628328` proved that aggregate pose/track expansion was unbounded and arithmetic expressions resolving within floating-point noise of a whole frame were rejected;
- validation now caps the complete typed-motion document at 10,000 generated property-keyframe values before Cartesian lowering;
- frame and duration expressions normalize only bounded round-off around whole numbers and continue to reject material fractional values;
- visual motion targets are indexed once instead of rescanning the full source map for every pose target;
- the checked pose-shape invariant now returns an authored `pose_shape_mismatch` diagnostic rather than using a panic shortcut;
- exact implementation head `197d4cd` passed rustfmt, Clippy, the complete Rust suite, browser contracts, and Cairn scan/lint in CI run 704 before the durable contract evidence was committed.

A review arriving immediately after #159 merged found that its absolute `1e-9`
rounding cap could be smaller than one floating-point ULP for large supported frame
values. PR #160 continues the same P2 hardening rather than opening a roadmap item:

- exact-head RED `4fd80d7` in CI run 708 passed formatting, Clippy, browser contracts, and every pre-existing Rust test, while the new large-frame regression failed with `invalid_frame`;
- the regression evaluates `(0.1 + 0.2) × 1_000_000_000`, which is one representable step above frame `300_000_000`;
- frame normalization now derives the actual ULP at the evaluated magnitude, retains the bounded eight-ULP/`1e-9` window at ordinary magnitudes, and floors that window at one ULP only when representable spacing is larger;
- exact implementation head `210929e` passed rustfmt, Clippy, the complete Rust suite, browser contracts, and Cairn scan/lint in CI run 709 before this durable contract evidence was committed.

A final unresolved Qodo thread on #159 found that the monotonic aggregate budget
emitted the same limit diagnostic for every track after the first crossing. PR #161
continues the same P2 diagnostic hardening:

- exact-head RED `4541ade` in CI run 713 passed rustfmt, Clippy, browser contracts, every pre-existing Rust test, and the existing 10,500-value rejection contract; only the new three-track regression failed because it received two limit diagnostics instead of one;
- the regression uses three individually valid 5,250-value tracks and requires exactly one `motion_keyframe_expansion_limit` at the second track, which first crosses the 10,000-value budget;
- validation now compares the aggregate count before and after each track, reports only the first threshold crossing, and continues validating later tracks for unrelated diagnostics;
- exact implementation head `e67bb23` passed rustfmt, Clippy, the complete Rust suite, browser contracts, and Cairn scan/lint in CI run 714 before this durable contract evidence was committed.

A late review thread on #160 found that its one-ULP floor accepted an exactly
authored half-frame once floating-point spacing reached half a frame. PR #162
continues the same P2 precision hardening:

- exact-head RED `e14a423` in CI run 718 passed rustfmt, Clippy, browser contracts, and every pre-existing Rust test; only the new half-frame regression failed because frame `2_251_799_813_685_248.5` was rounded up and accepted;
- `tests/authoring_motion_contract.rs` reuses the established motion fixture to pin both the half-frame boundary and the first magnitude where one ULP reaches a whole frame, without retaining a duplicate precision fixture;
- normalization retains the capped multi-ULP window below half-frame spacing, requires exact whole-frame equality when spacing reaches half a frame, and rejects magnitudes where spacing reaches a whole frame;
- exact implementation head `b2e72c9` passed rustfmt, Clippy, the complete Rust suite, browser contracts, Cairn architecture validation, site, demo, official-runtime evaluation, Playwright, and visual regression in CI run 720 before this durable contract evidence was committed.

The roadmap tracks incremental typed authoring operations as a separate P3
milestone rather than leaving that AI-skill unblock condition implicit.

The shared-easing continuation remains within this P2 todo:

- exact-head RED `35c2054` in branch workflow run `31091633813` retained a focused contract for reusable cubic easing definitions, scalar-expression control points, authored-path diagnostics, deterministic runtime deduplication, cross-track reuse, canonical builder acceptance, schema exposure, and a dedicated easing source-map entry;
- the frontend resolves every declared easing before the no-track return, bounds cubic time-axis control points, rejects hold/easing conflicts and unknown references, declares each referenced interpolator locally with one stable generated name, records every declaration path, and relies on the canonical builder to deduplicate identical definitions into one runtime object;
- implementation reuses the canonical SceneSpec interpolator path rather than adding encoder logic or exposing runtime indices.

Review hardening for PR #163 remains within this P2 slice:

- exact-head RED `ed94807` in CI run `31092625558` proves the JSON contract was exposed while Rust callers could not name `MotionEasingSpec` through the public typed API;
- the Rust 1.85 compatibility probe at `8ce6f23` in workflow run `31093299020` instead stopped at the locked dependency graph because `darling` 0.23 already requires Rust 1.88, so the repository now declares and continuously checks its actual Rust 1.88 minimum;
- implementation `adcf857` publicly re-exports `MotionEasingSpec`, retains the typed construction regression under Cairn ownership, and extracts frame timing into `motion/timing.rs` so this slice no longer pushes `motion.rs` beyond the module-size guideline;
- preflight workflow run `31094134192` passed the typed API regression, Rust 1.88 library check, formatting, Clippy, Cairn scan, and Cairn lint before durable evidence was committed;
- the existing oversized warnings for `constraint.rs`, `visual.rs`, and `lower.rs` predate this slice and remain separate architecture cleanup rather than expanding the easing PR.

The opacity continuation remains within this P2 todo:

- exact-head RED `cb9334a` in CI run `31110782636` passed formatting, Clippy, the Rust 1.88 minimum check, browser contracts, and every pre-existing Rust test; only the four new opacity contracts failed because the strict pose-target schema rejected `opacity`;
- pose targets may declare optional scalar opacity without a redundant transform object;
- opacity reuses the canonical SceneSpec `opacity` property-keyframe path, the shared ratio validator, easing resolution, deterministic naming, pose-shape checks, and expansion budget;
- transform and opacity property discovery is centralized in `motion/property.rs`, reducing duplicate counting and lowering logic while keeping `motion.rs` below the Cairn module-size guideline.

The dimension continuation and architecture review hardening remain within this P2
todo rather than opening a parallel feature milestone:

- PR #165 adds positive pixel-valued `width` and `height` pose properties for parametric shape geometry while preserving transform and opacity routing to the primary transform object;
- exact test-only head `df70913` in CI run `31179565122` passed formatting, Clippy, browser contracts, and every pre-existing Rust test; only the new compound-raw-target regression failed because width and height were routed to the first compatible geometry child;
- source-map runtime names and scene paths are now consumed through checked paired bindings, with a retained exception for an unnamed raw entry carrying one root scene path;
- malformed binding cardinality or a binding that does not resolve to a typed scene object returns `invalid_source_map_binding` rather than being silently truncated by `zip` and `filter_map`;
- the first registry-only exact-match implementation at `dc9c99a` exposed a second invariant in CI run `31179925223`: parametric geometry also advertises transform keys, so compatibility alone cannot identify semantic ownership;
- internal runtime bindings now carry `Transform`, `Geometry`, or `Other` roles before the canonical builder registry is consulted;
- zero compatible bindings return `unsupported_motion_property`, exactly one lowers normally, and more than one returns `ambiguous_motion_property_target` at the authored property path;
- public `SourceMapEntry` serialization remains unchanged, avoiding a source-map format break while the compiler gains checked internal bindings.

A late review on PR #165 found that treating primary identity as the `Transform`
role made a root raw parametric object lose its geometry capability. PR #168
continues the same P2 correctness work rather than creating a roadmap gap:

- exact regression-only head `3189283` passed the Rust 1.88 minimum in run `31185371891`, rustfmt, Clippy, all pre-existing tests, and browser contracts; CI run `31185371412` failed only because the new raw-rectangle contract received `unsupported_motion_property` at `$.motion.poses[0].targets[0].width`;
- runtime bindings now record primary identity independently from their semantic `Geometry` or `Other` role;
- transform and opacity properties select the primary binding, while width and height select parametric geometry; the canonical builder property registry remains the final compatibility check for both paths;
- a primary raw rectangle can therefore own `x`, `width`, and `height`, while a typed shape still routes dimensions to its child geometry and multiple compatible raw geometries remain ambiguous;
- the obsolete `Transform` role and its duplicated role-matching path were removed, and property selection is consolidated in `PoseProperty::supports_runtime_object`;
- exact implementation head `1fbfba1` passed the Rust 1.88 minimum in run `31185735792` and the complete repository suite in run `31185735638`: rustfmt, Clippy, all Rust tests, browser contracts, Cairn architecture validation, official-runtime evidence, demo, site, Playwright, and visual regression;
- the public schema and source-map format are unchanged.

The first one-pass compiler characterization slice remains within this P2 todo.
PR #169 pins the migration boundary before production architecture changes:

- `tests/authoring_compiler_characterization.rs` covers two typed tracks followed by two raw animation escapes, a raw state machine referencing the second typed track, repeated deterministic lowering, exact animation and source-map ordering, exact authored IDs and paths, runtime names, scene paths, canonical-builder acceptance, and a second-pass raw diagnostic rewritten across the generated two-track prefix;
- initial head `eff8fbb` passed the Rust 1.88 minimum in run `31187208884` and browser contracts, while CI run `31187208893` stopped only at `cargo fmt --check` before Clippy or Rust tests; no behavioral conclusion was drawn from that formatting-only failure;
- formatting-only head `f035f70` passed the Rust 1.88 minimum in run `31187442874` and the complete repository suite in run `31187441981`: rustfmt, Clippy, all Rust tests, browser contracts, Cairn architecture validation, official-runtime evidence, demo, site, Playwright, and visual regression;
- the slice changes no production code, public schema, source-map format, or product behavior; it establishes the exact contract that the one-pass compiler state must preserve while deleting the current clone, second lower, raw-fragment bridge, and string-offset repair path.

The compiler-state boundary continuation remains within this P2 todo. PR #170
introduces the migration owner before moving scene construction into it:

- the only open draft was continued after reconciling the roadmap, this todo, retained branches, review state, and exact-head CI; no new roadmap gap was found;
- initial head `3b59218` failed CI run `31263772976` and minimum-Rust run `31263773001` because `frontend.rs` was accidentally truncated, so no behavioral conclusion was drawn;
- corrected implementation head `d01cbc0` restored the characterized frontend and passed minimum-Rust run `31267191127` plus complete CI run `31267191151` across rustfmt, Clippy, all Rust tests, browser contracts, Cairn architecture validation, official-runtime evidence, demo, site, Playwright, and visual regression;
- frontend lowering now enters through one internal `AuthoringCompiler` that owns the borrowed document and current lowered target graph while preserving public schema, source-map JSON, diagnostics, ordering, and canonical-builder behavior;
- `dec.authoring-compiler-state` records why the state boundary precedes one-pass scene ownership, and the PR #169 characterization contract is now claimed by the Cairn Rust verification module.

## Architecture gate before further feature expansion

The public boundary remains:

`AuthoringSpec -> SceneSpec -> canonical builder -> encoder/validator/runtime proof`

Before typed behavior/statecharts or another broad Authoring feature slice, deepen
the implementation behind that boundary in this order:

1. **Characterized in PR #169.** Preserve mixed typed/raw animation, raw state-machine references to typed tracks, deterministic ordering, exact diagnostic paths, and source-map identity.
2. **Boundary introduced in PR #170.** Route frontend lowering through one internal `AuthoringCompiler` that initially owns the authored document and current lowered target graph.
3. Move resolved symbols, a canonical scene draft, the runtime-name registry, checked runtime bindings, the motion-target index, and the source-map builder into that state.
4. Lower assets and visuals into that state once; lower typed motion directly into the same scene draft; append raw escapes afterward; construct and validate canonical `SceneSpec` once.
5. Remove typed-motion conversion back into `RawSceneFragment`, the cloned/cleared second `AuthoringSpec`, the second full visual lowering, and string-based diagnostic/source-path repair.
6. Make typed behavior consume the same compiler state only after the one-pass motion path is characterized and verified.
7. Once compiler state exists, introduce a validated internal authoring model so ordinary authored symbol/reference rules have one user-facing owner; retain canonical lowerer and builder checks as defense in depth.
8. Consolidate only stable contract-test helpers such as `literal`, `lower`, diagnostic assertions, and keyframe lookup. Keep scenario JSON local so tests continue to expose the public document contract.

This is internal architecture hardening inside the existing motion and behavior todos,
not a new product milestone. Preserve exact diagnostics and public source-map JSON.
Do not split cohesive solver, registry, visual sum-type, or integration-test modules
solely because they exceed a line-count guideline.

## Remaining

- Complete compiler-state and one-pass lowering steps 3–8 above before behavior/statecharts or broad Authoring expansion.
- Semantic entrance, exit, stagger, spring, bounce, and similar motion helpers.
- Color and additional non-transform property tracks.
- A complex animated showcase with retained official-runtime frame evidence.

## Dependency

Depends on the visual slice and AuthoringSpec source mapping.
