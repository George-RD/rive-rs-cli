---
node: rive-cli.delivery.site
status: open
created: 2026-08-29
---

# P1 — Separate public verification proof from original-work proof

Issue #198 defines an independent public-proof track for the GitHub Pages site. The
Verification Lab remains the upstream-versus-generated parity surface, while original
work moves to a separate showcase. This track reuses one site-to-Rive playback seam
without changing the AuthoringSpec compiler dependency graph.

## Execution slices

- #199 — frame-lock each Verification Lab pair behind one deterministic logical clock
  and reusable site playback interface; completed in PR #216.
- #200 — reuse that interface for a provenance-aware original-work showcase.
- #201 — lead the landing page with original/production proof while keeping parity as
  the separate correctness route.

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

- Parent specification: #198.
- #199 completes the shared playback seam in PR #216; #200 reuses it.
- #200 blocks #201.
- No dependency is introduced on the completed #179-#186 Authoring chain.
