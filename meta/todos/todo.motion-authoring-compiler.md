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

## Remaining

- Shared easing definitions and deterministic easing reuse.
- Semantic entrance, exit, stagger, spring, bounce, and similar motion helpers.
- Color and other non-transform property tracks.
- A complex animated showcase with retained official-runtime frame evidence.

## Dependency

Depends on the visual slice and AuthoringSpec source mapping.
