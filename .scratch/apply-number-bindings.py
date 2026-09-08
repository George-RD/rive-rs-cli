from pathlib import Path
import subprocess


def replace(path, before, after):
    target = Path(path)
    text = target.read_text()
    if before not in text:
        if after in text:
            return
        raise RuntimeError(f'missing patch anchor: {path}: {before[:70]}')
    target.write_text(text.replace(before, after))


def append(path, marker, addition):
    target = Path(path)
    text = target.read_text() if target.exists() else ''
    if marker not in text:
        target.write_text(text + addition)


replace('.github/workflows/ci.yml', '''      - name: Run typed behavior runtime contract
        run: node tests/playwright/authoring-behavior-runtime.js''', '''      - name: Run typed behavior runtime contracts
        run: |
          node tests/playwright/authoring-behavior-runtime.js
          node tests/playwright/authoring-number-binding-runtime.js''')
replace('cairn.blueprint', '                "./tests/authoring_operations_contract.rs",', '                "./tests/authoring_number_binding_contract.rs",\n                "./tests/authoring_operations_contract.rs",')
replace('docs/authoring-spec-v0.md', 'typed boolean view models and bindings,', 'typed boolean and numeric view models and bindings,')
replace('docs/authoring-spec-v0.md', 'A behavior model may currently declare boolean properties,', 'A behavior model may declare boolean or numeric properties,')
replace('docs/authoring-spec-v0.md', 'Transition conditions are an untagged union of four forms:', 'Transition conditions are an untagged union of five forms:')
replace('docs/authoring-spec-v0.md', '- `{"binding": ..., "equals": <bool>}` checks a view-model binding.', '- `{"binding": ..., "equals": <bool>}` checks a boolean view-model binding.\n- `{"binding": ..., "compare": ..., "value": <scalar expression>}` checks a numeric view-model binding with the same six comparisons as a number input.')
replace('docs/authoring-spec-v0.md', 'view-model number and trigger properties remain outside', 'view-model properties beyond boolean and number remain outside')
if '## Numeric view-model bindings' not in Path('docs/authoring-spec-v0.md').read_text():
    replace('docs/authoring-spec-v0.md', '## Transition duration\n', '''## Numeric view-model bindings

A model property accepts `{"kind": "number", "id": "load", "value": <scalar expression>}`.
Its initial value and a bound comparison threshold use document parameters and
scalar units. Unknown parameters, incompatible units, non-finite numbers, and
values that overflow or underflow the runtime's f32 representation fail at the
authored expression path, including unused model properties.

```json
"when": {
  "binding": "gate-load",
  "compare": "greater_or_equal",
  "value": { "kind": "parameter", "name": "threshold" }
}
```

`examples/authoring/number-binding.v0.json` provides the complete model, binding,
parameters, motion and reverse transition. The binding must name a numeric property
for this condition, or a boolean property for `equals`; a mismatch reports
`invalid_condition_binding` at `.when.binding`. An unknown binding reports
`unknown_behavior_binding` there. Root transitions and parallel regions share this
validation and lowering path.

**A bound condition reads the model instance, not the synthesized machine input.**
The property's authored value initializes that input, following the existing boolean
convention. It does not serialize a default view-model instance or continuously
synchronize the input. The host must create, initialize, and bind its model instance.
Resolve model and property runtime names from the compile report's source map:

```javascript
const instance = runtime.viewModelByName(modelRuntimeName).instance();
const load = instance.number(propertyRuntimeName);
load.value = 25;
runtime.bindViewModelInstance(instance);
load.value = 60;
```

Changing only a synthesized input, including through `render --input`, does not
change the model condition. Typed blend inputs and listener actions still target
explicitly declared machine inputs. Converters, string/enum/trigger model properties,
model-bound blends, and listener writes to model properties are outside this slice.

The official-runtime test compiles this example through the public CLI, proves that
input-only mutation does not transition, then mutates the model through 59, 60, 90
and 59 against a threshold of 60. The typed-behavior CI artifact retains authored
JSON, the compile report/source map, binary, PNGs, measured positions and hashes.
Both schema versions remain unchanged; published schemas include the new forms.

## Transition duration
''')
replace('examples/authoring/README.md', '| `behavior-binding.v0.json` | Boolean view-model binding driving a named typed statechart |', '| `behavior-binding.v0.json` | Boolean view-model binding driving a named typed statechart |\n| `number-binding.v0.json` | Parameterized numeric view-model threshold with forward and reverse named transitions |')
replace('examples/authoring/README.md', '''exit time, and view-model number or trigger properties
are not exposed by the AuthoringSpec frontend.''', '''advanced exit timing, and view-model properties beyond boolean and number
are not exposed by the AuthoringSpec frontend. The numeric binding example requires
host initialization of the model instance, not just a machine-input assignment.
See [numeric model bindings](../../docs/authoring-spec-v0.md#numeric-view-model-bindings).''')
replace('meta/contracts/authoring.md', 'untagged union of four\nforms:', 'untagged union of five\nforms:')
replace('meta/contracts/authoring.md', '''`binding` form instead resolves against `bindings` and returns
`unknown_behavior_binding` at `$....transitions[i].when.binding`.''', '''`binding` forms instead resolve against `bindings`: `equals` requires a boolean
property, while `{"binding": ..., "compare": ..., "value": <scalar expression>}`
requires a number property. A mismatch returns `invalid_condition_binding` and an
unknown id returns `unknown_behavior_binding`, both at the authored `.when.binding`.
Numeric model values and thresholds use document parameters, scalar units and the
existing finite/f32-survival checks, including unused properties. Each using chart
gets one synthesized input per binding, shared across its regions. The condition
reads a host-initialized bound model instance, not a synchronized machine input.
The authored property value initializes the synthesized input only.''')
replace('meta/contracts/authoring.md', 'view-model properties other than `bool`', 'view-model properties beyond `bool` and `number`')
append('meta/contracts/builder.md', '## Numeric view-model transition bindings', '''
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
''')
replace('ROADMAP.md', 'view-model properties other than `bool`', 'view-model properties beyond `bool` and `number`')
replace('ROADMAP.md', 'view-model properties beyond `bool` remain open', 'view-model properties beyond `bool` and `number` remain open')
replace('ROADMAP.md', 'animation exit-time gates added in #227 |', 'animation exit-time gates added in #227; numeric model bindings added in #237 / PR #238 |')
if '[#237]' not in Path('ROADMAP.md').read_text():
    replace('ROADMAP.md', 'Select further work from an explicit unblocked issue', '''[#237](https://github.com/George-RD/rive-rs-cli/issues/237), implemented in PR #238,
adds numeric model properties and named bound comparisons through the existing
compiler/builder path. Root charts and parallel regions share typed validation,
parameters, all six comparisons, source maps and atomic operations. Retained runtime
proof separates model mutation from input-only mutation and verifies threshold
crossing and reversal. Numeric property-name serialization and finite-value checks
are corrected. Model initialization remains the host's responsibility; conversions,
string/enum/trigger models and model-bound blends remain outside this slice.
The parent behavior todo stays open.

Select further work from an explicit unblocked issue''')
replace('meta/todos/todo.behavior-authoring-compiler.md', 'but `BehaviorPropertySpec` still has one variant, `bool`, so no numeric or enumerated conversion can be authored.', '`BehaviorPropertySpec` supports `bool` and `number`, but conversions and enumerated properties remain outside the typed frontend.')
replace('meta/todos/todo.behavior-authoring-compiler.md', 'view-model properties other than `bool`', 'view-model properties beyond `bool` and `number`')
replace('meta/todos/todo.behavior-authoring-compiler.md', 'non-boolean view-model properties.', 'model properties beyond boolean and number, and conversions.')
append('meta/todos/todo.behavior-authoring-compiler.md', '## Numeric view-model binding slice (#237)', '''
## Numeric view-model binding slice (#237)

PR #238 adds numeric model properties and `{binding, compare, value}` conditions.
Document-scoped scalar expressions, typed reference validation, chart-scoped binding
inputs, parallel-region lowering, deterministic source maps and atomic rejection
reuse the existing compiler/builder seams. The runtime proof required correcting
the numeric model property's inherited name field. The authored initial value
belongs to the synthesized input; host model instances require initialization and
binding. These comparisons are not input synchronization or model-bound blends.
The parent behavior todo remains open for its remaining capabilities.

The 15 public contracts cover operators, parameters, authored diagnostics, unused
properties, multi-model/multibyte indices, chart/region identity, determinism,
compatibility and canonical finite-value rejection. The runtime proof compiles
`number-binding.v0.json`, holds after input-only mutation, then crosses and reverses
at model values 59/60/90/59. Source, binary, PNGs, positions and hashes are retained.

[Numeric model binding evidence](../research/number-model-bindings.md) records
observed red/green runs and pinned format provenance. Final exact-head CI/MSRV and
separate Standards/Spec self-review are recorded in PR #238 before merge. Local
Cargo is unavailable; Rust and official-runtime execution use GitHub Actions.
''')
replace('CHANGELOG.md', '## [Unreleased]\n\n### Added\n', '''## [Unreleased]

### Added

- **Numeric view-model transition bindings.** Number model properties and `{binding, compare, value}` conditions use document-scoped scalar expressions and all six comparisons in root charts and parallel regions. Typed diagnostics, deterministic source maps, and atomic edits reuse the existing compiler. Canonical bindings reject invalid kinds and non-finite runtime values. Numeric properties now encode the inherited view-model name field, fixing runtime lookup. A retained example and official-runtime proof distinguish input-only mutation from model threshold crossing and reversal. Hosts must initialize and bind model instances; input synchronization, converters and model-bound blends remain outside this slice.
''')
append('meta/research/number-model-bindings.md', 'id: res.number-model-bindings', '''---
id: res.number-model-bindings
nodes:
  - rive-cli.intelligence.authoring
  - rive-cli.core.builder
  - rive-cli.core.objects
  - rive-cli.verification.rust
  - rive-cli.verification.browser
date: 2026-09-08
method: primary
---

# Numeric model bindings

Issue #237 / PR #238 continues the existing behavior todo under #175.
Fixed base: `a1f88b18b2f6f43fd2402c53581ba0bf43f1356c`.

## Observed verification

- Run `34251480151`, head `4eaa83a306d5153ad10db264ac7d3451a999693a`:
  the test compiled and failed because model kind number was unknown.
- Run `34251879833`, executed source `bdf7bffd6a349fae4399a5eb0c7301b853944afd`:
  the first contract, formatting, Clippy and full Rust suite passed. Downloaded
  artifact `10066439736` pins the executed source and clean working tree.
- Run `34252234877`, head `10d2ac01b3a8814328b6a2da8b24309667a9e841`:
  the original contract passed; four new invalid-kind/condition contracts failed.
  Artifact finalization separately failed with HTTP 403 in this and the first red
  run; job logs `102149214532` and `102146722769` retain the behavioral failures.
- Run `34253194379`: kind guards and full Rust checks passed, but runtime lookup
  failed for the numeric model property. Artifact `10066948039` retains the cause.
- Run `34253812557`, executed source `0b3a35d9b5eae062046a87122de4c0fc91341811`:
  thirteen contracts passed; name-field and canonical overflow contracts failed.
  Downloaded artifact `10067059056` was inspected before those fixes.
- Run `34254207250`, executed source `600052b77ae3f4eddd864ef7f0def4011154df12`:
  all fifteen contracts, formatting, all-target/all-feature Clippy, locked
  all-feature Rust tests and the numeric model runtime proof passed. Downloaded
  artifact `10067304424` retains source, logs, binary, six PNGs and evidence JSON.
  All six PNG hashes were checked. Panel centres are 39.5px initially, after
  input-only mutation and at model 59; 159.5px at model 60 and 90; 39.5px after
  reversal to 59. The synthesized input remains 90 during those model changes.

Final exact-head CI/MSRV and Standards/Spec review are recorded in PR #238; these
intermediate results are not a substitute for the final merge gate.

## Pinned runtime format provenance

Official `rive-app/rive-runtime` commit:
`25a2dc10786955df879ebc295e7c67923b8bde90`.

- `include/rive/generated/viewmodel/viewmodel_property_number_base.hpp`: type 431
  inherits ViewModelProperty/ViewModelComponent, not Component.
- `include/rive/generated/viewmodel/viewmodel_component_base.hpp`: name property
  557. Numeric properties previously wrote Component fields 4/5, leaving their
  names undiscoverable. Corrected emission follows the boolean property pattern.
  The legacy public parent_id field remains for API compatibility but is not
  serialized, as with the boolean type.
- `include/rive/generated/data_bind/bindable_property_number_base.hpp`: type 473,
  value field 636.
- `include/rive/generated/animation/transition_value_number_comparator_base.hpp`:
  type 484, value field 652.
- `src/animation/transition_viewmodel_condition.cpp`: model-backed numeric
  comparands. Existing object types and vendored runtime assets are reused.

The authored property value initializes the synthesized input only. The host
initializes and binds its model instance. This slice does not synchronize inputs,
add model-bound blends or conversions, or complete the broader behavior roadmap.
Local cloning failed DNS resolution and local Cargo is unavailable. Source review
and JavaScript syntax checks are local; Rust/browser execution is in Actions.
''')
subprocess.run(['git', 'add', '.github/workflows/ci.yml'], check=True)
