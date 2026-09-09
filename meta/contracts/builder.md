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
Model instance initialization belongs to the host. No input synchronization or
conversion is promised.

## Numeric view-model direct-blend bindings

An input-source canonical direct-blend child pointing to a number input with
`view_model_binding` consumes that model property, not the synthesized input value.
The builder resolves model/property indices through the same resolver as numeric
transition conditions. It emits `BindablePropertyNumber`, its `DataBindContext`
using the numeric value property key and encoded source path, then the
`BlendAnimationDirect` with data-bind source 2 and no input ID. This ordering is
required by the runtime's most-recent-bindable-property importer. The bindable
property uses the validated finite initial value from the canonical number input.

An absent or null source selector means input source 0. Unbound input-driven output
is unchanged. Explicit non-input source selectors are not reinterpreted; in
particular a fixed mix source 1 keeps its mix value and does not gain a binding.
The canonical schema and existing binary object types are unchanged.

## Numeric view-model one-dimensional blend bindings

A canonical `blend_state_1d` whose number input has `view_model_binding` consumes
that model property. Input references by name or index use the same resolved input.
The builder emits `BindablePropertyNumber`, its `DataBindContext`, then
`BlendState1DViewModel`, followed by the existing ordered animation children.
Each native state owns a distinct bindable property; source resolution may be shared
but importer-owned objects must not be shared across consumers. The finite initial
value and encoded model/property path reuse the direct-blend helper and resolver.
Unbound number inputs still emit `BlendState1DInput` with unchanged bytes.
The canonical schema stays unchanged; hosts initialize and bind model instances.
