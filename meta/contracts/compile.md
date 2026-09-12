---
node: rive-cli.core.compile
---

# Canonical SceneSpec compilation contract

The compilation seam owns the complete `SceneSpec -> .riv bytes` transition for
application adapters. `compile_scene` retains the optional scene-relative base
directory and explicit file ID. The resolver-aware entry accepts a SceneSpec,
`CompileOptions` and a borrowed `AssetResolver`.

It must:

- delegate validation, reference resolution and object construction to one
  canonical builder, completing its existing checks before resolver callbacks;
- delegate binary emission to the existing deterministic encoder;
- preserve caller-supplied file IDs, asset order and successful fixture bytes;
- expose stable logical asset diagnostics on the embedded API, while preserving
  the legacy entry point's `invalid-scene` classification and builder string errors;
- enforce per-asset and file-total supplied-byte limits, including repeated source
  use and assets spread across artboards;
- perform no implicit filesystem or network fallback for a missing supplied source;
- remain the shared seam for raw SceneSpec and lowered AuthoringSpec adapters,
  preserving the accepted compiler-owned scene and source-map state.

Some canonical reference checks run during object construction, not the initial
structural pass. Source-bearing assets reserve their contents positions in that
same graph. Only after construction succeeds does the builder resolve and fill
those positions. No incomplete contents slot can be returned or encoded. This
must not become a second validation pipeline or change accepted name resolution.

Memory resolution, byte budgets and signature-family checks are portable policy.
Filesystem reads, root discovery, path canonicalization and containment belong to
`FilesystemAssets`, not object construction. JSON parsing, output writes, runtime
rendering and transport response envelopes remain host-adapter responsibilities.
Custom resolvers own any side effects they introduce; the library cannot turn
arbitrary resolver code into a sandbox or roll back their external side effects.

Identical SceneSpec/options/asset bytes must yield identical `.riv` bytes. A
resolver must be stable during compilation; callers must snapshot mutable external
sources when repeatability is needed. A missing `source` retains the existing
explicit external-asset declaration behavior and does not invoke a resolver.
See [embedded assets](../../docs/embedded-assets.md) for ownership, defaults,
diagnostics, host-policy limits and the separate compatibility/runtime proofs.
