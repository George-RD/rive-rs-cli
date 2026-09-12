---
node: rive-cli.delivery.site
status: in-progress
created: 2026-09-12
---

# Rive-generated Pages landing page

User-requested continuation of the independent public-proof surface, implemented
in PR #276. It is independent of the parallel #257 asset resolver and #258 official
reference adapter. No compiler, crate-boundary or parity-result changes.

## Acceptance

- The full visible landing interface is generated with the public AuthoringSpec
  CLI, including typography, navigation labels, controls and motion.
- Rebuilding all three responsive compositions is byte-for-byte deterministic.
- Native Rive preset listeners and host numeric blending drive visible pixels.
- Pointer, touch and keyboard controls work while motion is paused.
- Resizing preserves user inputs and play intent; reduced motion, bfcache and
  unavailable runtime paths remain usable.
- Showcase and Verification Lab remain separate links and keep their existing
  correctness tests. The previous landing remains an explicit text alternative.
- Screenshots and exact-head browser evidence are inspected before merge.

## Evidence

Local public-CLI generation and validation pass on all three layouts. The
initial missing-generator contract failed before implementation; geometry,
source-map and public-output checks are retained. Runtime acceptance and visual
review remain pending until the repository runner returns its observation.
