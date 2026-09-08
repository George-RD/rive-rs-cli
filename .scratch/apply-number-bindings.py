from pathlib import Path
import subprocess

files = {}


def replace(path, before, after, count=1):
    text = files.setdefault(path, Path(path).read_text())
    if text.count(before) != count:
        raise RuntimeError(f'patch anchor count for {path}: {before[:90]}')
    files[path] = text.replace(before, after)


def append(path, text):
    files[path] = files.get(path, Path(path).read_text()) + text


if '## Numeric view-model binding slice (#237)' not in Path('meta/todos/todo.behavior-authoring-compiler.md').read_text():
    builder = 'src/builder/state_machines.rs'
    replace(builder, '''                        if let Some(binding) = view_model_binding {
                            let (view_model_id, property_id, property) = resolve_view_model_binding_ids(''', '''                        if let Some(binding) = view_model_binding {
                            if !value.is_finite() {
                                return Err(format!("bound number input '{}' requires a finite initial value", name));
                            }
                            let (view_model_id, property_id, property) = resolve_view_model_binding_ids(''')
    replace(builder, '''                                            .and_then(json_value_to_f32)
                                            .is_none()''', '''                                            .and_then(json_value_to_f32)
                                            .filter(|value| value.is_finite())
                                            .is_none()''')
    replace('src/objects/data_binding.rs', '''impl RiveObject for ViewModelPropertyNumber {
    fn type_key(&self) -> u16 {
        type_keys::VIEW_MODEL_PROPERTY_NUMBER
    }
    fn properties(&self) -> Vec<Property> {
        vec![
            Property {
                key: property_keys::COMPONENT_NAME,
                value: PropertyValue::String(self.name.clone()),
            },
            Property {
                key: property_keys::COMPONENT_PARENT_ID,
                value: PropertyValue::UInt(self.parent_id),
            },
        ]
    }
}''', '''impl RiveObject for ViewModelPropertyNumber {
    fn type_key(&self) -> u16 {
        type_keys::VIEW_MODEL_PROPERTY_NUMBER
    }
    fn properties(&self) -> Vec<Property> {
        vec![Property {
            key: property_keys::VIEW_MODEL_COMPONENT_NAME,
            value: PropertyValue::String(self.name.clone()),
        }]
    }
}''')
    replace('.github/workflows/ci.yml', '''      - name: Run typed behavior runtime contract
        run: node tests/playwright/authoring-behavior-runtime.js''', '''      - name: Run typed behavior runtime contracts
        run: |
          node tests/playwright/authoring-behavior-runtime.js
          node tests/playwright/authoring-number-binding-runtime.js''')
    replace('cairn.blueprint', '''                "./tests/authoring_operations_contract.rs",''', '''                "./tests/authoring_number_binding_contract.rs",
                "./tests/authoring_operations_contract.rs",''')
    public = 'docs/authoring-spec-v0.md'
    replace(public, 'typed boolean view models and bindings,', 'typed boolean and numeric view models and bindings,')
    replace(public, 'A behavior model may currently declare boolean properties,', 'A behavior model may declare boolean or numeric properties,')
    replace(public, 'Transition conditions are an untagged union of four forms:', 'Transition conditions are an untagged union of five forms:')
    replace(public, '- `{"binding": ..., "equals": <bool>}` checks a view-model binding.', '- `{"binding": ..., "equals": <bool>}` checks a boolean view-model binding.\n- `{"binding": ..., "compare": ..., "value": <scalar expression>}` checks a numeric view-model binding with the same six comparisons as a number input.')
    replace(public, '## Transition duration\n', '''## Numeric view-model bindings

A model property accepts `{"kind": "number", "id": "load", "value": <scalar expression>}`. Its value and a bound comparison's threshold use document parameters and scalar units. Unknown parameters, incompatible units, and values that overflow or underflow the runtime's `f32` representation fail at the authored expression path, including unused model properties.

For example, a named binding can drive a transition without an authored runtime index:

```json
"when": {
  "binding": "gate-load",
  "compare": "greater_or_equal",
  "value": { "kind": "parameter", "name": "threshold" }
}
```

`examples/authoring/number-binding.v0.json` provides the complete model, binding, parameters, motion and reverse transition. The binding must name a numeric property for this condition, or a boolean property for `equals`; a mismatch reports `invalid_condition_binding` at the transition's `.when.binding`. An unknown binding reports `unknown_behavior_binding` there. Root transitions and parallel regions share this validation and lowering path.

**A bound condition reads the model instance, not the synthesized machine input.** The property's authored `value` initializes that input, following the existing boolean convention. It does not serialize a default view-model instance or continuously synchronize the input. The host must create an instance, initialize its numeric property, and bind that instance to the runtime. Resolve the model and property runtime names from the compile report's source map, then use the runtime's `viewModelByName`, `instance`, `number` and `bindViewModelInstance` APIs. Changing only a synthesized input does not change the model condition. Typed blend inputs and listener actions still target explicitly declared machine inputs; converters and model-bound blend inputs are not part of this slice.

`node tests/playwright/authoring-number-binding-runtime.js` compiles the example through the public CLI, proves that changing only the machine input does not transition, then mutates the model through 59, 60, 90 and 59 against a threshold of 60. It measures the rendered panel before, at and after the boundary and after reversal. The typed-behavior CI artifact retains authored JSON, the compile report/source map, the binary, PNGs, measured positions and runtime/binary/frame hashes.

## Transition duration
''')
    contract = 'meta/contracts/authoring.md'
    replace(contract, 'untagged union of four\nforms:', 'untagged union of five\nforms:')
    replace(contract, '''`binding` form instead resolves against `bindings` and returns
`unknown_behavior_binding` at `$....transitions[i].when.binding`.''', '''`binding` forms instead resolve against `bindings`: `equals` requires a boolean
property, while `{"binding": ..., "compare": ..., "value": <scalar expression>}`
requires a number property. A mismatch returns `invalid_condition_binding` and an
unknown id returns `unknown_behavior_binding`, both at the authored `.when.binding`.
Numeric model values and comparison thresholds use the document's parameter scope,
scalar units and existing finite/f32-range validation, including unused properties.
The compiler reuses one synthesized input for a binding within a chart and assigns
separate runtime names across charts; regions use their chart's input. The condition
reads the bound model instance, not that input. The authored value initializes only
the synthesized input; the host must initialize and bind its model instance.''')
    replace(contract, 'view-model properties other than `bool`', 'view-model properties beyond `bool` and `number`')
    append('meta/contracts/builder.md', '''
## Numeric view-model transition bindings

A canonical `number` input may carry the same optional `view_model_binding`
object as a `bool` input: `{"view_model": <name>, "property": <name>}`. Omission
or null preserves the existing unbound-number path. A numeric binding must resolve
to a direct `view_model_property_number` child of the named artboard model, and
both the input's initial value and each condition's numeric value must remain
finite as runtime `f32` values. Missing references, wrong property kinds, and
non-numeric or overflowing condition values are rejected before encoded output.

The existing builder resolves model/property indices and emits
`TransitionViewModelCondition`, `BindablePropertyNumber`, `DataBindContext`,
`TransitionPropertyViewModelComparator` and `TransitionValueNumberComparator`.
Its source path uses the existing variable-length integer encoding, including
multi-byte property indices. Numeric model properties serialize the inherited
`ViewModelComponent` name field, not generic `Component` name/parent fields.
Unbound number conditions and valid boolean binding binaries retain their existing
object paths. No new runtime type, binary version or second compiler is introduced.

Binding affects transition evaluation only. It does not synchronize the synthesized
state-machine input or create/initialize a view-model instance; the host owns that
instance. Raw model-bound blend inputs and model-property conversion remain outside
this contract. Public compile/encoded-property tests and the official-runtime
numeric binding proof enforce this boundary.
''')
    roadmap = 'ROADMAP.md'
    text = Path(roadmap).read_text()
    files[roadmap] = text.replace('view-model properties other than `bool`', 'view-model properties beyond `bool` and `number`').replace('view-model properties beyond `bool` remain open', 'view-model properties beyond `bool` and `number` remain open')
    replace(roadmap, 'Select further work from an explicit unblocked issue', '''[#237](https://github.com/George-RD/rive-rs-cli/issues/237), implemented in PR #238,
adds numeric view-model properties and named bound comparisons through the existing
compiler/builder path. Root charts and parallel regions share typed validation;
parameters, all six comparisons, model/property indexing and atomic edits are covered.
The official-runtime proof separates model mutation from machine-input mutation and
retains threshold crossing and reversal evidence. Numeric property-name serialization
and bound-number finite-value validation are corrected. Model instance initialization
remains the host's responsibility; conversions, string/enum models and model-bound
blends remain outside this slice. The parent behavior todo stays open.

Select further work from an explicit unblocked issue''')
    todo = 'meta/todos/todo.behavior-authoring-compiler.md'
    replace(todo, 'but `BehaviorPropertySpec` still has one variant, `bool`, so no numeric or enumerated conversion can be authored.', '`BehaviorPropertySpec` supports `bool` and `number`, but numeric conversions and enumerated properties remain outside the typed frontend.')
    replace(todo, 'view-model properties other than `bool`', 'view-model properties beyond `bool` and `number`')
    replace(todo, 'non-boolean view-model properties.', 'string/enum view-model properties and conversions.')
    append(todo, '''
## Numeric view-model binding slice (#237)

PR #238 adds document-scoped scalar expressions for numeric model properties and
`{binding, compare, value}` conditions, with all six numeric comparisons. The shared
behavior compiler owns model/property names, binding kinds, chart-scoped inputs,
region lowering and source maps. Invalid kind pairs return `invalid_condition_binding`
at the authored condition path. The existing canonical builder resolves named number
bindings, rejects invalid references/kinds and runtime-float overflow, and emits its
existing numeric model-condition objects. `ViewModelPropertyNumber` now uses the
inherited view-model name field; the previous generic-component field left numeric
properties undiscoverable by name in the official runtime.

The property's authored value initializes the synthesized machine input only. A bound
condition reads a host-initialized model instance; it is not input synchronization.
Converters, string/enum models, model-bound blends and listener writes to model
properties remain out of scope, and this parent todo remains open.

`tests/authoring_number_binding_contract.rs` covers the public lowering/compile/encoded
seams, all operators, document parameters, authored diagnostics, unused-property
validation, multi-model and multi-byte property indices, chart/region source maps,
determinism, compatibility, finite canonical values and atomic rejection.
`tests/playwright/authoring-number-binding-runtime.js` compiles the retained
`number-binding.v0.json` example and distinguishes input-only mutation from model
values 59/60/90/59 at a threshold of 60. Source, binary, PNGs, measured positions and
hashes are retained under the existing typed-behavior runtime CI artifact.

Observed TDD failures: run `34251480151` rejected the new numeric property variant;
run `34252234877` accepted four invalid binding-kind/condition cases; runtime run
`34253194379` could not find the numeric model property by its source-mapped name;
run `34253812557` passed thirteen contracts and failed the two regressions for numeric
property-name encoding and overflowing canonical floats. Standards/Spec self-review
and final exact-head CI/MSRV evidence are recorded on PR #238. Rust execution and
schema generation used GitHub Actions because local Cargo was unavailable.
''')
    replace('CHANGELOG.md', '## [Unreleased]\n\n### Added\n', '''## [Unreleased]

### Added

- **Numeric view-model transition bindings.** Authored model properties accept `number` values as document-scoped scalar expressions. `{binding, compare, value}` conditions support all six numeric comparisons in root charts and parallel regions, with authored type/reference diagnostics and deterministic source maps. The existing canonical builder resolves numeric model bindings and rejects wrong property kinds, non-numeric comparisons and values that overflow runtime floats. Numeric properties now encode their inherited view-model name field, fixing official-runtime lookup. A retained example and runtime proof distinguish model mutation from machine-input changes and verify threshold crossing and reversal. The authored value initializes the synthesized input only; hosts must initialize and bind their model instances. Conversions, string/enum models and model-bound blend inputs remain outside this slice.
''')
    for path, text in files.items():
        Path(path).write_text(text)

subprocess.run(['git', 'add', '.github/workflows/ci.yml'], check=True)
