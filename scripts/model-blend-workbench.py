from pathlib import Path

if 'pub enum BehaviorDirectBlendMotionSpec' in Path('src/authoring/spec.rs').read_text():
    raise SystemExit(0)


def replace(path, before, after):
    file = Path(path)
    text = file.read_text()
    assert text.count(before) == 1, (path, before[:80])
    file.write_text(text.replace(before, after))


replace('src/authoring/spec.rs', '''#[serde(deny_unknown_fields)]
pub struct BehaviorDirectBlendMotionSpec {
    pub motion: String,
    pub input: String,
}''', '''#[serde(untagged, deny_unknown_fields)]
pub enum BehaviorDirectBlendMotionSpec {
    Input { motion: String, input: String },
    Binding { motion: String, binding: String },
}

impl BehaviorDirectBlendMotionSpec {
    pub(crate) fn motion(&self) -> &str {
        match self {
            Self::Input { motion, .. } | Self::Binding { motion, .. } => motion,
        }
    }

    pub(crate) fn source_id(&self) -> &str {
        match self {
            Self::Input { input, .. } => input,
            Self::Binding { binding, .. } => binding,
        }
    }

    pub(crate) fn binding(&self) -> Option<&str> {
        match self {
            Self::Input { .. } => None,
            Self::Binding { binding, .. } => Some(binding),
        }
    }
}''')

p = 'src/authoring/frontend/compiler/behavior.rs'
replace(p, '''            .filter_map(|transition| transition.when.binding().map(|(id, _)| id))
            .collect::<HashSet<_>>();''', '''            .filter_map(|transition| transition.when.binding().map(|(id, _)| id))
            .chain(
                statechart.states.iter()
                    .chain(statechart.regions.iter().flat_map(|region| &region.states))
                    .filter_map(|state| state.direct_blend.as_ref())
                    .flat_map(|blend| &blend.motions)
                    .filter_map(|motion| motion.binding()),
            )
            .collect::<HashSet<_>>();''')
replace(p, '''        let mut input_name_by_binding = HashMap::new();
        let mut inputs = Vec::new();''', '''        let mut input_name_by_binding = HashMap::new();
        let mut input_index_by_id = HashMap::new();
        let mut inputs = Vec::new();''')
replace(p, '''            input_name_by_binding.insert(binding.id.as_str(), input_name.clone());''', '''            input_name_by_binding.insert(binding.id.as_str(), input_name.clone());
            input_index_by_id.insert(binding.id.as_str(), input_index);''')
replace(p, '''        let mut input_name_by_id = HashMap::new();
        let mut input_index_by_id = HashMap::new();''', '''        let mut input_name_by_id = HashMap::new();''')
replace(p, '''animation_runtime_name(spec, &motion.motion)''', '''animation_runtime_name(spec, motion.motion())''')
replace(p, '''input_index_by_id.get(motion.input.as_str())''', '''input_index_by_id.get(motion.source_id())''')
replace(p, '''            statechart_id,
            parameters,
            motion_tracks,
            inputs,
            diagnostics,
        );
    }
    if !states.contains(initial)''', '''            StateMotionContext {
                statechart_id,
                parameters,
                motion_tracks,
                inputs,
                bindings,
            },
            diagnostics,
        );
    }
    if !states.contains(initial)''')
replace(p, '''fn validate_state_motion(
    state: &BehaviorStateSpec,
    state_path: &str,
    statechart_id: &str,
    parameters: &BTreeMap<String, Quantity>,
    motion_tracks: &HashSet<&str>,
    inputs: &HashMap<&str, &BehaviorInputSpec>,
    diagnostics: &mut Vec<AuthoringDiagnostic>,
) {
    if let Some(blend) = &state.direct_blend {''', '''struct StateMotionContext<'a> {
    statechart_id: &'a str,
    parameters: &'a BTreeMap<String, Quantity>,
    motion_tracks: &'a HashSet<&'a str>,
    inputs: &'a HashMap<&'a str, &'a BehaviorInputSpec>,
    bindings: &'a HashMap<&'a str, Option<BehaviorInputKind>>,
}

fn validate_state_motion(
    state: &BehaviorStateSpec,
    state_path: &str,
    context: StateMotionContext<'_>,
    diagnostics: &mut Vec<AuthoringDiagnostic>,
) {
    let StateMotionContext {
        statechart_id, parameters, motion_tracks, inputs, bindings,
    } = context;
    if let Some(blend) = &state.direct_blend {''')
replace(p, '''            if !motion_tracks.contains(motion.motion.as_str()) {
                diagnostics.push(AuthoringDiagnostic::new(
                    format!("{path}.motion"),
                    "unknown_behavior_motion",
                    format!("motion track '{}' is not defined", motion.motion),
                ));
            }
            match inputs.get(motion.input.as_str()) {''', '''            if !motion_tracks.contains(motion.motion()) {
                diagnostics.push(AuthoringDiagnostic::new(
                    format!("{path}.motion"),
                    "unknown_behavior_motion",
                    format!("motion track '{}' is not defined", motion.motion()),
                ));
            }
            if let Some(binding) = motion.binding() {
                match bindings.get(binding) {
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
                                "direct blend binding '{binding}' must reference a number property but references {}",
                                actual.as_str()
                            ),
                        ));
                    }
                    Some(_) => {}
                }
                continue;
            }
            match inputs.get(motion.source_id()) {''')
replace(p, '''                        motion.input
''', '''                        motion.source_id()
''')
replace(p, '''                            motion.input,
''', '''                            motion.source_id(),
''')

p = 'src/builder/state_machines.rs'
replace(p, '''fn encode_id_path(ids: &[u64]) -> Vec<u8> {''', '''const DIRECT_BLEND_SOURCE_INPUT: u64 = 0;
const DIRECT_BLEND_SOURCE_DATA_BIND: u64 = 2;

fn encode_id_path(ids: &[u64]) -> Vec<u8> {''')
replace(p, '''                                append_blend_state_direct_child(child, objects);''', '''                                append_blend_state_direct_child(
                                    child,
                                    state_machine.inputs.as_deref().unwrap_or_default(),
                                    &bound_number_input_paths,
                                    objects,
                                );''')
replace(p, '''fn append_blend_state_direct_child(
    spec: &BlendStateDirectChildSpec,
    objects: &mut Vec<Box<dyn RiveObject>>,
) {''', '''fn append_blend_state_direct_child(
    spec: &BlendStateDirectChildSpec,
    inputs: &[InputSpec],
    bound_number_input_paths: &HashMap<String, (u64, u64)>,
    objects: &mut Vec<Box<dyn RiveObject>>,
) {''')
replace(p, '''    objects.push(Box::new(BlendAnimationDirect {
        animation_id: *animation_id,
        input_id: input_id.unwrap_or(u32::MAX as u64),
        mix_value: mix_value.unwrap_or(100.0),
        blend_source: blend_source.unwrap_or(0),
    }));''', '''    let binding = input_id
        .and_then(|index| usize::try_from(index).ok())
        .and_then(|index| inputs.get(index))
        .and_then(|input| match input {
            InputSpec::Number { name, value, .. } => bound_number_input_paths
                .get(name)
                .map(|ids| (*ids, *value)),
            _ => None,
        });
    let mut input_id = input_id.unwrap_or(u32::MAX as u64);
    let mut blend_source = blend_source.unwrap_or(DIRECT_BLEND_SOURCE_INPUT);
    if blend_source == DIRECT_BLEND_SOURCE_INPUT
        && let Some(((view_model_id, property_id), value)) = binding
    {
        objects.push(Box::new(BindablePropertyNumber { property_value: value }));
        objects.push(Box::new(DataBindContext::new(
            property_keys::BINDABLE_PROPERTY_NUMBER_VALUE as u64,
            0,
            encode_id_path(&[view_model_id, property_id]),
        )));
        input_id = u32::MAX as u64;
        blend_source = DIRECT_BLEND_SOURCE_DATA_BIND;
    }
    objects.push(Box::new(BlendAnimationDirect {
        animation_id: *animation_id,
        input_id,
        mix_value: mix_value.unwrap_or(100.0),
        blend_source,
    }));''')
