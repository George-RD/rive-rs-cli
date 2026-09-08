from pathlib import Path

if Path('meta/research/model-blend-authoring.md').exists():
    raise SystemExit(0)


def replace(path, before, after):
    file = Path(path)
    text = file.read_text()
    assert text.count(before) == 1, (path, before[:80])
    file.write_text(text.replace(before, after))


p = Path('tests/authoring_model_blend_contract.rs')
s = p.read_text()
start = s.index('\nfn lower(input:')
end = s.index('\n#[test]\nfn model_binding_sources', start)
helpers = s[start:end]
s = s[:start] + s[end:]
position = s.index('\n#[test]')
p.write_text(s[:position] + helpers + s[position:])

replace('cairn.blueprint', '''                "./tests/authoring_mirror_contract.rs",
''', '''                "./tests/authoring_mirror_contract.rs",
                "./tests/authoring_model_blend_contract.rs",
''')

p = 'docs/authoring-spec-v0.md'
replace(p, '''change the model condition. Typed blend inputs and listener actions still target
explicitly declared machine inputs. Converters, string/enum/trigger model properties,
model-bound blends, and listener writes to model properties are outside this slice.''', '''change the model condition. Direct blends may also select numeric bindings as
described below; one-dimensional blend inputs and listener actions still target
explicitly declared machine inputs. Converters, string/enum/trigger model properties,
model-bound one-dimensional blends, and listener writes to model properties remain
outside the typed subset.''')
replace(p, '''`direct_blend` gives each named motion its own local number input instead of selecting neighbouring stops from a shared input:''', '''`direct_blend` gives each named motion its own number input or numeric model binding instead of selecting neighbouring stops from a shared input:''')
replace(p, '''Binding IDs and raw-animation IDs are not substitutes for these references. Unknown fields, including runtime indices, are rejected.''', '''A binding ID is not an `input` alias: use the exclusive `binding` form below. Raw-animation IDs are not typed motion references. Unknown fields, including runtime indices, are rejected.''')
replace(p, '''Static expression weights, additive states and model-bound direct blends remain outside this slice.''', '''Static expression weights and additive states remain outside the typed subset.''')
replace(p, '''## Parallel regions
''', '''### Model-bound direct weights

A child may use `binding` instead of `input`:

```json
{
  "id": "blending",
  "direct_blend": {
    "motions": [
      { "motion": "rest-track", "input": "foundation" },
      { "motion": "left-track", "binding": "left-model" },
      { "motion": "right-track", "binding": "right-model" }
    ]
  }
}
```

Each binding names a numeric model property through `behavior.bindings`. Both source
fields, neither field, null sources and unknown fields are rejected. An unknown
binding reports `unknown_behavior_binding` at the child's `.binding`; a boolean
property reports `invalid_blend_binding` there. Invalid model/property references
keep their declaration paths. The same rules apply in parallel regions.

The compiler emits a binding even when only a blend uses it. Transition, root-state
and region uses share one synthesized input per binding per chart. Actual emitted
indices and source-map paths remain chart-local. The canonical builder emits the
native numeric bindable property and data-binding context before its direct-blend
consumer, using the existing model/property index resolver. It does not continuously
copy model values into machine inputs or simulate motion on the host.

`examples/authoring/model-blend-panel.v0.json` retains a full-weight rest motion and
two independently bound contributions. The host creates, initializes and binds the
model instance, then changes its `number(...)` properties. Resolve model/property
runtime names from the compile report, as in the numeric binding example above.
Changing a synthesized input, including with `render --input`, does not change a
model-controlled weight. The percentages, clamping and authored-order semantics are
the same as for input-driven direct blends.

Run `node tests/playwright/authoring-direct-blend-runtime.js --model-bound` for the
public-CLI/official-runtime proof. It retains evidence under
`target/playwright-behavior/model-blend`, separately from the existing input proof.
It checks model-only changes, input-only non-effects, partial/full weights,
independence, clamping, reversal and timed exit/resume. The normal typed-behavior CI
artifact retains both modes. Existing input-driven documents keep their bytes;
`authoring_format_version` stays 0.

## Parallel regions
''')
replace(p, '''Additive blend states, model-bound direct blends, advanced exit timing, and view-model properties beyond boolean and number remain outside the current typed subset''', '''Additive blend states, model-bound one-dimensional blends, advanced exit timing, and view-model properties beyond boolean and number remain outside the current typed subset''')

p = 'meta/contracts/authoring.md'
replace(p, '''A behavior state declares exactly one of `motion` and `blend`. Neither returns
`missing_state_motion` and both return `ambiguous_state_motion`, each at the state
path.''', '''A behavior state declares exactly one of `motion`, `blend`, and `direct_blend`.
None returns `missing_state_motion` and multiple sources return
`ambiguous_state_motion`, each at the state path.''')
replace(p, '''alongside a statechart. Additive blend states, model-bound direct blends, advanced exit timing, and
view-model properties beyond `bool` and `number` are not exposed by this frontend.''', '''alongside a statechart. Additive blend states, model-bound one-dimensional blends,
advanced exit timing, and view-model properties beyond `bool` and `number` are not
exposed by this frontend.''')
replace(p, '''Duration is supported; exit-time gates on direct sources are rejected. Model-bound
weights, static weight expressions and additive states are not exposed.''', '''Duration is supported; exit-time gates on direct sources are rejected. Static
weight expressions and additive states are not exposed.

## Model-bound direct weights (#241)

Each direct-blend child selects exactly one of `{motion, input}` and
`{motion, binding}`. Only a numeric model property is a valid bound weight. Unknown
binding and wrong-kind diagnostics use the child's authored `.binding` path;
model/property declaration errors retain their own paths. The strict typed union
and published schema reject ambiguous, missing, null and unknown fields.

Blend-only uses participate in the same used-binding collection as transitions.
Shared root/region/transition uses emit one bound input per chart, in authored
binding order. Canonical input indices include all preceding bound inputs; a shared
binding's source map retains every chart-local input path. Root and region states
reuse the same lowering and atomic-operation validation. Existing unbound documents
and source identities remain unchanged.

The canonical builder consumes bound number inputs as native direct-blend data
bindings. Model mutation controls the weight independently of the synthesized
machine input; initialization and binding of the model instance belong to the host.
No parallel compiler, host-side input mirroring or direct binary encoding is added.
The same browser harness verifies input-driven and model-bound modes separately.''')

p = 'meta/contracts/builder.md'
replace(p, '''This affects transition evaluation only; model instance initialization belongs to
the host. No input synchronization, model-bound blend, or conversion is promised.''', '''Model instance initialization belongs to the host. No input synchronization or
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
The canonical schema and existing binary object types are unchanged.''')

p = Path('examples/authoring/README.md')
p.write_text(p.read_text() + '''

## Model-bound direct blend panel

`model-blend-panel.v0.json` uses the same two-panel motions but takes both weights
from the `weights` model's numeric `left` and `right` properties. `foundation` stays
at 100. The host must create, initialize and bind the model instance, resolving
runtime names from the compile report's source map. Change the model properties,
not the synthesized machine inputs; those inputs are not kept in sync.

```sh
cargo run -- authoring compile examples/authoring/model-blend-panel.v0.json -o /tmp/model-blend-panel.riv --json
node tests/playwright/authoring-direct-blend-runtime.js --model-bound
```

The shared browser harness measures both panels while varying model weights and
holding the synthesized inputs at unrelated values. Source, compiled binary, compile
report, eleven PNGs, positions and hashes are retained separately under
`target/playwright-behavior/model-blend`. `reset` and `resume` retain the existing
100ms state transitions. No host-side motion calculations or input mirroring are
used to produce the rendered result.
''')

p = Path('ROADMAP.md')
s = p.read_text().replace('additive blend states, model-bound direct blends, advanced exit timing', 'additive blend states, model-bound one-dimensional blends, advanced exit timing')
s = s.replace('Additive blend states, model-bound direct blends, advanced exit timing', 'Additive blend states, model-bound one-dimensional blends, advanced exit timing')
s = s.replace('input-driven direct blends added in #239 |', 'input-driven direct blends added in #239; model-bound direct weights added in #241 / PR #242 |')
s = s.replace('## Priority order', '''[#241](https://github.com/George-RD/rive-rs-cli/issues/241), implemented in PR #242,
adds numeric model-bound direct weights through the same compiler and canonical
builder. Exclusive `{motion, binding}` children share emitted bindings with root,
region and transition consumers. Native data-bound objects, not synthesized-input
mirroring, control the weights. Retained public-CLI runtime evidence separates
model mutation from input-only mutation and covers independent contributions,
clamping, reversal and state transitions. Existing input-driven output is preserved.
The broader behavior todo remains open for its remaining capabilities.

## Priority order''')
p.write_text(s)

p = Path('meta/todos/todo.behavior-authoring-compiler.md')
p.write_text(p.read_text() + '''

## Model-bound direct blend slice (#241 / PR #242)

Direct children accept exclusive `{motion, binding}` beside the unchanged input
form. Numeric model bindings used only by blends are emitted; shared transition,
root and region uses are deduplicated per chart. Actual chart-local indices and
source-map paths are retained. Unknown bindings and boolean sources fail at the
child's authored path, while invalid model/property declarations retain theirs.

The canonical builder emits the native bindable-number/context/direct-child sequence
through existing object types and model index resolution. It does not synchronize
synthesized inputs or simulate animations. Explicit raw fixed sources are not
reinterpreted. The original input panel retains its prior compiled SHA-256.

Ten public contracts cover native encoding, exclusive source forms, reference/type
errors, region-only uses, shared consumers, multi-chart offsets, multibyte property
indices, fixed-source preservation, byte compatibility and atomic-edit rollback.
The existing direct-blend browser harness adds a model-bound mode rather than a
second copy of the harness. It retains model-only and input-only evidence, independent
weights, clamping, reversal and timed exit/resume for `model-blend-panel.v0.json`.

[Model blend evidence](../research/model-blend-authoring.md) records observed
red/green revisions and runtime provenance. Final exact-head CI/MSRV and separate
Standards/Spec self-review are recorded on PR #242. This parent remains open for
additive states, model-bound one-dimensional blends, broader timing, other property
kinds and conversions.
''')

Path('meta/research/model-blend-authoring.md').write_text('''---
id: res.model-blend-authoring
nodes:
  - rive-cli.intelligence.authoring
  - rive-cli.core.builder
  - rive-cli.verification.rust
  - rive-cli.verification.browser
date: 2026-09-09
method: primary
---

# Model-bound direct weights (#241)

## Reconciliation and native semantics

Main `40d9a5ee4e1b50819a44c3ff5bc49d611fdf44c8` contained merged #240 and no open
PRs. The remaining behavior todo explicitly called for model-bound direct blends.
#241 continues #175 after the completed number-binding and input-direct slices;
it does not reopen the independent low-level coverage programme.

The runtime header `rive-app/rive-runtime/include/rive/animation/blend_animation_direct.hpp`
defines direct sources input=0, fixed mix=1 and data bind=2. Its
`src/animation/blend_animation_direct.cpp` imports source 2 from the most recent
`BindablePropertyImporter`. These primary files were inspected through GitHub.
The implementation uses existing numeric bindable/context objects before the direct
child; pointing at the synthesized input alone would not bind a model weight.

The canonical number input's `view_model_binding` supplies the named model/property
path through the existing resolver. Explicit raw non-input sources are preserved.
Authoring uses an exclusive input/binding union and includes blend-only references
in used-binding discovery without a new lowering pass or index-repair mechanism.
The host initializes and binds the model instance; no input mirroring is promised.

## Observed test-first evidence

- Test-only source `21ec42762acc65a18098929dd856cff064d3812a`, run `34276198243`,
  job `102229687624`, compiled and failed because `binding` was an unknown direct
  child field. The contract also checks native encoded source 2 and bindable/context
  objects, so schema acceptance alone cannot satisfy it.
- The unchanged contract passed on source
  `b1d94ece335a3da50a5ec610ae0619e741f6e979`, run `34276456042`, together with
  all-target/all-feature Clippy.
- Expanded contracts and Clippy passed in run `34277015841`. They cover authored
  errors, shared/region-only references, scoped/multibyte indices, native ordering,
  explicit fixed-source preservation, old-byte compatibility and atomic rollback.

Local cloning and Rust execution were unavailable. Branch-scoped Actions performed
formatting, schema generation and executable verification; downloaded source/log
artifacts supported local inspection. Temporary workbench files are excluded from
the delivery diff. Final exact-head CI/MSRV and separate Standards/Spec self-review
are retained on PR #242; intermediate runs are not the final merge gate.

## Runtime evidence

`tests/playwright/authoring-direct-blend-runtime.js --model-bound` compiles the new
example through the public CLI and loads the bundled official runtime. It reuses the
existing input-driven harness and its pixel measurements instead of duplicating
that infrastructure. The model supplies independent left/right contributions after
a full-weight rest motion. Mutating synthesized inputs alone leaves both panels at
rest; model-only mutation changes their measured positions while those inputs remain
at unrelated 90/10 values.

The eleven cases cover initial binding, input-only mutation, zero/partial/full model
weights, independent contributions, reversal, out-of-range clamping, timed exit and
resume, and return to zero. Source, binary, compile report/source map, PNGs, positions,
runtime/WASM hashes and browser version are retained in
`target/playwright-behavior/model-blend`, under the normal typed-behavior CI artifact.
The original input mode remains separately verified. The parent behavior todo stays
open; one-dimensional model blends, additive states, static expression weights,
other property kinds, broader timing and conversions are not included.
''')
