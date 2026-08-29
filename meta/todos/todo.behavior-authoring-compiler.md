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

## Evidence

PR #202 / issue #179 establishes the first typed behavior tracer bullet on the shared Authoring compiler state: boolean view-model properties, authored bindings, named states, named transitions, deterministic source maps, canonical-builder validation, and retained official-runtime evidence that mutating the bound view-model boolean changes state. The slice preserves `raw_state_machines` as an expert escape without introducing a second full-document lowering pass.

PR #204 / issue #180 extends that same compiler path with authored boolean state-machine inputs, named Rive events, typed pointer/event listeners, boolean listener actions, and input-driven transitions. Authored visual and event IDs resolve to generated runtime names without exposing runtime indices; invalid input, event, action, and listener references report authored JSON paths. The retained behavior runtime contract compiles the typed fixture through `authoring compile` and drives the resulting `.riv` through both `render --input` and `render --pointer`, requiring both paths to converge on the same visible state.

This todo remains open. Issue #181 completes the complex interactive showcase and broader behavior exit gate.
