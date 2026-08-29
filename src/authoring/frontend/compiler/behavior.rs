use std::collections::{HashMap, HashSet};

use serde_json::{Value, json};

use crate::builder::{SceneSpec, build_scene};

use super::super::super::lower::runtime_name;
use super::super::super::spec::{
    AuthoringDiagnostic, AuthoringError, AuthoringSpec, BehaviorBindingSpec, BehaviorInputSpec,
    BehaviorListenerActionSpec, BehaviorListenerType, BehaviorModelSpec, BehaviorPropertySpec,
    BehaviorTransitionConditionSpec, SourceMapEntry,
};
use super::MotionTargetIndex;

pub(super) struct BehaviorLoweringOutput {
    pub(super) artboard_children: Vec<Value>,
    pub(super) state_machines: Vec<Value>,
    pub(super) source_entries: Vec<SourceMapEntry>,
}

pub(super) fn lower_behavior(
    spec: &AuthoringSpec,
    child_index_base: usize,
    state_machine_index_base: usize,
    listener_targets: MotionTargetIndex,
) -> Result<BehaviorLoweringOutput, AuthoringError> {
    let diagnostics = validate_behavior(spec);
    if !diagnostics.is_empty() {
        return Err(AuthoringError::many(diagnostics));
    }

    let mut source_entries = Vec::new();
    let mut artboard_children = Vec::with_capacity(
        spec.behavior.models.len()
            + spec
                .behavior
                .statecharts
                .iter()
                .map(|statechart| statechart.events.len())
                .sum::<usize>(),
    );
    let mut model_runtime_by_id = HashMap::new();
    let mut property_runtime_by_id = HashMap::new();
    for (model_index, model) in spec.behavior.models.iter().enumerate() {
        let model_name = runtime_name(&[spec.artboard.id.clone(), model.id.clone()], "view_model");
        let model_scene_path = format!("/artboard/children/{}", child_index_base + artboard_children.len());
        let mut properties = Vec::with_capacity(model.properties.len());
        for (property_index, property) in model.properties.iter().enumerate() {
            let property_name = runtime_name(
                &[
                    spec.artboard.id.clone(),
                    model.id.clone(),
                    property.id().to_string(),
                ],
                "view_model_property",
            );
            match property {
                BehaviorPropertySpec::Bool { .. } => properties.push(json!({
                    "type": "view_model_property_boolean",
                    "name": property_name
                })),
            }
            property_runtime_by_id
                .insert((model.id.as_str(), property.id()), property_name.clone());
            source_entries.push(SourceMapEntry {
                authored_id: format!("{}/{}", model.id, property.id()),
                authored_path: format!(
                    "$.behavior.models[{model_index}].properties[{property_index}]"
                ),
                definition_path: None,
                runtime_names: vec![property_name],
                scene_paths: vec![format!("{model_scene_path}/children/{property_index}")],
            });
        }
        model_runtime_by_id.insert(model.id.as_str(), model_name.clone());
        source_entries.push(SourceMapEntry {
            authored_id: model.id.clone(),
            authored_path: format!("$.behavior.models[{model_index}]"),
            definition_path: None,
            runtime_names: vec![model_name.clone()],
            scene_paths: vec![model_scene_path],
        });
        artboard_children.push(json!({
            "type": "view_model",
            "name": model_name,
            "children": properties
        }));
    }

    let mut state_machines = Vec::with_capacity(spec.behavior.statecharts.len());
    let mut binding_runtime: HashMap<usize, (Vec<String>, Vec<String>)> = HashMap::new();

    for (statechart_index, statechart) in spec.behavior.statecharts.iter().enumerate() {
        let scene_machine_index = state_machine_index_base + statechart_index;
        let machine_path = format!("/artboard/state_machines/{scene_machine_index}");
        let machine_name = runtime_name(
            &[spec.artboard.id.clone(), statechart.id.clone()],
            "state_machine",
        );
        let statechart_path = format!("$.behavior.statecharts[{statechart_index}]");

        let mut event_name_by_id = HashMap::new();
        for (event_index, event) in statechart.events.iter().enumerate() {
            let event_name = runtime_name(
                &[
                    spec.artboard.id.clone(),
                    statechart.id.clone(),
                    event.id.clone(),
                ],
                "event",
            );
            let event_scene_path = format!(
                "/artboard/children/{}",
                child_index_base + artboard_children.len()
            );
            event_name_by_id.insert(event.id.as_str(), event_name.clone());
            artboard_children.push(json!({
                "type": "event",
                "name": event_name
            }));
            source_entries.push(SourceMapEntry {
                authored_id: format!("{}/{}", statechart.id, event.id),
                authored_path: format!("{statechart_path}.events[{event_index}]"),
                definition_path: None,
                runtime_names: vec![event_name],
                scene_paths: vec![event_scene_path],
            });
        }

        let used_bindings = statechart
            .transitions
            .iter()
            .filter_map(|transition| match &transition.when {
                BehaviorTransitionConditionSpec::Binding(condition) => {
                    Some(condition.binding.as_str())
                }
                BehaviorTransitionConditionSpec::Input(_) => None,
            })
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
            let model_name = model_runtime_by_id
                .get(binding.model.as_str())
                .expect("validated behavior model");
            let property_name = property_runtime_by_id
                .get(&(binding.model.as_str(), binding.property.as_str()))
                .expect("validated behavior property");
            inputs.push(json!({
                "type": "bool",
                "name": input_name,
                "value": binding_bool_value(spec, binding),
                "view_model_binding": {
                    "view_model": model_name,
                    "property": property_name
                }
            }));

            let runtime = binding_runtime.entry(binding_index).or_default();
            runtime.0.push(input_name);
            runtime
                .1
                .push(format!("{machine_path}/inputs/{input_index}"));
        }

        let mut input_name_by_id = HashMap::new();
        for (input_index, input) in statechart.inputs.iter().enumerate() {
            let scene_input_index = inputs.len();
            let input_name = runtime_name(
                &[
                    spec.artboard.id.clone(),
                    statechart.id.clone(),
                    input.id().to_string(),
                ],
                "input",
            );
            inputs.push(json!({
                "type": "bool",
                "name": input_name,
                "value": input.bool_value()
            }));
            input_name_by_id.insert(input.id(), input_name.clone());
            source_entries.push(SourceMapEntry {
                authored_id: format!("{}/{}", statechart.id, input.id()),
                authored_path: format!("{statechart_path}.inputs[{input_index}]"),
                definition_path: None,
                runtime_names: vec![input_name],
                scene_paths: vec![format!("{machine_path}/inputs/{scene_input_index}")],
            });
        }

        let mut listeners = Vec::with_capacity(statechart.listeners.len());
        for (listener_index, listener) in statechart.listeners.iter().enumerate() {
            let listener_path = format!("{statechart_path}.listeners[{listener_index}]");
            let target = if listener.listener_type == BehaviorListenerType::Event {
                event_name_by_id
                    .get(listener.target.as_str())
                    .expect("validated behavior event")
                    .clone()
            } else {
                resolve_listener_target(
                    &listener_targets,
                    &listener.target,
                    &format!("{listener_path}.target"),
                )?
            };
            let actions = listener
                .actions
                .iter()
                .map(|action| match action {
                    BehaviorListenerActionSpec::BoolChange { input, value } => json!({
                        "type": "bool_change",
                        "input": input_name_by_id
                            .get(input.as_str())
                            .expect("validated behavior listener input"),
                        "value": value
                    }),
                })
                .collect::<Vec<_>>();
            listeners.push(json!({
                "target": target,
                "listener_type": listener.listener_type.as_str(),
                "actions": actions
            }));
            source_entries.push(SourceMapEntry {
                authored_id: format!("{}/{}", statechart.id, listener.id),
                authored_path: listener_path,
                definition_path: None,
                runtime_names: Vec::new(),
                scene_paths: vec![format!("{machine_path}/listeners/{listener_index}")],
            });
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
        states.push(json!({ "type": "exit" }));

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
            let (input, equals) = match &transition.when {
                BehaviorTransitionConditionSpec::Binding(condition) => (
                    input_name_by_binding
                        .get(condition.binding.as_str())
                        .expect("validated transition binding"),
                    condition.equals,
                ),
                BehaviorTransitionConditionSpec::Input(condition) => (
                    input_name_by_id
                        .get(condition.input.as_str())
                        .expect("validated transition input"),
                    condition.equals,
                ),
            };
            transitions.push(json!({
                "from": from,
                "to": to,
                "conditions": [
                    {
                        "input": input,
                        "value": equals
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

        let mut machine = json!({
            "name": machine_name,
            "inputs": inputs,
            "layers": [
                {
                    "states": states,
                    "transitions": transitions
                }
            ]
        });
        if !listeners.is_empty() {
            machine["listeners"] = Value::Array(listeners);
        }
        state_machines.push(machine);
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
        artboard_children,
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
                        property.id(),
                        model.id
                    ),
                ));
            }
        }
    }

    let mut bindings = HashMap::new();
    for (binding_index, binding) in spec.behavior.bindings.iter().enumerate() {
        let binding_path = format!("$.behavior.bindings[{binding_index}]");
        validate_id(&binding.id, &format!("{binding_path}.id"), &mut diagnostics);
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

        let mut inputs = HashMap::new();
        for (input_index, input) in statechart.inputs.iter().enumerate() {
            let input_path = format!("{statechart_path}.inputs[{input_index}]");
            validate_id(input.id(), &format!("{input_path}.id"), &mut diagnostics);
            if inputs.insert(input.id(), input).is_some() {
                diagnostics.push(AuthoringDiagnostic::new(
                    format!("{input_path}.id"),
                    "duplicate_behavior_input",
                    format!(
                        "behavior input id '{}' is duplicated in statechart '{}'",
                        input.id(), statechart.id
                    ),
                ));
            }
            if bindings.contains_key(input.id()) {
                diagnostics.push(AuthoringDiagnostic::new(
                    format!("{input_path}.id"),
                    "behavior_input_binding_collision",
                    format!(
                        "behavior input id '{}' conflicts with a behavior binding id",
                        input.id()
                    ),
                ));
            }
        }

        let mut events = HashSet::new();
        for (event_index, event) in statechart.events.iter().enumerate() {
            let event_path = format!("{statechart_path}.events[{event_index}]");
            validate_id(&event.id, &format!("{event_path}.id"), &mut diagnostics);
            if !events.insert(event.id.as_str()) {
                diagnostics.push(AuthoringDiagnostic::new(
                    format!("{event_path}.id"),
                    "duplicate_behavior_event",
                    format!(
                        "behavior event id '{}' is duplicated in statechart '{}'",
                        event.id, statechart.id
                    ),
                ));
            }
        }

        let mut listeners = HashSet::new();
        for (listener_index, listener) in statechart.listeners.iter().enumerate() {
            let listener_path = format!("{statechart_path}.listeners[{listener_index}]");
            validate_id(
                &listener.id,
                &format!("{listener_path}.id"),
                &mut diagnostics,
            );
            if !listeners.insert(listener.id.as_str()) {
                diagnostics.push(AuthoringDiagnostic::new(
                    format!("{listener_path}.id"),
                    "duplicate_behavior_listener",
                    format!(
                        "behavior listener id '{}' is duplicated in statechart '{}'",
                        listener.id, statechart.id
                    ),
                ));
            }
            if listener.listener_type == BehaviorListenerType::Event
                && !events.contains(listener.target.as_str())
            {
                diagnostics.push(AuthoringDiagnostic::new(
                    format!("{listener_path}.target"),
                    "unknown_behavior_event",
                    format!(
                        "behavior event '{}' is not defined in statechart '{}'",
                        listener.target, statechart.id
                    ),
                ));
            }
            for (action_index, action) in listener.actions.iter().enumerate() {
                match action {
                    BehaviorListenerActionSpec::BoolChange { input, .. }
                        if !inputs.contains_key(input.as_str()) =>
                    {
                        diagnostics.push(AuthoringDiagnostic::new(
                            format!("{listener_path}.actions[{action_index}].input"),
                            "unknown_behavior_input",
                            format!(
                                "behavior input '{}' is not defined in statechart '{}'",
                                input, statechart.id
                            ),
                        ));
                    }
                    BehaviorListenerActionSpec::BoolChange { .. } => {}
                }
            }
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
            match &transition.when {
                BehaviorTransitionConditionSpec::Binding(condition)
                    if !bindings.contains_key(condition.binding.as_str()) =>
                {
                    diagnostics.push(AuthoringDiagnostic::new(
                        format!("{transition_path}.when.binding"),
                        "unknown_behavior_binding",
                        format!(
                            "behavior binding '{}' is not defined",
                            condition.binding
                        ),
                    ));
                }
                BehaviorTransitionConditionSpec::Input(condition)
                    if !inputs.contains_key(condition.input.as_str()) =>
                {
                    diagnostics.push(AuthoringDiagnostic::new(
                        format!("{transition_path}.when.input"),
                        "unknown_behavior_input",
                        format!(
                            "behavior input '{}' is not defined in statechart '{}'",
                            condition.input, statechart.id
                        ),
                    ));
                }
                _ => {}
            }
        }
    }

    diagnostics
}

fn resolve_listener_target(
    listener_targets: &MotionTargetIndex,
    target: &str,
    path: &str,
) -> Result<String, AuthoringError> {
    let bindings = listener_targets.resolve(target, path).map_err(|diagnostic| {
        let code = match diagnostic.code.as_str() {
            "unknown_motion_target" => "unknown_listener_target",
            "ambiguous_motion_target" => "ambiguous_listener_target",
            "unsupported_motion_target" => "unsupported_listener_target",
            _ => "invalid_listener_target",
        };
        AuthoringError::one(AuthoringDiagnostic::new(
            path,
            code,
            diagnostic.message.replace("visual target", "listener target"),
        ))
    })?;
    let binding = bindings
        .iter()
        .find(|binding| binding.is_primary)
        .or_else(|| bindings.first())
        .expect("validated listener target must expose a runtime binding");
    Ok(binding.runtime_name.clone())
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
