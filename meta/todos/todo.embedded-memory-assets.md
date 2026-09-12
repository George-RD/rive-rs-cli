---
node: rive-cli.core.compile
status: open
created: 2026-09-12
---

# Embedded A1: caller-owned image and font assets

Execution issue: #257. Parent spec: #256. PR: #275.
Branch: `agent/256-a1-memory-assets`. The independent #258 reference seed belongs
to another session and is not part of this change. #259 remains blocked until
#257 is verified and merged.

## Implemented

The public memory compilation entry, deterministic options and typed asset errors
use the existing builder and encoder. Both legacy entry points forward through that
same builder without changing their signatures or adding legacy error variants.
Image/font ownership, explicit external assets, per-asset and cross-artboard total
budgets are documented and contract-tested. Filesystem discovery, bounded reads,
canonicalization and containment live in an adapter, not object construction.
All file-scoped asset names are validated before any resolver callback.

There is no alternate SceneSpec graph, source-map state, encoder, workspace move,
RML parser, official CLI dependency, or Yarnling-specific code.

## Verification record

The first preparation run observed a missing-API red test and then passed 29 new
resolver/compiler contracts. Its independently captured manifest contains 95
successful legacy SceneSpec/AuthoringSpec fixtures and four explicit skips; every
successful output remained byte-identical after the compiler wiring.

Run 34693878309 passed the image/font official-runtime control test and the Rust
1.88 focused contracts on `5f991ef7665fe6182dcefb9a5489c86ef5664485`. The retained
[observation](../../tests/evidence/embedded-assets/observation.json) records source,
asset/runtime/browser identities, frame hashes and measured pixel differences.
The initial Playwright-executable launch failure is disclosed separately; no
runtime success is inferred from that attempt.

A subsequent test-only pixel-iteration change addresses Clippy without weakening
assertions or changing production sources. Temporary source preparation and
verification files have been removed. PR #275 carries final exact-head Rust,
MSRV, browser/runtime and Cairn gates plus separate Standards and Spec self-reviews.
This todo stays open until those final gates and review findings are resolved.

See [the API contract](../../docs/embedded-assets.md) and
[the reproduction record](../../tests/evidence/embedded-assets/README.md).
