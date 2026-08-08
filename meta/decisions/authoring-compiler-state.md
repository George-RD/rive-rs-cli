---
id: dec.authoring-compiler-state
nodes:
  - rive-cli.intelligence.authoring
  - rive-cli.core.builder
status: accepted
date: 2026-08-08
revisit_triggers:
  - "The one-pass migration cannot preserve the characterized source-map or diagnostic contracts"
  - "Typed behavior requires a separate lowering graph rather than the shared Authoring compiler state"
  - "The canonical builder exposes a higher-level incremental construction API that replaces the compiler-owned scene draft"
---

# Route Authoring lowering through one compiler state boundary

## Decision

- Keep `AuthoringSpec -> SceneSpec -> canonical builder` as the public and
  validation boundary.
- Route frontend lowering through one internal `AuthoringCompiler` state before
  deleting the existing cloned second lowering pass.
- In the first bounded slice, let the state own the borrowed authored document,
  the lowered target graph, typed-motion lowering, and final runtime-name
  validation without changing public schema, source maps, or diagnostics.
- Move resolved symbols, the canonical scene draft, runtime-name registry,
  checked runtime bindings, motion-target index, and source-map construction into
  that state in subsequent slices of the existing motion todo.
- Do not begin typed behavior/statechart lowering until typed motion and raw
  escapes share one compiler-owned scene draft.

## Why

The mixed typed/raw characterization in PR #169 fixes ordering, cross-reference,
source-map identity, canonical-builder acceptance, and authored diagnostic paths.
Introducing a state boundary before moving scene ownership keeps those contracts
stable while giving the one-pass migration one explicit home. A direct rewrite
would combine state introduction, behavior preservation, and deletion of the
second lower in one hard-to-review change.

## Evidence

- PR #169 established the mixed compiler characterization before production
  architecture changed.
- PR #170 initial head `3b59218633043918dd7e21198b92fe83c0505822`
  failed CI run `31263772976` and minimum-Rust run `31263773001` because
  `frontend.rs` was accidentally truncated; the failure was syntactic and no
  behavioral conclusion was drawn.
- Corrected implementation head `d01cbc0fb515afab4b436fa2951868bdb147cd8f`
  restored the characterized frontend and passed minimum-Rust run `31267191127`
  plus complete CI run `31267191151`: formatting, Clippy, all Rust tests,
  browser contracts, Cairn scan/lint, official-runtime evidence, demo, site,
  Playwright, and visual regression.

## Alternatives considered

- **Delete the second lowering pass immediately.** Reaches the target sooner but
  mixes state design, scene ownership, path repair, and behavioral migration.
- **Keep the existing free-function chain.** Smaller now, but leaves no stable
  owner for the scene draft and registries required by typed behavior.
- **Give behavior a separate compiler.** Reduces short-term coupling but creates
  competing symbol, runtime-name, and source-map ownership.

## Trade-offs

The first slice adds an internal type before it removes work, so it temporarily
wraps the existing pipeline rather than improving runtime cost. In return, each
subsequent deletion can be reviewed against the retained characterization suite,
and motion and behavior converge on one compiler-owned graph instead of growing
parallel lowering paths.
