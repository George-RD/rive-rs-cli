use std::collections::HashMap;

use crate::objects::core::RiveObject;
use crate::objects::state_machine::{
    AnimationState, AnyState, BlendAnimation, BlendAnimation1D, BlendAnimationDirect, BlendState,
    BlendState1D, BlendState1DInput, BlendStateDirect, EntryState, ExitState,
    ListenerBoolChange, ListenerNumberChange, ListenerTriggerChange, StateMachine,
    StateMachineBool, StateMachineLayer, StateMachineListener, StateMachineNumber,
    StateMachineTrigger, StateTransition, TransitionBoolCondition, TransitionInputCondition,
    TransitionNumberCondition, TransitionPropertyComparator, TransitionTriggerCondition,
    TransitionValueBooleanComparator, TransitionValueColorComparator, TransitionValueCondition,
    TransitionValueEnumComparator, TransitionValueNumberComparator,
    TransitionValueStringComparator, TransitionValueTriggerComparator,
    TransitionViewModelCondition,
};

use super::parsers::{input_is_trigger, json_value_to_f32, parse_color, parse_condition_op};
use super::spec::{
    BlendState1DChildSpec, BlendStateChildSpec, BlendStateDirectChildSpec, InputSpec,
    ListenerActionSpec, StateMachineSpec, StateSpec, TransitionChildSpec,
};

/// Builds all state machine objects for an artboard.
pub(crate) fn build_state_machines(
    state_machines: &[StateMachineSpec],
    artboard_start: usize,
    objects: &mut Vec<Box<dyn RiveObject>>,
    object_name_to_index: &HashMap<String, usize>,
    animation_name_to_index: &HashMap<String, usize>,
) -> Result<(), String> {
    for state_machine in state_machines {
        objects.push(Box::new(StateMachine::new(state_machine.name.clone())));

        let mut input_name_to_index: HashMap<String, usize> = HashMap::new();
        if let Some(inputs) = &state_machine.inputs {
            for (input_index, input) in inputs.iter().enumerate() {
                match input {
                    InputSpec::Number { name, value } => {
                        objects.push(Box::new(StateMachineNumber {
                            name: name.clone(),
                            value: *value,
                        }));
                        input_name_to_index.insert(name.clone(), input_index);
                    }
                    InputSpec::Bool { name, value } => {
                        objects.push(Box::new(StateMachineBool {
                            name: name.clone(),
                            value: if *value { 1 } else { 0 },
                        }));
                        input_name_to_index.insert(name.clone(), input_index);
                    }
                    InputSpec::Trigger { name } => {
                        objects.push(Box::new(StateMachineTrigger { name: name.clone() }));
                        input_name_to_index.insert(name.clone(), input_index);
                    }
                }
            }
        }

        if let Some(listeners) = &state_machine.listeners {
            for listener in listeners {
                let target_global =
                    *object_name_to_index.get(&listener.target).ok_or_else(|| {
                        format!(
                            "unknown target referenced in state machine listener: '{}'",
                            listener.target
                        )
                    })?;
                let listener_target_id =
                    target_global.checked_sub(artboard_start).ok_or_else(|| {
                        format!(
                            "state machine listener target '{}' precedes current artboard",
                            listener.target
                        )
                    })? as u64;
                objects.push(Box::new(StateMachineListener {
                    target_id: listener_target_id,
                    listener_type_value: listener.listener_type_value.unwrap_or(0),
                }));

                if let Some(actions) = &listener.actions {
                    for action in actions {
                        match action {
                            ListenerActionSpec::BoolChange { input, value } => {
                                let input_index = *input_name_to_index.get(input).ok_or_else(|| {
                                    format!(
                                        "unknown input referenced in listener action: '{}'",
                                        input
                                    )
                                })?;
                                let bool_value = match value {
                                    Some(serde_json::Value::Bool(v)) => {
                                        if *v { 1 } else { 0 }
                                    }
                                    Some(serde_json::Value::Number(n)) => n
                                        .as_u64()
                                        .ok_or_else(|| {
                                            format!(
                                                "listener bool_change value for input '{}' must be bool or unsigned integer",
                                                input
                                            )
                                        })?,
                                    Some(_) => {
                                        return Err(format!(
                                            "listener bool_change value for input '{}' must be bool or unsigned integer",
                                            input
                                        ))
                                    }
                                    None => 1,
                                };
                                objects.push(Box::new(ListenerBoolChange {
                                    input_id: input_index as u64,
                                    value: bool_value,
                                }));
                            }
                            ListenerActionSpec::TriggerChange { input } => {
                                let input_index = *input_name_to_index.get(input).ok_or_else(|| {
                                    format!(
                                        "unknown input referenced in listener action: '{}'",
                                        input
                                    )
                                })?;
                                objects.push(Box::new(ListenerTriggerChange {
                                    input_id: input_index as u64,
                                }));
                            }
                            ListenerActionSpec::NumberChange { input, value } => {
                                let input_index = *input_name_to_index.get(input).ok_or_else(|| {
                                    format!(
                                        "unknown input referenced in listener action: '{}'",
                                        input
                                    )
                                })?;
                                let number_value = match value {
                                    Some(v) => json_value_to_f32(v).ok_or_else(|| {
                                        format!(
                                            "listener number_change value for input '{}' must be numeric",
                                            input
                                        )
                                    })?,
                                    None => 0.0,
                                };
                                objects.push(Box::new(ListenerNumberChange {
                                    input_id: input_index as u64,
                                    value: number_value,
                                }));
                            }
                        }
                    }
                }
            }
        }

        for (layer_index, layer) in state_machine.layers.iter().enumerate() {
            objects.push(Box::new(StateMachineLayer {
                name: format!("Layer {}", layer_index),
            }));

            let has_any = layer.states.iter().any(|s| matches!(s, StateSpec::Any));

            let mut user_to_final: Vec<usize> = Vec::new();
            let mut final_idx = if has_any { 0 } else { 1 };
            for _ in &layer.states {
                user_to_final.push(final_idx);
                final_idx += 1;
            }

            if !has_any {
                objects.push(Box::new(AnyState));
            }

            for (user_idx, state) in layer.states.iter().enumerate() {
                match state {
                    StateSpec::Entry => {
                        objects.push(Box::new(EntryState));
                    }
                    StateSpec::Exit => {
                        objects.push(Box::new(ExitState));
                    }
                    StateSpec::Any => {
                        objects.push(Box::new(AnyState));
                    }
                    StateSpec::Animation { animation } => {
                        let animation_id =
                            *animation_name_to_index.get(animation).ok_or_else(|| {
                                format!("unknown animation referenced: '{}'", animation)
                            })? as u64;
                        objects.push(Box::new(AnimationState::new(animation_id)));
                    }
                    StateSpec::BlendState { children } => {
                        objects.push(Box::new(BlendState));
                        if let Some(children) = children {
                            for child in children {
                                append_blend_state_child(child, objects);
                            }
                        }
                    }
                    StateSpec::BlendStateDirect { children } => {
                        objects.push(Box::new(BlendStateDirect));
                        if let Some(children) = children {
                            for child in children {
                                append_blend_state_direct_child(child, objects);
                            }
                        }
                    }
                    StateSpec::BlendState1d { input_id, children } => {
                        objects.push(Box::new(BlendState1D));
                        if let Some(input_id) = input_id {
                            objects.push(Box::new(BlendState1DInput {
                                input_id: *input_id,
                            }));
                        }
                        if let Some(children) = children {
                            for child in children {
                                append_blend_state_1d_child(child, objects);
                            }
                        }
                    }
                }

                if let Some(transitions) = &layer.transitions {
                    for transition in transitions {
                        if transition.from != user_idx {
                            continue;
                        }
                        let state_to_id =
                            *user_to_final.get(transition.to).ok_or_else(|| {
                                format!(
                                    "transition target index {} out of bounds (layer has {} states)",
                                    transition.to,
                                    user_to_final.len()
                                )
                            })? as u64;
                        let mut state_transition = StateTransition::new(state_to_id);
                        if let Some(duration) = transition.duration {
                            state_transition.duration = duration;
                        }
                        objects.push(Box::new(state_transition));

                        if let Some(conditions) = &transition.conditions {
                            for condition in conditions {
                                let input_index = *input_name_to_index
                                    .get(&condition.input)
                                    .ok_or_else(|| {
                                        format!(
                                            "unknown input referenced in condition: '{}'",
                                            condition.input
                                        )
                                    })?;
                                {
                                    let input_id = input_index as u64;
                                    let op = condition
                                        .op
                                        .as_deref()
                                        .map(parse_condition_op)
                                        .unwrap_or(0);
                                    match condition.value.as_ref() {
                                        Some(serde_json::Value::Number(_)) => {
                                            let value = condition
                                                .value
                                                .as_ref()
                                                .and_then(json_value_to_f32)
                                                .ok_or_else(|| {
                                                    format!(
                                                        "invalid numeric condition value for input '{}'",
                                                        condition.input
                                                    )
                                                })?;
                                            objects.push(Box::new(
                                                TransitionNumberCondition::new(
                                                    input_id, op, value,
                                                ),
                                            ));
                                        }
                                        Some(serde_json::Value::Bool(v)) => {
                                            let bool_op = if condition.op.is_some() {
                                                condition
                                                    .op
                                                    .as_deref()
                                                    .map(parse_condition_op)
                                                    .unwrap_or(0)
                                            } else if *v {
                                                0 // equal: true when input is true
                                            } else {
                                                1 // notEqual: true when input is false
                                            };
                                            objects.push(Box::new(
                                                TransitionBoolCondition::new(
                                                    input_id, bool_op,
                                                ),
                                            ));
                                        }
                                        _ => {
                                            if condition.op.is_some() {
                                                objects.push(Box::new(
                                                    TransitionValueCondition {
                                                        input_id,
                                                        op,
                                                    },
                                                ));
                                            } else if input_is_trigger(
                                                &condition.input,
                                                state_machine.inputs.as_ref(),
                                            ) {
                                                objects.push(Box::new(
                                                    TransitionTriggerCondition { input_id },
                                                ));
                                            } else {
                                                objects.push(Box::new(
                                                    TransitionInputCondition { input_id },
                                                ));
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        if let Some(children) = &transition.children {
                            for child in children {
                                append_transition_child(child, objects)?;
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

fn append_blend_state_child(spec: &BlendStateChildSpec, objects: &mut Vec<Box<dyn RiveObject>>) {
    let BlendStateChildSpec::BlendAnimation { animation_id } = spec;
    objects.push(Box::new(BlendAnimation {
        animation_id: *animation_id,
    }));
}

fn append_blend_state_direct_child(
    spec: &BlendStateDirectChildSpec,
    objects: &mut Vec<Box<dyn RiveObject>>,
) {
    let BlendStateDirectChildSpec::BlendAnimationDirect {
        animation_id,
        input_id,
        mix_value,
        blend_source,
    } = spec;
    objects.push(Box::new(BlendAnimationDirect {
        animation_id: *animation_id,
        input_id: input_id.unwrap_or(u32::MAX as u64),
        mix_value: mix_value.unwrap_or(100.0),
        blend_source: blend_source.unwrap_or(0),
    }));
}

fn append_blend_state_1d_child(
    spec: &BlendState1DChildSpec,
    objects: &mut Vec<Box<dyn RiveObject>>,
) {
    let BlendState1DChildSpec::BlendAnimation1D {
        animation_id,
        value,
    } = spec;
    objects.push(Box::new(BlendAnimation1D {
        animation_id: *animation_id,
        value: value.unwrap_or(0.0),
    }));
}

fn append_transition_child(
    spec: &TransitionChildSpec,
    objects: &mut Vec<Box<dyn RiveObject>>,
) -> Result<(), String> {
    match spec {
        TransitionChildSpec::TransitionPropertyComparator => {
            objects.push(Box::new(TransitionPropertyComparator));
        }
        TransitionChildSpec::TransitionViewModelCondition { op_value } => {
            objects.push(Box::new(TransitionViewModelCondition {
                op_value: op_value.unwrap_or(0),
            }));
        }
        TransitionChildSpec::TransitionValueBooleanComparator { value } => {
            objects.push(Box::new(TransitionValueBooleanComparator { value: *value }));
        }
        TransitionChildSpec::TransitionValueColorComparator { value } => {
            let color = parse_color(value)?;
            objects.push(Box::new(TransitionValueColorComparator { value: color }));
        }
        TransitionChildSpec::TransitionValueNumberComparator { value } => {
            objects.push(Box::new(TransitionValueNumberComparator { value: *value }));
        }
        TransitionChildSpec::TransitionValueEnumComparator => {
            objects.push(Box::new(TransitionValueEnumComparator));
        }
        TransitionChildSpec::TransitionValueStringComparator { value } => {
            objects.push(Box::new(TransitionValueStringComparator {
                value: value.clone(),
            }));
        }
        TransitionChildSpec::TransitionValueTriggerComparator { value } => {
            objects.push(Box::new(TransitionValueTriggerComparator {
                value: value.unwrap_or(0),
            }));
        }
    }
    Ok(())
}
