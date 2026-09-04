use std::collections::{BTreeMap, HashMap, HashSet};

use serde_json::{Value, json};

use crate::builder::{SceneSpec, build_scene};

use super::super::super::expression::evaluate_expression;
use super::super::super::lower::{runtime_name, without_asset_sources};
use super::super::super::spec::{
    AuthoringDiagnostic, AuthoringError, AuthoringSpec, BehaviorBindingSpec, BehaviorInputKind,
    BehaviorInputSpec, BehaviorListenerActionSpec, BehaviorListenerType, BehaviorModelSpec,
    BehaviorPropertySpec, BehaviorStateSpec, BehaviorTransitionConditionSpec,
    BehaviorTransitionSpec, Quantity, SourceMapEntry, Unit,
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
        let model_scene_path = format!(
            "/artboard/children/{}",
            child_index_base + artboard_children.len()
        );
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
            .chain(
                statechart
                    .regions
                    .iter()
                    .flat_map(|region| &region.transitions),
            )
            .filter_map(|transition| match &transition.when {
                BehaviorTransitionConditionSpec::Binding(condition) => {
                    Some(condition.binding.as_str())
                }
                _ => None,
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
            let input_path = format!("{statechart_path}.inputs[{input_index}]");
            let scene_input_index = inputs.len();
            let input_name = runtime_name(
                &[
                    spec.artboard.id.clone(),
                    statechart.id.clone(),
                    input.id().to_string(),
                ],
                "input",
            );
            inputs.push(match input {
                BehaviorInputSpec::Bool { value, .. } => json!({
                    "type": "bool",
                    "name": input_name,
                    "value": value
                }),
                BehaviorInputSpec::Number { value, .. } => json!({
                    "type": "number",
                    "name": input_name,
                    "value": evaluate_expression(
                        value,
                        &format!("{input_path}.value"),
                        &spec.parameters,
                        Unit::Scalar,
                    )
                    .map_err(AuthoringError::one)?
                }),
                BehaviorInputSpec::Trigger { .. } => json!({
                    "type": "trigger",
                    "name": input_name
                }),
            });
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
            let mut actions = Vec::with_capacity(listener.actions.len());
            for (action_index, action) in listener.actions.iter().enumerate() {
                let input = input_name_by_id
                    .get(action.input())
                    .expect("validated behavior listener input");
                actions.push(match action {
                    BehaviorListenerActionSpec::BoolChange { value, .. } => json!({
                        "type": "bool_change",
                        "input": input,
                        "value": value
                    }),
                    BehaviorListenerActionSpec::NumberChange { value, .. } => json!({
                        "type": "number_change",
                        "input": input,
                        "value": evaluate_expression(
                            value,
                            &format!("{listener_path}.actions[{action_index}].value"),
                            &spec.parameters,
                            Unit::Scalar,
                        )
                        .map_err(AuthoringError::one)?
                    }),
                    BehaviorListenerActionSpec::TriggerChange { .. } => json!({
                        "type": "trigger_change",
                        "input": input
                    }),
                });
            }
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

        let mut layers = Vec::with_capacity(1 + statechart.regions.len());
        layers.push(
            lower_region(
                RegionContext {
                    spec,
                    statechart_id: &statechart.id,
                    machine_path: &machine_path,
                    layer_index: 0,
                    authored_path: statechart_path.clone(),
                    region_id: None,
                    initial: &statechart.initial,
                    states: &statechart.states,
                    transitions: &statechart.transitions,
                },
                &input_name_by_id,
                &input_name_by_binding,
                &mut source_entries,
            )
            .map_err(AuthoringError::one)?,
        );
        for (region_index, region) in statechart.regions.iter().enumerate() {
            let region_path = format!("{statechart_path}.regions[{region_index}]");
            let layer_index = layers.len();
            layers.push(
                lower_region(
                    RegionContext {
                        spec,
                        statechart_id: &statechart.id,
                        machine_path: &machine_path,
                        layer_index,
                        authored_path: region_path.clone(),
                        region_id: Some(&region.id),
                        initial: &region.initial,
                        states: &region.states,
                        transitions: &region.transitions,
                    },
                    &input_name_by_id,
                    &input_name_by_binding,
                    &mut source_entries,
                )
                .map_err(AuthoringError::one)?,
            );
            source_entries.push(SourceMapEntry {
                authored_id: format!("{}/{}", statechart.id, region.id),
                authored_path: region_path,
                definition_path: None,
                runtime_names: Vec::new(),
                scene_paths: vec![format!("{machine_path}/layers/{layer_index}")],
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
            "layers": layers
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

struct RegionContext<'a> {
    spec: &'a AuthoringSpec,
    statechart_id: &'a str,
    machine_path: &'a str,
    layer_index: usize,
    authored_path: String,
    region_id: Option<&'a str>,
    initial: &'a str,
    states: &'a [BehaviorStateSpec],
    transitions: &'a [BehaviorTransitionSpec],
}

fn lower_region(
    context: RegionContext<'_>,
    input_name_by_id: &HashMap<&str, String>,
    input_name_by_binding: &HashMap<&str, String>,
    source_entries: &mut Vec<SourceMapEntry>,
) -> Result<Value, AuthoringDiagnostic> {
    let RegionContext {
        spec,
        statechart_id,
        machine_path,
        layer_index,
        authored_path,
        region_id,
        initial,
        states: authored_states,
        transitions: authored_transitions,
    } = context;

    let state_index_by_id = authored_states
        .iter()
        .enumerate()
        .map(|(index, state)| (state.id.as_str(), index + 1))
        .collect::<HashMap<_, _>>();

    let mut states = vec![json!({ "type": "entry" })];
    for (state_index, state) in authored_states.iter().enumerate() {
        let state_path = format!("{authored_path}.states[{state_index}]");
        let lowered = match (&state.motion, &state.blend) {
            (Some(motion), None) => json!({
                "type": "animation",
                "animation": animation_runtime_name(spec, motion)
            }),
            (None, Some(blend)) => {
                let input = input_name_by_id
                    .get(blend.input.as_str())
                    .expect("validated blend input");
                let mut children = Vec::with_capacity(blend.stops.len());
                for (stop_index, stop) in blend.stops.iter().enumerate() {
                    children.push(json!({
                        "type": "blend_animation_1d",
                        "animation": animation_runtime_name(spec, &stop.motion),
                        "value": evaluate_expression(
                            &stop.value,
                            &format!("{state_path}.blend.stops[{stop_index}].value"),
                            &spec.parameters,
                            Unit::Scalar,
                        )?
                    }));
                }
                json!({
                    "type": "blend_state_1d",
                    "input": input,
                    "children": children
                })
            }
            _ => {
                return Err(AuthoringDiagnostic::new(
                    state_path,
                    "missing_state_motion",
                    "a behavior state must declare exactly one of 'motion' or 'blend'",
                ));
            }
        };
        states.push(lowered);
        source_entries.push(SourceMapEntry {
            authored_id: region_scoped_id(statechart_id, region_id, &state.id),
            authored_path: state_path,
            definition_path: None,
            runtime_names: Vec::new(),
            scene_paths: vec![format!(
                "{machine_path}/layers/{layer_index}/states/{}",
                state_index + 1
            )],
        });
    }
    states.push(json!({ "type": "exit" }));

    let initial_state = *state_index_by_id
        .get(initial)
        .expect("validated initial state");
    let mut transitions = vec![json!({ "from": 0, "to": initial_state })];
    for (transition_index, transition) in authored_transitions.iter().enumerate() {
        let transition_path = format!("{authored_path}.transitions[{transition_index}]");
        let from = *state_index_by_id
            .get(transition.from.as_str())
            .expect("validated transition source");
        let to = *state_index_by_id
            .get(transition.to.as_str())
            .expect("validated transition target");
        let condition = match &transition.when {
            BehaviorTransitionConditionSpec::Binding(condition) => json!({
                "input": input_name_by_binding
                    .get(condition.binding.as_str())
                    .expect("validated transition binding"),
                "value": condition.equals
            }),
            BehaviorTransitionConditionSpec::Input(condition) => json!({
                "input": input_name_by_id
                    .get(condition.input.as_str())
                    .expect("validated transition input"),
                "value": condition.equals
            }),
            BehaviorTransitionConditionSpec::Number(condition) => json!({
                "input": input_name_by_id
                    .get(condition.input.as_str())
                    .expect("validated transition input"),
                "op": condition.compare.as_str(),
                "value": evaluate_expression(
                    &condition.value,
                    &format!("{transition_path}.when.value"),
                    &spec.parameters,
                    Unit::Scalar,
                )?
            }),
            BehaviorTransitionConditionSpec::Trigger(condition) => json!({
                "input": input_name_by_id
                    .get(condition.trigger.as_str())
                    .expect("validated transition trigger")
            }),
        };
        transitions.push(json!({
            "from": from,
            "to": to,
            "conditions": [condition]
        }));
        source_entries.push(SourceMapEntry {
            authored_id: region_scoped_id(statechart_id, region_id, &transition.id),
            authored_path: transition_path,
            definition_path: None,
            runtime_names: Vec::new(),
            scene_paths: vec![format!(
                "{machine_path}/layers/{layer_index}/transitions/{}",
                transition_index + 1
            )],
        });
    }

    Ok(json!({
        "states": states,
        "transitions": transitions
    }))
}

fn animation_runtime_name(spec: &AuthoringSpec, motion: &str) -> String {
    runtime_name(&[spec.artboard.id.clone(), motion.to_string()], "animation")
}

fn region_scoped_id(statechart_id: &str, region_id: Option<&str>, id: &str) -> String {
    match region_id {
        Some(region) => format!("{statechart_id}/{region}/{id}"),
        None => format!("{statechart_id}/{id}"),
    }
}

pub(super) fn validate_lowered_scene(scene: &Value) -> Result<(), AuthoringError> {
    let scene: SceneSpec =
        serde_json::from_value(without_asset_sources(scene)).map_err(|error| {
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
                        input.id(),
                        statechart.id
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
                let action_path = format!("{listener_path}.actions[{action_index}].input");
                match inputs.get(action.input()) {
                    None => diagnostics.push(AuthoringDiagnostic::new(
                        action_path,
                        "unknown_behavior_input",
                        format!(
                            "behavior input '{}' is not defined in statechart '{}'",
                            action.input(),
                            statechart.id
                        ),
                    )),
                    Some(input) if input.kind() != action.input_kind() => {
                        diagnostics.push(AuthoringDiagnostic::new(
                            action_path,
                            "invalid_listener_input",
                            format!(
                                "listener action expects a {} input but '{}' is declared as {}",
                                action.input_kind().as_str(),
                                action.input(),
                                input.kind().as_str()
                            ),
                        ));
                    }
                    Some(_) => {}
                }
            }
        }

        let region = RegionValidation {
            statechart_id: &statechart.id,
            region_path: &statechart_path,
            initial: &statechart.initial,
            states: &statechart.states,
            transitions: &statechart.transitions,
        };
        validate_region(
            region,
            &spec.parameters,
            &motion_tracks,
            &inputs,
            &bindings,
            &mut diagnostics,
        );

        let statechart_scoped_ids = statechart
            .states
            .iter()
            .map(|state| state.id.as_str())
            .chain(
                statechart
                    .transitions
                    .iter()
                    .map(|transition| transition.id.as_str()),
            )
            .chain(statechart.inputs.iter().map(|input| input.id()))
            .chain(statechart.events.iter().map(|event| event.id.as_str()))
            .chain(
                statechart
                    .listeners
                    .iter()
                    .map(|listener| listener.id.as_str()),
            )
            .collect::<HashSet<_>>();
        let mut regions = HashSet::new();
        for (region_index, region) in statechart.regions.iter().enumerate() {
            let region_path = format!("{statechart_path}.regions[{region_index}]");
            validate_id(&region.id, &format!("{region_path}.id"), &mut diagnostics);
            if !regions.insert(region.id.as_str()) {
                diagnostics.push(AuthoringDiagnostic::new(
                    format!("{region_path}.id"),
                    "duplicate_behavior_region",
                    format!(
                        "behavior region id '{}' is duplicated in statechart '{}'",
                        region.id, statechart.id
                    ),
                ));
            }
            if statechart_scoped_ids.contains(region.id.as_str()) {
                diagnostics.push(AuthoringDiagnostic::new(
                    format!("{region_path}.id"),
                    "behavior_region_id_collision",
                    format!(
                        "behavior region id '{}' is also a state, transition, input, event or listener id in statechart '{}', so both would claim the source-map identity '{}/{}'",
                        region.id, statechart.id, statechart.id, region.id
                    ),
                ));
            }
            validate_region(
                RegionValidation {
                    statechart_id: &statechart.id,
                    region_path: &region_path,
                    initial: &region.initial,
                    states: &region.states,
                    transitions: &region.transitions,
                },
                &spec.parameters,
                &motion_tracks,
                &inputs,
                &bindings,
                &mut diagnostics,
            );
        }
    }

    diagnostics
}

const BLEND_STOP_MINIMUM: usize = 2;
const BLEND_STOP_LIMIT: usize = 1000;

struct RegionValidation<'a> {
    statechart_id: &'a str,
    region_path: &'a str,
    initial: &'a str,
    states: &'a [BehaviorStateSpec],
    transitions: &'a [BehaviorTransitionSpec],
}

fn validate_region(
    region: RegionValidation<'_>,
    parameters: &BTreeMap<String, Quantity>,
    motion_tracks: &HashSet<&str>,
    inputs: &HashMap<&str, &BehaviorInputSpec>,
    bindings: &HashMap<&str, &BehaviorBindingSpec>,
    diagnostics: &mut Vec<AuthoringDiagnostic>,
) {
    let RegionValidation {
        statechart_id,
        region_path,
        initial,
        states: authored_states,
        transitions: authored_transitions,
    } = region;

    let mut states = HashSet::new();
    for (state_index, state) in authored_states.iter().enumerate() {
        let state_path = format!("{region_path}.states[{state_index}]");
        validate_id(&state.id, &format!("{state_path}.id"), diagnostics);
        if !states.insert(state.id.as_str()) {
            diagnostics.push(AuthoringDiagnostic::new(
                format!("{state_path}.id"),
                "duplicate_behavior_state",
                format!(
                    "behavior state id '{}' is duplicated in statechart '{statechart_id}'",
                    state.id
                ),
            ));
        }
        validate_state_motion(
            state,
            &state_path,
            statechart_id,
            parameters,
            motion_tracks,
            inputs,
            diagnostics,
        );
    }
    if !states.contains(initial) {
        diagnostics.push(AuthoringDiagnostic::new(
            format!("{region_path}.initial"),
            "unknown_behavior_state",
            format!("initial state '{initial}' is not defined in statechart '{statechart_id}'"),
        ));
    }

    let mut transitions = HashSet::new();
    for (transition_index, transition) in authored_transitions.iter().enumerate() {
        let transition_path = format!("{region_path}.transitions[{transition_index}]");
        validate_id(
            &transition.id,
            &format!("{transition_path}.id"),
            diagnostics,
        );
        if !transitions.insert(transition.id.as_str()) {
            diagnostics.push(AuthoringDiagnostic::new(
                format!("{transition_path}.id"),
                "duplicate_behavior_transition",
                format!(
                    "behavior transition id '{}' is duplicated in statechart '{statechart_id}'",
                    transition.id
                ),
            ));
        }
        for (field, state) in [("from", &transition.from), ("to", &transition.to)] {
            if !states.contains(state.as_str()) {
                diagnostics.push(AuthoringDiagnostic::new(
                    format!("{transition_path}.{field}"),
                    "unknown_behavior_state",
                    format!("state '{state}' is not defined in statechart '{statechart_id}'"),
                ));
            }
        }
        validate_condition(
            &transition.when,
            &transition_path,
            statechart_id,
            inputs,
            bindings,
            diagnostics,
        );
    }
}

fn validate_state_motion(
    state: &BehaviorStateSpec,
    state_path: &str,
    statechart_id: &str,
    parameters: &BTreeMap<String, Quantity>,
    motion_tracks: &HashSet<&str>,
    inputs: &HashMap<&str, &BehaviorInputSpec>,
    diagnostics: &mut Vec<AuthoringDiagnostic>,
) {
    match (&state.motion, &state.blend) {
        (None, None) => diagnostics.push(AuthoringDiagnostic::new(
            state_path,
            "missing_state_motion",
            format!(
                "behavior state '{}' must declare either a motion track or a blend",
                state.id
            ),
        )),
        (Some(_), Some(_)) => diagnostics.push(AuthoringDiagnostic::new(
            state_path,
            "ambiguous_state_motion",
            format!(
                "behavior state '{}' declares both a motion track and a blend",
                state.id
            ),
        )),
        (Some(motion), None) => {
            if !motion_tracks.contains(motion.as_str()) {
                diagnostics.push(AuthoringDiagnostic::new(
                    format!("{state_path}.motion"),
                    "unknown_behavior_motion",
                    format!("motion track '{motion}' is not defined"),
                ));
            }
        }
        (None, Some(blend)) => {
            match inputs.get(blend.input.as_str()) {
                None => diagnostics.push(AuthoringDiagnostic::new(
                    format!("{state_path}.blend.input"),
                    "unknown_behavior_input",
                    format!(
                        "behavior input '{}' is not defined in statechart '{statechart_id}'",
                        blend.input
                    ),
                )),
                Some(input) if input.kind() != BehaviorInputKind::Number => {
                    diagnostics.push(AuthoringDiagnostic::new(
                        format!("{state_path}.blend.input"),
                        "invalid_blend_input",
                        format!(
                            "blend input '{}' must be a number input but is declared as {}",
                            blend.input,
                            input.kind().as_str()
                        ),
                    ));
                }
                Some(_) => {}
            }
            if !(BLEND_STOP_MINIMUM..=BLEND_STOP_LIMIT).contains(&blend.stops.len()) {
                diagnostics.push(AuthoringDiagnostic::new(
                    format!("{state_path}.blend.stops"),
                    "invalid_blend_stops",
                    format!(
                        "a blend needs between {BLEND_STOP_MINIMUM} and {BLEND_STOP_LIMIT} stops but declares {}",
                        blend.stops.len()
                    ),
                ));
            }
            let mut previous = None;
            for (stop_index, stop) in blend.stops.iter().enumerate() {
                if !motion_tracks.contains(stop.motion.as_str()) {
                    diagnostics.push(AuthoringDiagnostic::new(
                        format!("{state_path}.blend.stops[{stop_index}].motion"),
                        "unknown_behavior_motion",
                        format!("motion track '{}' is not defined", stop.motion),
                    ));
                }
                let stop_path = format!("{state_path}.blend.stops[{stop_index}].value");
                let value =
                    match evaluate_expression(&stop.value, &stop_path, parameters, Unit::Scalar) {
                        Ok(value) => value,
                        Err(diagnostic) => {
                            diagnostics.push(diagnostic);
                            continue;
                        }
                    };
                let value = value as f32;
                if let Some(previous_value) = previous
                    && value <= previous_value
                {
                    diagnostics.push(AuthoringDiagnostic::new(
                        stop_path,
                        "invalid_blend_stop_order",
                        format!(
                            "blend stop values must increase once narrowed to the emitted 32-bit float, but {value} does not follow {previous_value}"
                        ),
                    ));
                }
                previous = Some(value);
            }
        }
    }
}

fn validate_condition(
    condition: &BehaviorTransitionConditionSpec,
    transition_path: &str,
    statechart_id: &str,
    inputs: &HashMap<&str, &BehaviorInputSpec>,
    bindings: &HashMap<&str, &BehaviorBindingSpec>,
    diagnostics: &mut Vec<AuthoringDiagnostic>,
) {
    let (field, id, expected) = match condition {
        BehaviorTransitionConditionSpec::Binding(condition) => {
            if !bindings.contains_key(condition.binding.as_str()) {
                diagnostics.push(AuthoringDiagnostic::new(
                    format!("{transition_path}.when.binding"),
                    "unknown_behavior_binding",
                    format!("behavior binding '{}' is not defined", condition.binding),
                ));
            }
            return;
        }
        BehaviorTransitionConditionSpec::Input(condition) => {
            ("input", &condition.input, BehaviorInputKind::Bool)
        }
        BehaviorTransitionConditionSpec::Number(condition) => {
            ("input", &condition.input, BehaviorInputKind::Number)
        }
        BehaviorTransitionConditionSpec::Trigger(condition) => {
            ("trigger", &condition.trigger, BehaviorInputKind::Trigger)
        }
    };
    match inputs.get(id.as_str()) {
        None => diagnostics.push(AuthoringDiagnostic::new(
            format!("{transition_path}.when.{field}"),
            "unknown_behavior_input",
            format!("behavior input '{id}' is not defined in statechart '{statechart_id}'"),
        )),
        Some(input) if input.kind() != expected => {
            diagnostics.push(AuthoringDiagnostic::new(
                format!("{transition_path}.when.{field}"),
                "invalid_condition_input",
                format!(
                    "condition expects a {} input but '{id}' is declared as {}",
                    expected.as_str(),
                    input.kind().as_str()
                ),
            ));
        }
        Some(_) => {}
    }
}

fn resolve_listener_target(
    listener_targets: &MotionTargetIndex,
    target: &str,
    path: &str,
) -> Result<String, AuthoringError> {
    let bindings = listener_targets
        .resolve(target, path)
        .map_err(|diagnostic| {
            let code = match diagnostic.code.as_str() {
                "unknown_motion_target" => "unknown_listener_target",
                "ambiguous_motion_target" => "ambiguous_listener_target",
                "unsupported_motion_target" => "unsupported_listener_target",
                _ => "invalid_listener_target",
            };
            AuthoringError::one(AuthoringDiagnostic::new(
                path,
                code,
                diagnostic
                    .message
                    .replace("visual target", "listener target"),
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
