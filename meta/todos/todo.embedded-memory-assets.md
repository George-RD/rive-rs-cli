---
node: rive-cli.core.compile
status: open
created: 2026-09-12
---

# Embedded A1: caller-owned image and font assets

Execution issue: #257. Parent spec: #256. PR: #275.
Branch: `agent/256-a1-memory-assets`. The independent #258 reference seed belongs
to another session and is not part of this change. #259 starts after #257 is
verified and merged.

## Implemented

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

The original observation records a real runtime run but its image-removal control
alone was insufficient to establish which image bytes were used. The strengthened
control changes only supplied image bytes, requires exactly the image pixels to
change to the supplied color, and preserves every other pixel. CI run 34702984571
passed that runtime test: 57,344 image pixels and 2,865 font-dependent text pixels.
Its overall run failed the separate nested-asset callback contract; it is not a
full-suite pass.

Review identified both invalid nested assets and unknown nested-artboard targets
as paths that could reach a resolver before late builder errors. A broader Node
validation change was tried but rejected before commit because it changed a legacy
fixture's accepted name handling. The oracle was not refreshed. Deferring byte
resolution until successful canonical construction fixes the callback ordering
without changing those semantics.

Run 34703563148 reproduced the late-reference failure, then passed 32 focused
resolver/compiler contracts, the library suite and Clippy on the corrected source
committed as `8b28800442283eb0eb8a8c6be810f0868426007b`. Its temporary development
workflow removed itself; no source-rewriting job belongs to the final product.

Final exact-head full Rust, MSRV, runtime/browser, Cairn and Standards/Spec review
remain merge gates on PR #275. This todo stays open until those gates are resolved.

See [the API contract](../../docs/embedded-assets.md) and
[the reproduction record](../../tests/evidence/embedded-assets/README.md).
