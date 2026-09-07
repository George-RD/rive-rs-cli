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
