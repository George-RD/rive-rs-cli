---
node: rive-cli.core.compile
status: done
created: 2026-09-12
completed: 2026-09-12
---

# Embedded A1: caller-owned image and font assets

Execution issue: #257. Parent spec: #256. Implementation PR: #275.
Branch: `agent/256-a1-memory-assets`. The independent #258 reference seed belongs
to another session and is not part of this change. #259 starts after PR #275 is
verified and merged; this completion record does not close the parent spec.

## Delivered

The public memory compilation entry, deterministic options and typed asset errors
use the existing builder and encoder. Both legacy entry points forward through
that builder without changing their signatures or adding legacy error variants.
Image/font ownership, explicit external assets, per-asset and cross-artboard total
budgets are documented and contract-tested. Filesystem discovery, bounded reads,
canonicalization and containment live in an adapter, not object construction.

All existing canonical construction checks complete before resolver callbacks.
Explicit sources reserve their contents positions in the same graph; bounded
resolution fills them before the graph can be returned. No parallel validation
walk, alternate graph or change to existing name-resolution semantics is needed.

There is no workspace move, RML parser, official CLI dependency, new asset type,
source-map state duplication, or application-specific compiler.

## Verification and review corrections

The pre-refactor oracle contains 95 successful legacy SceneSpec/AuthoringSpec
fixtures and four explicit skips. Every successful output hash is preserved.
The 32 focused asset contracts cover ownership, errors, limits, filesystem policy,
byte ordering and invalid-scene rejection before resolver callbacks.

Review exposed two callback-order gaps: invalid nested assets and unknown nested
artboard targets could reach caller code before late builder errors. A broader
Node-validation change was tried but rejected before commit because it changed a
legacy fixture's accepted name handling. The oracle was not refreshed. Deferring
byte resolution until successful canonical construction fixes both gaps while
preserving the original validator and name semantics.

Run 34703563148 reproduced the late-reference failure, then passed all focused
contracts, the library suite and Clippy on the corrected source committed as
`8b28800442283eb0eb8a8c6be810f0868426007b`. Its temporary development workflow
removed itself; no source-rewriting job remains in the product.

The original image-removal control was strengthened to vary only supplied image
bytes while preserving the entire scene and dimensions. In the final observation,
all 57,344 image pixels take the supplied replacement color and zero other pixels
change. Withholding font bytes removes exactly the same 2,865 glyph pixels as
removing text. Repeated captures and binaries match.

Full CI run 34703777838 attempt 2 passed on
`8a762224ed0dc95b6a8c658bf9030a3a9b393e93`: 1,198 Rust tests passed, zero failed;
the sole ignored test is the existing schema-regeneration utility. Formatting,
Clippy, Rust 1.88, browser/runtime evaluation, visual regression, demo/site and
Cairn gates passed. Attempt 1 had an existing console-test Chrome launch timeout;
its retry used unchanged source and assertions. Codex review of this head found
no major issues after the three earlier findings were addressed.

The final evidence/documentation commit changes neither source nor tests; PR #275
records its own exact-head checks and separate Standards/Spec author reviews.
Only merge after those checks pass. The historical and strengthened observations
remain distinct in the [reproduction record](../../tests/evidence/embedded-assets/README.md)
and [verified observation](../../tests/evidence/embedded-assets/verified-observation.json).
See the [public API contract](../../docs/embedded-assets.md).
