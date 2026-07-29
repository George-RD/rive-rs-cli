---
node: rive-cli.intelligence.authoring
status: open
created: 2026-07-29
---

# P0 — Specify AuthoringSpec v0 and the lowering boundary

Define the smallest strict JSON frontend that can represent a useful complex
scene while preserving `SceneSpec` as the canonical lowered IR.

## Required v0 concepts

- stable IDs and generated runtime names;
- typed units and safe expression ASTs;
- components, instances, and a raw SceneSpec escape hatch;
- explicit visual, motion, and behavior sections;
- deterministic lowering and an authored-to-expanded source map;
- versioning and validation errors reported against authored paths.

## Acceptance criteria

- Schema and compiler interfaces are approved before broad implementation.
- Characterization tests prove existing SceneSpec behavior is unchanged.
- Two representative authored examples lower deterministically and validate.
- No binary encoder logic is duplicated in the frontend.

## Dependencies

Runtime evidence must be available or landing in parallel so the frontend is
judged on official-runtime output, not only JSON shape.
