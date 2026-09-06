---
node: rive-cli.core.builder
---
# Canonical SceneSpec builder contract

`SceneSpec` is the complete, explicit, deterministic representation consumed by
the existing builder and encoder. It is the lowered IR, not the preferred model
input for complex AI authoring.

The builder must:

- reject invalid hierarchy, references, names, properties, and cycles before emission;
- keep reference resolution deterministic and independent of declaration accidents;
- preserve the public SceneSpec v1 contract while the Authoring frontend evolves;
- accept only explicit runtime concepts. Parametric sugar belongs in the Authoring module;
- remain usable directly as an expert/raw escape hatch.

## Transition exit-time gates

`TransitionSpec.exit_time` is optional whole milliseconds represented by `u32`.
When present, it sets the existing StateTransition exit-time property and named
EnableExitTime flag, including when zero. Other timing flags remain unset.
Omission/null preserves prior binary behavior. `duration` is independent.
Validation checks transition bounds before inspecting its source, then rejects any
exit-time gate whose source is not an animation state. Entry, exit, any, and blend
sources cannot silently bypass the gate. Source references and condition checks
retain the existing validate-first path and SceneSpec v1 remains unchanged.
