---
node: rive-cli.delivery.site
status: done
created: 2026-08-29
completed: 2026-08-30
---

# P1 — Separate public verification proof from original-work proof

Issue #198 defines an independent public-proof track for the GitHub Pages site. The
Verification Lab remains the upstream-versus-generated parity surface, while original
work moves to a separate showcase. This track reuses one site-to-Rive playback seam
without changing the AuthoringSpec compiler dependency graph.

## Execution slices

- #199 — frame-lock each Verification Lab pair behind one deterministic logical clock
  and reusable site playback interface; completed in PR #216.
- #200 — reuse that interface for a provenance-aware original-work showcase; completed
  in PR #218.
- #201 — lead the landing page with original/production proof while keeping parity as
  the separate correctness route; completed in PR #219.

## Acceptance criteria

- Verification Lab playback cannot create apparent parity defects from independent
  wall-clock start times.
- Parity metrics and representative frames remain sourced only from
  `parity/results.json`.
- Shared browser playback hides Rive readiness, deterministic seek, state-machine
  rebuild/replay, resize, and cleanup from page card builders.
- Original-work provenance remains separate from parity evidence.
- Staged browser coverage exercises the public pages with the vendored Rive runtime.
- The track remains independent of #179/#182 and does not add Authoring compiler
  blockers.

## Dependencies

- Parent specification: #198; completed by #199-#201.
- #199 completed the shared playback seam in PR #216; #200 reused it in PR #218;
  #201 completed the landing/production-proof slice in PR #219.
- No dependency was introduced on the completed #179-#186 Authoring chain.

## Paused-control clock correction (#229 / PR #230)

PR #222 left a known defect in the shared playback seam: separately dispatched
paused control updates advanced the state-machine clock while its displayed logical
frame stayed fixed. At 60fps, one control event after frame 30 advanced the runtime
from 500ms to 516.6667ms. Coalescing concurrent promises could not prevent this.

The controller now settles inputs and triggers at its existing deterministic
render timestamp, without incrementing its step count. The same draw/flush/cancel
path updates the visible state without allowing elapsed animation time to leak
into unrelated regions. Playing, backward replay, retained assigned inputs,
momentary triggers, cleanup, and failed-refresh recovery keep their existing paths.
No compiler, schema, binary, baseline, or parity-metric change is involved.

`node --test tests/playwright/site-playback-contract.js` exercises the public
single and paired timelines against a fake external Rive runtime. Seven contracts
cover sequential/concurrent controls, 30/60fps clocks, immediate rendering, same-
and forward-frame seeks, backward replay, trigger lifetime, playing, reduced
motion, destruction, and failed-refresh recovery. The unchanged main playback blob
`a3c5d937c6145c593ab6591d63ee4524a2c9a246` failed six contracts locally; the
zero-time refresh passes all seven. These are scheduler contracts, not a substitute
for official-runtime evidence.

`node tests/playwright/site-paused-controls.js` drives the committed interactive
console in the staged browser with the vendored runtime. It requires pixel-identical
frames after 25 separate unchanged-input tasks, immediate number/bool/trigger
response without timeline advancement, a stable same-frame seek, and actual motion
on the next explicit frame. Source/artifact hashes, PNGs, and the result are retained
in the `paused-control-playback-evidence` CI artifact. Test-only CI run
`34103077392` pins the red phase at `c1c8fef906b71e0ef716bfd5ea0ccccf5ba5db4b`;
final exact-head outcomes and separate Standards/Spec review are recorded on PR #230.

This is a correctness follow-up within the completed public-proof track, not a new
Authoring milestone. The broader behavior-authoring todo remains open.

## Startup investigation: retained diagnostics (#231)

The first bounded slice addresses the missing evidence in the existing showcase
harness. Its failure path now retains the stage, original error/stack, collected
console/page/request errors, viewport, per-card readiness/playing/logical-frame and
paint state, and a screenshot under `target/showcase-validation`. CI uploads that
directory and the harness log as `showcase-validation-diagnostics`, even on failure.
Loading and first paint have distinct stages; desktop, lifecycle, phone, and reduced-
motion waits identify their own context.

The normal validation and failure report share one card reader. An unreadable canvas
is reported separately from a canvas with zero painted pixels and cannot hide other
cards. Diagnostic operations are bounded and their failures are recorded separately,
so a closed/stalled page or unwritable artifact directory does not replace the original
failure. The existing validation timeouts, all-canvases-painted predicate, runtime,
assets, baselines, and production playback code are unchanged. No retries are added.

`node --test tests/playwright/showcase-diagnostics-contract.js` covers retained browser
state/PNG evidence, closed-page failure isolation, per-card paint-read failure, output
write failure, and a stalled page probe. The local red/green sequence exposed and
corrected loss of the original error on a closed page, loss of all cards when one canvas
was unreadable, and an unbounded pending probe. Local browser execution is diagnostic-
contract evidence, not a substitute for the pinned official-runtime CI suite.

That diagnostics-only slice did not establish or fix either reported startup cause.
The scheduler follow-up below reproduces the first-paint symptom; the earlier loading
symptom remains distinct. Keep this investigation independent of the Authoring
feature roadmap.

## Startup investigation: scheduler ownership (#231)

The vendored `assets/rive.js` schedules instance draws through the runtime's own
`requestAnimationFrame` queue, falling back to the browser only when that API is
absent. Runtime handles are not browser handles. `site/playback.js` incorrectly
passed runtime handles to browser cancellation, leaving automatic runtime draws
queued while potentially cancelling unrelated browser playback or paint callbacks.

Controlled playback now cancels through `instance.runtime.cancelAnimationFrame`
when available, preserving the browser fallback. The logical timeline's own browser
frame handles still use browser cancellation. No runtime assets, compiled scenes,
visual baselines, readiness timeouts, painted predicates, or clock semantics change.

Four public-controller contracts in `tests/playwright/site-scheduling-contract.js`
exercise two external scheduler namespaces and the browser-only fallback. They
require unrelated browser callbacks to survive initialization and automatic runtime
draws not to advance a controlled scene. The pinned production implementation fails
both runtime-queue contracts; the correction passes all four and the existing seven
playback contracts. CI runs the new contracts in the browser-contracts job.

Comparison run `34146258757`, head `46d7e422b8db4e7f4889c1b849efabbf4ccf3d99`,
retains base and corrected sources, Node 20.20.2 / Playwright 1.62.0, runtime/artifact
hashes, logs, PNGs, and result JSON in artifact `10027787158`. Its sources were checked
against base `602ca840f750e966ce85d116cea48ee682d172ba` and corrected playback blob
`1c4461fa174f76423928992b98b96717d554f0bf`.

Three predetermined showcase runs per implementation used identical vendored runtime,
assets, viewport and assertions, alternating comparison order. Base results were
pass/fail/pass: the second run timed out at `desktop:first-paint` after 30 seconds.
All six cards were ready with no collected browser errors; the Horaxon canvas had
zero painted pixels, and four timelines were stuck at frame zero while the other two
reached frame 1798. The failure JSON and screenshot retain that distinction.
All three corrected runs passed, including interaction, bfcache, phone layout and
reduced motion. Corrected site validation, paired playback and paused-control runtime
checks also passed. Twenty-five separately dispatched unchanged inputs preserve
identical frame-30 PNG hashes; explicit frame 31 advances motion.

These are fixed-count comparison results, not retry-until-green or a new CI retry
policy. Local Node tests also pass; full browser and Rust verification runs in GitHub
Actions because the local browser navigation is policy-blocked and Rust is unavailable.
Final exact-head CI and Standards/Spec self-review are recorded in the pull request.
At the end of PR #233, issue #231 remained open for the earlier loading-stage
timeout: the first-paint reproduction alone did not establish a shared cause.


## Startup investigation: readiness observation resolved (#231)

A separate reproduction now identifies why the original readiness wait could time
out despite successful loading. The browser-frame polling callback used by
`page.waitForFunction` was cancelled by the old `detachScheduledFrame` implementation.
The cards became ready, but the observer never ran again. PR #233 already fixes that
owner-level defect; no second production change is required.

Uninstrumented comparison run `34158421547` at head
`35e405465c81bb3cb2b3d5eeef95b8bd8bac70bb`, artifact `10031794719`, compares
`site/playback.js` from the original failing head
`c1c8fef906b71e0ef716bfd5ea0ccccf5ba5db4b`, pre-fix base
`602ca840f750e966ce85d116cea48ee682d172ba`, and corrected main
`14b5131db043b2d6ad52c600036e411ff91ada4f`. Each gets four predetermined runs,
with alternating order, fresh browser contexts, identical runtime/assets, a
1280x1000 viewport, reduced motion, and the original 20000ms RAF readiness predicate.
The historical and pre-fix versions each fail two of four runs; corrected main passes
all four. Every timeout snapshot shows all six cards ready at frame zero with no
collected browser/request errors. No controls are driven in this comparison.

Observational trace run `34158724551` at head
`f044cd887bf05a5e78618c3d4a122d742a5090f9`, artifact `10031893423`, repeats the same
fixed-count comparison, adding bounded callback and readiness-mutation recording.
Each old version again fails two of four runs; current passes all four. In each
failure, the trace retains a Playwright predicate callback cancelled by
`detachScheduledFrame` through browser cancellation, with no subsequent firing.
All cards report ready 194.8-235ms after navigation. One retained sequence is:

- Original failing version, round 1: poll handle 1 requested at 128.9ms;
  `detachScheduledFrame -> startStateMachine -> initialize` cancels it at 229.8ms;
  all six cards report ready at 235ms; the wait times out after 20010ms.
- Corrected version, round 4: both observed polling callbacks fire, neither is
  cancelled, and the readiness wait succeeds after 150ms. Corrected runtime draw
  cancellation passes through `ia.cancelAnimationFrame`, the owning runtime API.

The traces preserve callback text, request/cancellation stacks, readiness timestamps,
source bytes, hashes, screenshots, and the executed probe. Both runs use Node
20.20.2, Playwright 1.62.0, Chromium 151.0.7922.34, and the same vendored assets.
The corrected playback SHA-256 is
`acd31678f9157a2d8b8b9f6d24b3782438109088c6374893ce0bd738adb1d94d`;
the original failing test source is retained alongside the comparison.
The original historical artifact did not capture callback state, so this attributes
the reproduced failure on its pinned implementation, not an unavailable original
trace. It supplies the previously missing causal evidence without assuming that
all future readiness timeouts have this cause.

`site-paused-controls.js` restores explicit RAF polling instead of the later 50ms
polling workaround, without changing its 20-second limit or ready predicate. The
existing runtime test still checks 25 separate paused inputs, boolean/number/trigger
response, unchanged same-frame pixels, and forward-frame motion. It now uses the
shared bounded failure reporter instead of its duplicated unbounded page probe,
retaining the failing stage, request errors, card/frame/paint state, original stack,
PNG, and any secondary diagnostic failures. Playback/runtime hashes accompany the
existing source/artifact hashes. CI retains stderr fallback output with `pipefail`,
and report-write failure no longer bypasses browser/server cleanup.

This completes the causal investigation needed by #231 rather than introducing a
new scheduler or compiler milestone. The permanent deterministic scheduler contracts
remain in `site-scheduling-contract.js`; the restored official-runtime guard covers
the real readiness call site. Temporary tracing/comparison helpers are not retained
in the production diff. Final runtime verification, failure-injection results,
Standards/Spec self-review, and exact-head CI outcomes are recorded in the PR.
