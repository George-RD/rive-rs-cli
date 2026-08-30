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
