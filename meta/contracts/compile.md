---
node: rive-cli.core.compile
---

# Canonical SceneSpec compilation contract

The compilation seam owns the complete `SceneSpec -> .riv bytes` transition for
application adapters. `compile_scene` retains the optional scene-relative base
directory and explicit file ID. The #257 resolver-aware entry accepts a SceneSpec,
`CompileOptions` and a borrowed `AssetResolver`.

It must:

- delegate validation, reference resolution and object construction to one
  canonical builder, with global asset-name validation before resolver calls;
- delegate binary emission to the existing deterministic encoder;
- preserve caller-supplied file IDs, asset order and successful fixture bytes;
- expose stable logical asset diagnostics on the embedded API, while preserving
  the legacy entry point's `invalid-scene` classification and builder string errors;
- enforce per-asset and file-total supplied-byte limits, including repeated source
  use and assets spread across artboards;
- perform no implicit filesystem or network fallback for a missing supplied source;
- remain the shared seam for raw SceneSpec and lowered AuthoringSpec adapters,
  preserving the accepted compiler-owned scene and source-map state.

Memory resolution, byte budgets and signature-family checks are portable policy.
Filesystem reads, root discovery, path canonicalization and containment belong to
`FilesystemAssets`, not object construction. JSON parsing, output writes, runtime
rendering and transport response envelopes remain host-adapter responsibilities.
Custom resolvers own any side effects they introduce; the library cannot turn
arbitrary resolver code into a sandbox.

Identical SceneSpec/options/asset bytes must yield identical `.riv` bytes. A
resolver must be stable during compilation; callers must snapshot mutable external
sources when repeatability is needed. A missing `source` retains the existing
explicit external-asset declaration behavior and does not invoke a resolver.
See [embedded assets](../../docs/embedded-assets.md) for ownership, defaults,
diagnostics, host-policy limits and the separate compatibility/runtime proofs.
