import json
from pathlib import Path


def replace(path, old, new):
    file = Path(path)
    text = file.read_text()
    if new in text:
        return
    assert text.count(old) == 1, (path, text.count(old))
    file.write_text(text.replace(old, new))


p = Path('src/authoring/frontend/compiler/behavior.rs')
s = p.read_text()
if "impl StateMotionContext<'_>" not in s:
    start = s.index('    let StateMotionContext {', s.index('fn validate_state_motion'))
    end = s.index('    if let Some(blend) = &state.direct_blend {', start)
    s = s[:start] + '''    let parameters = context.parameters;
    let motion_tracks = context.motion_tracks;
''' + s[end:]
    start = s.index('            if let Some(binding) = motion.binding() {', s.index('fn validate_state_motion'))
    end = s.index('        }\n        return;', start)
    s = s[:start] + '''            context.validate_blend_source(motion.source_id(), motion.binding(), &path, diagnostics);
''' + s[end:]
    start = s.index('            if let Some(binding) = blend.binding() {', s.index('fn validate_state_motion'))
    end = s.index('            if !(BLEND_STOP_MINIMUM', start)
    s = s[:start] + '''            context.validate_blend_source(
                blend.source_id(),
                blend.binding(),
                &format!("{state_path}.blend"),
                diagnostics,
            );
''' + s[end:]
    helper = '''impl StateMotionContext<'_> {
    fn validate_blend_source(
        &self,
        source: &str,
        binding: Option<&str>,
        path: &str,
        diagnostics: &mut Vec<AuthoringDiagnostic>,
    ) {
        if let Some(binding) = binding {
            match self.bindings.get(binding) {
                None => diagnostics.push(AuthoringDiagnostic::new(
                    format!("{path}.binding"),
                    "unknown_behavior_binding",
                    format!("behavior binding '{binding}' is not defined"),
                )),
                Some(Some(actual)) if *actual != BehaviorInputKind::Number => {
                    diagnostics.push(AuthoringDiagnostic::new(
                        format!("{path}.binding"),
                        "invalid_blend_binding",
                        format!(
                            "blend binding '{binding}' must reference a number property but references {}",
                            actual.as_str()
                        ),
                    ));
                }
                Some(_) => {}
            }
            return;
        }
        match self.inputs.get(source) {
            None => diagnostics.push(AuthoringDiagnostic::new(
                format!("{path}.input"),
                "unknown_behavior_input",
                format!(
                    "behavior input '{source}' is not defined in statechart '{}'",
                    self.statechart_id
                ),
            )),
            Some(input) if input.kind() != BehaviorInputKind::Number => {
                diagnostics.push(AuthoringDiagnostic::new(
                    format!("{path}.input"),
                    "invalid_blend_input",
                    format!(
                        "blend input '{source}' must be a number input but is declared as {}",
                        input.kind().as_str()
                    ),
                ));
            }
            Some(_) => {}
        }
    }
}

'''
    s = s.replace('fn validate_state_motion(\n', helper + 'fn validate_state_motion(\n')
    start = s.index('                    .filter_map(|state| state.direct_blend.as_ref())')
    end = s.index('            .collect::<HashSet<_>>();', start)
    s = s[:start] + '''                    .flat_map(|state| {
                        state
                            .blend
                            .as_ref()
                            .and_then(|blend| blend.binding())
                            .into_iter()
                            .chain(
                                state
                                    .direct_blend
                                    .iter()
                                    .flat_map(|blend| &blend.motions)
                                    .filter_map(|motion| motion.binding()),
                            )
                    }),
            )
''' + s[end:]
    p.write_text(s)

path = 'tests/authoring_model_blend_1d_contract.rs'
replace(path, '''    assert_eq!(lowered, lower(&input));
    compile(&lowered.scene);
}''', '''    assert_eq!(lowered, lower(&input));
    let parsed = parse_riv(&compile(&lowered.scene), &InspectFilter::default()).expect("binary");
    assert_eq!(
        parsed.objects.iter().filter(|object| object.type_key == type_keys::BLEND_STATE_1D_VIEW_MODEL).count(),
        2
    );
}''')

path = 'docs/authoring-spec-v0.md'
replace(path, '''described below; one-dimensional blend inputs and listener actions still target
explicitly declared machine inputs. Converters, string/enum/trigger model properties,
model-bound one-dimensional blends, and listener writes to model properties remain
outside the typed subset.''', '''described below, as may one-dimensional blends. Listener actions still target
explicitly declared machine inputs. Converters, string/enum/trigger model properties,
and listener writes to model properties remain outside the typed subset.''')
replace(path, '`blend` maps a number input onto at least two motion tracks,', '`blend` maps either a number input or numeric model binding onto at least two motion tracks,')
replace(path, '## Direct blend states\n', '''### Model-bound one-dimensional blends

Replace `input` with `binding` to drive the same ordered stops from a numeric
model property:

```json
{
  "id": "reading",
  "blend": {
    "binding": "model-load",
    "stops": [
      { "motion": "calm-track", "value": { "kind": "literal", "value": 0, "unit": "scalar" } },
      { "motion": "surge-track", "value": { "kind": "literal", "value": 100, "unit": "scalar" } }
    ]
  }
}
```

Exactly one source is required. Neither, both, null sources and runtime indices are
rejected by the strict source union. `unknown_behavior_binding` and
`invalid_blend_binding` report an absent binding or non-number model property at
`.blend.binding`; invalid model/property declarations retain their own paths.
The same 2..=1000 stop, scalar expression and emitted-float ordering checks apply.
Stop values are thresholds, not percentage weights; 0..100 is only this example's
range. Values outside the outer stops select the nearest endpoint.

A binding used only by a blend is still emitted. Root states, parallel regions,
direct blends and transition conditions share chart-scoped binding resolution and
source-map entries. The canonical scene remains a named `blend_state_1d` input
reference with `view_model_binding`. The builder emits a native
`BlendState1DViewModel` with its own preceding bindable number/context, rather than
an input-driven state. No second compiler pass or host animation simulation is used.

The host creates, initializes and binds the model instance as shown in the numeric
binding example. Changing its number property drives the blend; changing only the
synthesized input, including through `render --input`, does not. The authored model
value initializes the synthetic input, not a serialized default model instance.

`examples/authoring/model-blend-1d-panel.v0.json` provides two independent three-stop
blends, one in a parallel region. Run
`node tests/playwright/authoring-direct-blend-runtime.js --model-bound --one-dimensional`
for public-CLI/bundled-runtime verification. Evidence is retained under
`target/playwright-behavior/model-blend-1d`: exact stop positions, intermediate
values, clamping, reversal, input-only non-effects and timed exit/resume after a
model change while inactive. The normal typed-behavior CI artifact retains this
mode alongside the direct blend proofs. Existing input-only documents preserve
their JSON and bytes; `authoring_format_version` remains 0.

## Direct blend states
''')
replace(path, 'Additive blend states, model-bound one-dimensional blends, advanced exit timing,', 'Additive blend states, advanced exit timing,')

path = 'meta/contracts/authoring.md'
replace(path, '''"value": <scalar expression>}, ...]}` and lowers to a `blend_state_1d` whose children
are `blend_animation_1d` entries naming the lowered animations.''', '''"value": <scalar expression>}, ...]}` or the same stops with a numeric `binding`
instead of `input`. It lowers to a `blend_state_1d` whose children are
`blend_animation_1d` entries naming the lowered animations.''')
replace(path, '''alongside a statechart. Additive blend states, model-bound one-dimensional blends,
advanced exit timing,''', '''alongside a statechart. Additive blend states, advanced exit timing,''')
p = Path(path)
if '## Numeric model-bound one-dimensional blends (#243)' not in p.read_text():
    p.write_text(p.read_text() + '''
## Numeric model-bound one-dimensional blends (#243)

`blend` has exactly one source: a chart-local number `input` or a document numeric
`binding`. Both/neither/null forms and runtime fields fail strict deserialization.
Unknown bindings and non-number properties report `unknown_behavior_binding` and
`invalid_blend_binding` at `.blend.binding`. Invalid model/property declarations
retain their existing authored paths. Stops retain scalar expressions, 2..=1000
cardinality and strict emitted-f32 ordering, including in parallel regions.

Blend-only uses participate in shared binding discovery. Root/region one-dimensional
and direct blends and transitions deduplicate bindings per chart, retain actual
input offsets, and map to the existing compiler-owned scene without a second pass.
The canonical named-input reference carries a view-model binding; the builder emits
native model-bound state objects, not synthetic-input synchronization. Each chart
resolves independently. Model initialization and instance binding remain host work.
Model changes affect active blends and are observed on re-entry after inactive
changes; input-only writes do not drive them. Existing input-only bytes remain stable.

Public contracts cover source exclusivity, validation paths, float stop ordering,
region-only discovery, shared/multi-chart uses, nonzero/multibyte indices,
deterministic maps/bytes, native encoding and atomic-edit rejection. The three-stop
panel example and existing browser harness's one-dimensional mode retain CLI/source,
binary, measured renders and hashes in the typed-behavior runtime artifact.
''')
p = Path('meta/contracts/builder.md')
if '## Numeric view-model one-dimensional blend bindings' not in p.read_text():
    p.write_text(p.read_text() + '''
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
''')
path = 'ROADMAP.md'
replace(path, 'without any input. Additive blend states, model-bound one-dimensional blends, advanced exit timing,', 'without any input. Additive blend states, advanced exit timing,')
replace(path, '## Priority order\n', '''[#243](https://github.com/George-RD/rive-rs-cli/issues/243) extends the same numeric
binding path to one-dimensional blends. Named `blend` states choose an exclusive
input or binding source while retaining ordered scalar stops. Root states and
parallel regions use native model-bound runtime states without host input mirroring.
The three-stop panel and retained public-CLI/runtime contract exercise intermediate
values, exact stops, clamping, reversal and timed exit/resume after inactive model
changes. The broader behavior todo remains open for its other capabilities.

## Priority order
''')
replace(path, 'model-bound direct weights added in #241 / PR #242 |', 'model-bound direct weights added in #241 / PR #242; model-bound one-dimensional blends added in #243 |')
replace(path, 'exactly; additive blend states, model-bound one-dimensional blends, advanced exit timing,', 'exactly; additive blend states, advanced exit timing,')

p = Path('meta/todos/todo.behavior-authoring-compiler.md')
if '## Model-bound one-dimensional blend slice (#243)' not in p.read_text():
    p.write_text(p.read_text() + '''
## Model-bound one-dimensional blend slice (#243)

One-dimensional `blend` states accept an exclusive numeric `binding` source beside
the existing number `input` form. The same ordered scalar stops, reference/kind
validation and chart-scoped binding discovery apply to root and parallel regions.
Bindings are shared with direct blends and conditions without duplicate input slots.
The canonical builder emits native view-model states with per-consumer bindable
properties and contexts, using the existing numeric resolver and direct-blend helper.
Unbound input output and canonical schema remain unchanged. Hosts create, initialize
and bind model instances; model changes are not copied to synthetic inputs.

Thirteen public contracts cover deterministic lowering/encoding, strict source
forms, authored errors, stop limits and f32 order, region-only discovery, shared
consumers, chart offsets, multibyte property indices, legacy input states and atomic
rollback. `model-blend-1d-panel.v0.json` and the existing browser harness cover exact
stops, between-stop movement, independence, clamping, reversal, input-only non-effects,
and timed exit/resume after model changes while inactive. CI retains all three blend
modes independently, without replacing existing runtime gates.

[One-dimensional model blend evidence](../research/model-blend-1d-authoring.md)
records test-first failures, native object provenance and verification. Final
exact-head CI/MSRV and separate Standards/Spec self-review belong on the PR before
merge. The parent remains open for additive states, broader timing, other model
property kinds and conversions; this is a bounded continuation, not a new track.
''')
p = Path('examples/authoring/model-blend-1d-panel.v0.json')
source = json.loads(p.read_text())
source['artboard']['id'] = 'model-blend-1d-panel'
p.write_text(json.dumps(source, indent=2) + '\n')

p = Path('meta/research/model-blend-1d-authoring.md')
if not p.exists():
    p.write_text('''---
id: res.model-blend-1d-authoring
nodes:
  - rive-cli.intelligence.authoring
  - rive-cli.core.builder
  - rive-cli.verification.rust
  - rive-cli.verification.browser
date: 2026-09-09
method: primary
---

# Numeric model-bound one-dimensional blends (#243)

## Reconciliation and native semantics

Main `74a88084b456318a1192e762751e094766c3e67a` contained merged PR #242 and no
open PRs. The remaining behavior todo explicitly listed model-bound one-dimensional
blends. #243 continues that acceptance after numeric properties/conditions and
model-bound direct weights, not a new roadmap track or low-level coverage programme.

Primary runtime source `rive-app/rive-runtime/src/animation/blend_state_1d_viewmodel.cpp`
imports the most recent `BindablePropertyImporter` and owns that property. Each
consumer therefore receives its own preceding bindable-number/context pair before
the native `BlendState1DViewModel`; reusing the synthetic input is insufficient.
Model/property path resolution and bindable emission reuse the existing canonical
builder. Authoring retains the existing named-input SceneSpec shape and strict
source union. Binding discovery and numeric source validation are shared with direct
blends. Hosts create, initialize and bind model instances; there is no input mirror.

## Observed test-first evidence

- Test-only commit `2f442f9540ee62a3f7359b4267a376660dfb12a7`, run `34306951457`,
  job `102325514248`, compiled and failed because `blend.binding` was unknown.
  The initial canonical-index assertion was corrected to the existing named-input
  representation before the green; the observed schema failure was unchanged.
- Run `34307286816`, job `102326512108`, passed the lowering contract on exported
  source `6e56dfa4a22e9147502afbc65f7a0d3a68152206`.
- The native-object contract first had a test-only byte-inspection type error. After
  correcting it to the public parser's byte-length representation, run `34307461860`,
  job `102327066340`, compiled and failed because no native view-model state existed.
  Lowering alone passed, proving that accepting the new source was insufficient.
- Run `34307566947` passed both public contracts after native-state emission and
  shared direct/one-dimensional bindable-number construction. Workflow triggering
  heads differ from the bot-published tested source; artifacts retain source heads.

Local cloning and Cargo execution were unavailable. Scoped GitHub Actions execute
Rust, schema generation and bundled-runtime proofs; exported sources and artifacts
support local inspection. A workbench push of `ci.yml` failed at the normal workflow
permission boundary, so the CI addition was made through the authorized connector.
No CI gates were removed or relaxed. Temporary workbench files are not delivery code.

## Verification contract

Thirteen public tests cover exclusive sources, authored reference/kind errors,
scalar parameters, stop bounds/f32 order, region-only binding discovery, shared
one-dimensional/direct/transition consumers, multiple charts with different input
offsets, nonzero model/multibyte property paths, native object ordering, legacy
input encoding, deterministic example output and atomic operation rollback.

The three-stop panel uses thresholds 0, 50 and 100 at x=40,120,200. The existing
browser harness adds a separate model-bound one-dimensional mode, preserving both
direct modes. It tests exact stops and interior ranges, not a false arithmetic-
midpoint promise for sequential neighboring-animation mixing. It also tests model-
only mutation, input-only non-effects, independence, clamping, reversal, timed
exit/resume and updates while inactive. JSON, binary, PNGs, positions and runtime/
WASM hashes are retained in the typed-behavior CI artifact. Final exact-head CI/MSRV
and separate Standards/Spec self-review are recorded on the delivery PR before merge.
''')
