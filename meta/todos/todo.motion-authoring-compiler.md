---
node: rive-cli.intelligence.authoring
status: open
created: 2026-07-29
---

# P2 — Implement poses and compact motion tracks

Add named poses, shared easings, typed time and angle units, compact property
tracks, loops, stagger helpers, and compiler-selected lowering to valid Rive
animations and blend endpoints.

## Acceptance criteria

- Constant poses no longer require manually authored two-frame animations.
- Reused easing definitions lower once and resolve deterministically.
- Motion targets use authored IDs and typed property paths.
- Generated animations render deterministically at required evaluation frames.
- The control-panel level and button motion can be represented materially more compactly than raw SceneSpec.

## Dependency

Depends on the visual slice and AuthoringSpec source mapping.
