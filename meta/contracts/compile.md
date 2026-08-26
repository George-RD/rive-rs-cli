---
node: rive-cli.core.compile
---

# Canonical SceneSpec compilation contract

The compilation seam owns the complete `SceneSpec -> .riv bytes` transition for application adapters. It accepts a parsed `SceneSpec`, an optional scene-relative asset base directory, and a file ID.

It must:

- delegate validation, reference resolution, and object construction to the canonical builder;
- delegate binary emission to the deterministic encoder;
- preserve relative asset resolution and caller-supplied file IDs;
- return typed, stable application-facing build errors without exposing runtime indices;
- remain the shared seam for raw SceneSpec generation and future AuthoringSpec adapters.

File reading, JSON parsing, output writing, and transport-specific response envelopes remain adapter responsibilities.
