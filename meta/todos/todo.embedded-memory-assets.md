---
node: rive-cli.core.compile
status: open
created: 2026-09-12
---

# Embedded A1: caller-owned image and font assets

Execution issue: #257. Parent spec: #256. Branch: `agent/256-a1-memory-assets`.
The independent reference seed #258 is being implemented in another session and
is not part of this change. #259 stays blocked until #257 is verified and merged.

## Acceptance criteria

- Compile a SceneSpec with an image and embedded font from caller-owned memory
  through the canonical builder and encoder, using explicit deterministic options.
- Existing compilation and builder callers forward through the same implementation;
  successful SceneSpec and AuthoringSpec fixture hashes, asset order and file IDs
  remain unchanged.
- Move filesystem/root/containment policy out of object construction into the host
  adapter, retaining relative, traversal, symlink, missing and empty contracts.
- Fail closed with logical asset diagnostics and per-payload/file-total limits.
  State ownership, repeatability, external-asset and custom-resolver assumptions.
- Retain positive official-runtime proof that both supplied assets drive visible
  output, separately from structural and byte-compatibility checks.
- Pass exact-head Rust, declared MSRV, runtime and Cairn gates; finish separate
  Standards and Spec reviews. Do not close #256 or unblock #259 on a partial draft.

## Current state

The branch contains an asset-resolution draft and a pinned, branch-only source
preparation job to work around the unavailable local Git/Rust environment. The
job must apply the actual builder patch, commit real Rust sources, and run the
contracts; it is not part of the compiler architecture and must be removed from
the final PR. There is no runtime dependency on source rewriting or a CI service.

Completion is not asserted here. The PR records the resulting exact commit,
checks that actually ran, failures, retained evidence, and both review axes.

## Constraints

Keep the single compiler-owned SceneSpec and source-map state. No workspace move,
RML parser, official CLI dependency, new asset format, or Yarnling-specific code.
See [the API and verification contract](../../docs/embedded-assets.md).
