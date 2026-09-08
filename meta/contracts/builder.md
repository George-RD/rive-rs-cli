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

## Numeric view-model transition bindings

A canonical number input accepts optional `view_model_binding` using the same
`{view_model, property}` form as boolean inputs. Omission/null preserves the existing
unbound path. A numeric binding must resolve to a direct number property of the
named artboard model. Initial and condition values must remain finite as f32;
missing references, wrong property kinds, non-numeric or overflowing conditions
fail before encoded output. Existing model/property index resolution and numeric
bindable-property/comparator objects are reused. Numeric model properties encode
the inherited ViewModelComponent name field, not Component name/parent fields.
Unbound numeric inputs and boolean binding binaries retain their existing behavior.
This affects transition evaluation only; model instance initialization belongs to
the host. No input synchronization, model-bound blend, or conversion is promised.
