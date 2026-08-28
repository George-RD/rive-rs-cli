use std::collections::{HashMap, HashSet};

use serde_json::{Value, json};

use crate::builder::{SceneSpec, build_scene};

use super::super::super::lower::runtime_name;
use super::super::super::spec::{
    AuthoringDiagnostic, AuthoringError, AuthoringSourceMap, AuthoringSpec, BehaviorBindingSpec,
    BehaviorModelSpec, BehaviorPropertySpec, SourceMapEntry,
};

pub(super) struct BehaviorLoweringOutput {
    pub(super) state_machines: Vec<Value>,
    pub(super) source_entries: Vec<SourceMapEntry>,
}

pub(super) fn lower_behavior(
    spec: &AuthoringSpec,
    state_machine_index_base: usize,
) -> Result<BehaviorLoweringOutput, AuthoringError> {
    let diagnostics = validate_behavior(spec);
    if !diagnostics.is_empty() {
        return Err(AuthoringError::many(diagnostics));
    }

    let mut state_machines = Vec::with_capacity(spec.behavior.statecharts.len());
    let mut source_entries = Vec::new();
    let mut binding_runtime: HashMap<usize, (Vec<String>, Vec<String>)> = HashMap::new();

    let bindings_by_id = spec
        .behavior
        .bindings
        .iter()
        .enumerate()
        .map(|(index, binding)| (binding.id.as_str(), index))
        .collect::<HashMap<_, _>>();

    for (statechart_index, statechart) in spec.behavior.statecharts.iter().enumerate() {
        let scene_machine_index = state_machine_index_base + statechart_index;
        let machine_path = format!("/artboard/state_machines/{scene_machine_index}");
        let machine_name = runtime_name(
            &[spec.artboard.id.clone(), statechart.id.clone()],
            "state_machine",
        );

        let used_bindings = statechart
            .transitions
            .iter()
            .map(|transition| transition.when.binding.as_str())
            .collect::<HashSet<_>>();
        let mut input_name_by_binding = HashMap::new();
        let mut inputs = Vec::new();
        for (binding_index, binding) in spec.behavior.bindings.iter().enumerate() {
            if !used_bindings.contains(binding.id.as_str()) {
                continue;
            }
            let input_index = inputs.len();
            let input_name = runtime_name(
                &[
                    spec.artboard.id.clone(),
                    statechart.id.clone(),
                    binding.id.clone(),
                ],
                "input",
            );
            input_name_by_binding.insert(binding.id.as_str(), input_name.clone());
            inputs.push(json!({
                "type": "bool",
                "name": input_name,
                "value": binding_bool_value(spec, binding)
            }));

            let runtime = binding_runtime.entry(binding_index).or_default();
            runtime.0.push(input_name);
            runtime
                .1
                .push(format!("{machine_path}/inputs/{input_index}"));
        }

        let state_index_by_id = statechart
            .states
            .iter()
            .enumerate()
            .map(|(index, state)| (state.id.as_str(), index + 1))
            .collect::<HashMap<_, _>>();

        let mut states = vec![json!({ "type": "entry" })];
        for (state_index, state) in statechart.states.iter().enumerate() {
            let animation_name = runtime_name(
                &[spec.artboard.id.clone(), state.motion.clone()],
                "animation",
            );
            states.push(json!({
                "type": "animation",
                "animation": animation_name
            }));
            source_entries.push(SourceMapEntry {
                authored_id: format!("{}/{}", statechart.id, state.id),
                authored_path: format!(
                    "$.behavior.statecharts[{statechart_index}].states[{state_index}]"
                ),
                definition_path: None,
                runtime_names: Vec::new(),
                scene_paths: vec![format!(
                    "{machine_path}/layers/0/states/{}",
                    state_index + 1
                )],
            });
        }

        let initial_state = *state_index_by_id
            .get(statechart.initial.as_str())
            .expect("validated initial state");
        let mut transitions = vec![json!({ "from": 0, "to": initial_state })];
        for (transition_index, transition) in statechart.transitions.iter().enumerate() {
            let from = *state_index_by_id
                .get(transition.from.as_str())
                .expect("validated transition source");
            let to = *state_index_by_id
                .get(transition.to.as_str())
                .expect("validated transition target");
            let input = input_name_by_binding
                .get(transition.when.binding.as_str())
                .expect("validated transition binding");
            transitions.push(json!({
                "from": from,
                "to": to,
                "conditions": [
                    {
                        "input": input,
                        "value": transition.when.equals
                    }
                ]
            }));
            source_entries.push(SourceMapEntry {
                authored_id: format!("{}/{}", statechart.id, transition.id),
                authored_path: format!(
                    "$.behavior.statecharts[{statechart_index}].transitions[{transition_index}]"
                ),
                definition_path: None,
                runtime_names: Vec::new(),
                scene_paths: vec![format!(
                    "{machine_path}/layers/0/transitions/{}",
                    transition_index + 1
                )],
            });
        }

        source_entries.push(SourceMapEntry {
            authored_id: statechart.id.clone(),
            authored_path: format!("$.behavior.statecharts[{statechart_index}]"),
            definition_path: None,
            runtime_names: vec![machine_name.clone()],
            scene_paths: vec![machine_path.clone()],
        });

        state_machines.push(json!({
            "name": machine_name,
            "inputs": inputs,
            "layers": [
                {
                    "states": states,
                    "transitions": transitions
                }
            ]
        }));
    }

    for (binding_index, (runtime_names, scene_paths)) in binding_runtime {
        let binding = &spec.behavior.bindings[binding_index];
        source_entries.push(SourceMapEntry {
            authored_id: binding.id.clone(),
            authored_path: format!("$.behavior.bindings[{binding_index}]"),
            definition_path: None,
            runtime_names,
            scene_paths,
        });
    }

    source_entries.sort_by(|left, right| left.authored_path.cmp(&right.authored_path));

    Ok(BehaviorLoweringOutput {
        state_machines,
        source_entries,
    })
}

pub(super) fn validate_lowered_scene(scene: &Value) -> Result<(), AuthoringError> {
    let scene: SceneSpec = serde_json::from_value(scene.clone()).map_err(|error| {
        AuthoringError::one(AuthoringDiagnostic::new(
            "$.behavior",
            "invalid_lowered_behavior",
            format!("typed behavior produced invalid canonical SceneSpec: {error}"),
        ))
    })?;
    build_scene(&scene, None).map_err(|error| {
        AuthoringError::one(AuthoringDiagnostic::new(
            "$.behavior",
            "invalid_lowered_behavior",
            format!("typed behavior failed canonical SceneSpec validation: {error}"),
        ))
    })?;
    Ok(())
}

fn validate_behavior(spec: &AuthoringSpec) -> Vec<AuthoringDiagnostic> {
    let mut diagnostics = Vec::new();
    let mut models = HashMap::new();
    for (model_index, model) in spec.behavior.models.iter().enumerate() {
        let model_path = format!("$.behavior.models[{model_index}]");
        validate_id(&model.id, &format!("{model_path}.id"), &mut diagnostics);
        if models.insert(model.id.as_str(), model).is_some() {
            diagnostics.push(AuthoringDiagnostic::new(
                format!("{model_path}.id"),
                "duplicate_behavior_model",
                format!("behavior model id '{}' is duplicated", model.id),
            ));
        }
        let mut properties = HashSet::new();
        for (property_index, property) in model.properties.iter().enumerate() {
            let property_path = format!("{model_path}.properties[{property_index}]");
            validate_id(
                property.id(),
                &format!("{property_path}.id"),
                &mut diagnostics,
            );
            if !properties.insert(property.id()) {
                diagnostics.push(AuthoringDiagnostic::new(
                    format!("{property_path}.id"),
                    "duplicate_behavior_property",
                    format!(
                        "behavior property id '{}' is duplicated in model '{}'",
                        property.id(), model.id
                    ),
                ));
            }
        }
    }

    let mut bindings = HashMap::new();
    for (binding_index, binding) in spec.behavior.bindings.iter().enumerate() {
        let binding_path = format!("$.behavior.bindings[{binding_index}]");
        validate_id(
            &binding.id,
            &format!("{binding_path}.id"),
            &mut diagnostics,
        );
        if bindings.insert(binding.id.as_str(), binding).is_some() {
            diagnostics.push(AuthoringDiagnostic::new(
                format!("{binding_path}.id"),
                "duplicate_behavior_binding",
                format!("behavior binding id '{}' is duplicated", binding.id),
            ));
        }
        match models.get(binding.model.as_str()) {
            None => diagnostics.push(AuthoringDiagnostic::new(
                format!("{binding_path}.model"),
                "unknown_behavior_model",
                format!("behavior model '{}' is not defined", binding.model),
            )),
            Some(model) if find_property(model, &binding.property).is_none() => {
                diagnostics.push(AuthoringDiagnostic::new(
                    format!("{binding_path}.property"),
                    "unknown_behavior_property",
                    format!(
                        "behavior property '{}.{}' is not defined",
                        binding.model, binding.property
                    ),
                ));
            }
            Some(_) => {}
        }
    }

    let motion_tracks = spec
        .motion
        .tracks
        .iter()
        .map(|track| track.id.as_str())
        .collect::<HashSet<_>>();
    let mut statecharts = HashSet::new();
    for (statechart_index, statechart) in spec.behavior.statecharts.iter().enumerate() {
        let statechart_path = format!("$.behavior.statecharts[{statechart_index}]");
        validate_id(
            &statechart.id,
            &format!("{statechart_path}.id"),
            &mut diagnostics,
        );
        if !statecharts.insert(statechart.id.as_str()) {
            diagnostics.push(AuthoringDiagnostic::new(
                format!("{statechart_path}.id"),
                "duplicate_behavior_statechart",
                format!("behavior statechart id '{}' is duplicated", statechart.id),
            ));
        }

        let mut states = HashSet::new();
        for (state_index, state) in statechart.states.iter().enumerate() {
            let state_path = format!("{statechart_path}.states[{state_index}]");
            validate_id(&state.id, &format!("{state_path}.id"), &mut diagnostics);
            if !states.insert(state.id.as_str()) {
                diagnostics.push(AuthoringDiagnostic::new(
                    format!("{state_path}.id"),
                    "duplicate_behavior_state",
                    format!(
                        "behavior state id '{}' is duplicated in statechart '{}'",
                        state.id, statechart.id
                    ),
                ));
            }
            if !motion_tracks.contains(state.motion.as_str()) {
                diagnostics.push(AuthoringDiagnostic::new(
                    format!("{state_path}.motion"),
                    "unknown_behavior_motion",
                    format!("motion track '{}' is not defined", state.motion),
                ));
            }
        }
        if !states.contains(statechart.initial.as_str()) {
            diagnostics.push(AuthoringDiagnostic::new(
                format!("{statechart_path}.initial"),
                "unknown_behavior_state",
                format!(
                    "initial state '{}' is not defined in statechart '{}'",
                    statechart.initial, statechart.id
                ),
            ));
        }

        let mut transitions = HashSet::new();
        for (transition_index, transition) in statechart.transitions.iter().enumerate() {
            let transition_path = format!("{statechart_path}.transitions[{transition_index}]");
            validate_id(
                &transition.id,
                &format!("{transition_path}.id"),
                &mut diagnostics,
            );
            if !transitions.insert(transition.id.as_str()) {
                diagnostics.push(AuthoringDiagnostic::new(
                    format!("{transition_path}.id"),
                    "duplicate_behavior_transition",
                    format!(
                        "behavior transition id '{}' is duplicated in statechart '{}'",
                        transition.id, statechart.id
                    ),
                ));
            }
            for (field, state) in [("from", &transition.from), ("to", &transition.to)] {
                if !states.contains(state.as_str()) {
                    diagnostics.push(AuthoringDiagnostic::new(
                        format!("{transition_path}.{field}"),
                        "unknown_behavior_state",
                        format!(
                            "state '{}' is not defined in statechart '{}'",
                            state, statechart.id
                        ),
                    ));
                }
            }
            if !bindings.contains_key(transition.when.binding.as_str()) {
                diagnostics.push(AuthoringDiagnostic::new(
                    format!("{transition_path}.when.binding"),
                    "unknown_behavior_binding",
                    format!(
                        "behavior binding '{}' is not defined",
                        transition.when.binding
                    ),
                ));
            }
        }
    }

    diagnostics
}

fn validate_id(id: &str, path: &str, diagnostics: &mut Vec<AuthoringDiagnostic>) {
    if id.trim().is_empty() {
        diagnostics.push(AuthoringDiagnostic::new(
            path,
            "invalid_id",
            "authored ids must not be empty",
        ));
    }
    if id.contains('/') {
        diagnostics.push(AuthoringDiagnostic::new(
            path,
            "invalid_id",
            "authored ids must not contain the reserved '/' source-map separator",
        ));
    }
}

fn find_property<'a>(
    model: &'a BehaviorModelSpec,
    property_id: &str,
) -> Option<&'a BehaviorPropertySpec> {
    model
        .properties
        .iter()
        .find(|property| property.id() == property_id)
}

fn binding_bool_value(spec: &AuthoringSpec, binding: &BehaviorBindingSpec) -> bool {
    let model = spec
        .behavior
        .models
        .iter()
        .find(|model| model.id == binding.model)
        .expect("validated behavior model");
    match find_property(model, &binding.property).expect("validated behavior property") {
        BehaviorPropertySpec::Bool { value, .. } => *value,
    }
}
