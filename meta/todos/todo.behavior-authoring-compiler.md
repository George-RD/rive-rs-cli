---
node: rive-cli.intelligence.authoring
status: open
created: 2026-07-29
---

# P2 — Implement view-model-first behavior and named statecharts

Add typed model properties, bindings, events, named states, named transitions,
parallel regions, and compiler lowering to Rive view models, data bindings, blend
states, listeners, and indexed state-machine objects.

## Acceptance criteria

- Authored transitions never reference array indices.
- Bindings express source, target, and conversion intent with typed validation.
- The compiler selects bindings, poses, or blend animations without changing authored intent.
- Interaction tests drive pointer and input events and retain runtime evidence.
- A complex interactive showcase is reproduced through the frontend.

## Dependency

Depends on the motion slice for pose and blend-state lowering. Do not begin typed
behavior implementation until the one-pass Authoring compiler architecture gate in
`todo.motion-authoring-compiler.md` is complete. Behavior must consume the same
compiler-owned scene draft, resolved-symbol model, runtime-name registry, checked
runtime bindings, and source-map builder; it must not introduce another raw-fragment
re-entry pass or a second full document lowering.
