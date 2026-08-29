use std::collections::HashMap;

use crate::objects::core::{RiveObject, property_keys};
use crate::objects::data_binding::{BindablePropertyBoolean, DataBindContext};
use crate::objects::state_machine::{
    AnimationState, AnyState, BlendAnimation, BlendAnimation1D, BlendAnimationDirect, BlendState,
    BlendState1DInput, BlendState1DViewModel, BlendStateDirect, EntryState, ExitState,
    ListenerAlignTarget, ListenerBoolChange, ListenerFireEvent, ListenerNumberChange,
    ListenerTriggerChange, ListenerViewModelChange, StateMachine, StateMachineBool,
    StateMachineComponentNestedArtboard, StateMachineFireAction, StateMachineFireEvent,
    StateMachineFireTrigger, StateMachineLayer, StateMachineListener, StateMachineNestedInput,
    StateMachineNumber, StateMachineTrigger, StateTransition, TransitionArtboardCondition,
    TransitionBoolCondition, TransitionInputCondition, TransitionNumberCondition,
    TransitionPropertyArtboardComparator, TransitionPropertyComparator,
    TransitionPropertyViewModelComparator, TransitionSelfComparator, TransitionTriggerCondition,
    TransitionValueArtboardComparator, TransitionValueAssetComparator,
    TransitionValueBooleanComparator, TransitionValueColorComparator, TransitionValueCondition,
    TransitionValueEnumComparator, TransitionValueIdComparator, TransitionValueNumberComparator,
    TransitionValueStringComparator, TransitionValueTriggerComparator,
    TransitionViewModelCondition,
};

use super::parsers::{
    input_is_trigger, json_value_to_f32, parse_color, parse_condition_op, parse_listener_type,
};
use super::references::{self, Namespace};
use super::spec::{
    BlendState1DChildSpec, BlendStateChildSpec, BlendStateDirectChildSpec, InputSpec,
    ListenerActionSpec, ObjectSpec, StateMachineComponentSpec, StateMachineSpec, StateSpec,
    TransitionChildSpec,
};

fn encode_id_path(ids: &[u64]) -> Vec<u8> {
    let mut output = Vec::new();
    for &id in ids {
        let mut value = id;
        loop {
            let mut byte = (value & 0x7f) as u8;
            value >>= 7;
            if value != 0 {
                byte |= 0x80;
            }
            output.push(byte);
            if value == 0 {
                break;
            }
        }
    }
    output
}

fn view_model_property_name(object: &ObjectSpec) -> Option<&str> {
    match object {
        ObjectSpec::ViewModelProperty { name, .. }
        | ObjectSpec::ViewModelPropertyNumber { name, .. }
        | ObjectSpec::ViewModelPropertyBoolean { name, .. }
        | ObjectSpec::ViewModelPropertyString { name, .. }
        | ObjectSpec::ViewModelPropertyColor { name, .. }
        | ObjectSpec::ViewModelPropertyList { name, .. }
        | ObjectSpec::ViewModelPropertyViewModel { name, .. }
        | ObjectSpec::ViewModelPropertyEnum { name, .. }
        | ObjectSpec::ViewModelPropertyEnumCustom { name, .. }
        | ObjectSpec::ViewModelPropertyEnumSystem { name, .. }
        | ObjectSpec::ViewModelPropertyTrigger { name, .. }
        | ObjectSpec::ViewModelPropertyAssetImage { name, .. }
        | ObjectSpec::ViewModelPropertyArtboard { name, .. }
        | ObjectSpec::ViewModelPropertySymbol { name, .. }
        | ObjectSpec::ViewModelPropertySymbolListIndex { name, .. } => Some(name),
        _ => None,
    }
}

fn resolve_view_model_binding_ids(
    artboard_children: &[ObjectSpec],
    view_model_id_base: u64,
    view_model_name: &str,
    property_name: &str,
) -> Option<(u64, u64)> {
    let mut view_model_id = view_model_id_base;
    for child in artboard_children {
        let ObjectSpec::ViewModel { name, children } = child else {
            continue;
        };
        if name == view_model_name {
            let mut property_id = 0u64;
            for property in children.as_deref().unwrap_or_default() {
                if let Some(name) = view_model_property_name(property) {
                    if name == property_name {
                        return Some((view_model_id, property_id));
                    }
                    property_id += 1;
                }
            }
            return None;
        }
        view_model_id += 1;
    }
    None
}

/// Builds all state machine objects for an artboard.
pub(crate) fn build_state_machines(
    state_machines: &[StateMachineSpec],
    artboard_start: usize,
    objects: &mut Vec<Box<dyn RiveObject>>,
    object_name_to_index: &HashMap<String, usize>,
    animation_name_to_index: &HashMap<String, usize>,
    artboard_children: &[ObjectSpec],
    view_model_id_base: u64,
) -> Result<(), String> {
    for state_machine in state_machines {
        objects.push(Box::new(StateMachine::new(state_machine.name.clone())));

        let mut input_name_to_index: HashMap<String, usize> = HashMap::new();
        let mut bound_bool_input_paths: HashMap<String, (u64, u64)> = HashMap::new();
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
                    InputSpec::Bool {
                        name,
                        value,
                        view_model_binding,
                    } => {
                        objects.push(Box::new(StateMachineBool {
                            name: name.clone(),
                            value: if *value { 1 } else { 0 },
                        }));
                        if let Some(binding) = view_model_binding {
                            let (view_model_id, property_id) =
                                resolve_view_model_binding_ids(
                                    artboard_children,
                                    view_model_id_base,
                                    &binding.view_model,
                                    &binding.property,
                                )
                                .ok_or_else(|| {
                                    format!(
                                        "unknown view-model binding referenced by bool input '{}': '{}.{}'",
                                        name, binding.view_model, binding.property
                                    )
                                })?;
                            bound_bool_input_paths
                                .insert(name.clone(), (view_model_id, property_id));
                        }
                        input_name_to_index.insert(name.clone(), input_index);
                    }
                    InputSpec::Trigger { name } => {
                        objects.push(Box::new(StateMachineTrigger { name: name.clone() }));
                        input_name_to_index.insert(name.clone(), input_index);
                    }
                }
            }
        }

        if let Some(components) = &state_machine.components {
            for component in components {
                match component {
                    StateMachineComponentSpec::FireEvent {
                        name,
                        event_id,
                        occurs_value,
                    } => {
                        objects.push(Box::new(StateMachineFireEvent {
                            name: name.clone(),
                            event_id: event_id.unwrap_or(0),
                            occurs_value: occurs_value.unwrap_or(0),
                        }));
                    }
                    StateMachineComponentSpec::FireTrigger { name } => {
                        objects.push(Box::new(StateMachineFireTrigger { name: name.clone() }));
                    }
                    StateMachineComponentSpec::FireAction { name } => {
                        objects.push(Box::new(StateMachineFireAction { name: name.clone() }));
                    }
                    StateMachineComponentSpec::NestedArtboard { name, artboard_id } => {
                        objects.push(Box::new(StateMachineComponentNestedArtboard {
                            name: name.clone(),
                            artboard_id: artboard_id.unwrap_or(0),
                        }));
                    }
                    StateMachineComponentSpec::NestedInput {
                        name,
                        nested_input_id,
                    } => {
                        objects.push(Box::new(StateMachineNestedInput {
                            name: name.clone(),
                            nested_input_id: nested_input_id.unwrap_or(0),
                        }));
                    }
                    StateMachineComponentSpec::BlendState1DViewModel => {
                        objects.push(Box::new(BlendState1DViewModel));
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
                let listener_type_value = match (
                    &listener.listener_type,
                    listener.listener_type_value,
                ) {
                    (Some(_), Some(_)) => {
                        return Err(format!(
                            "state machine listener on '{}' sets both 'listener_type' and 'listener_type_value'; use one or the other",
                            listener.target
                        ));
                    }
                    (Some(name), None) => parse_listener_type(name)?,
                    (None, Some(value)) => value,
                    (None, None) => 0,
                };
                objects.push(Box::new(StateMachineListener {
                    target_id: listener_target_id,
                    listener_type_value,
                }));

                if let Some(actions) = &listener.actions {
                    for action in actions {
                        match action {
                            ListenerActionSpec::BoolChange { input, value } => {
                                let input_index =
                                    *input_name_to_index.get(input).ok_or_else(|| {
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
                                let input_index =
                                    *input_name_to_index.get(input).ok_or_else(|| {
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
                                let input_index =
                                    *input_name_to_index.get(input).ok_or_else(|| {
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
                            ListenerActionSpec::AlignTarget { target_id } => {
                                objects.push(Box::new(ListenerAlignTarget {
                                    target_id: target_id.unwrap_or(0),
                                }));
                            }
                            ListenerActionSpec::FireEvent { event_id } => {
                                objects.push(Box::new(ListenerFireEvent {
                                    event_id: event_id.unwrap_or(0),
                                }));
                            }
                            ListenerActionSpec::ViewModelChange {
                                view_model_property_id,
                            } => {
                                objects.push(Box::new(ListenerViewModelChange {
                                    view_model_property_id: view_model_property_id.unwrap_or(0),
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
                    StateSpec::BlendState1d {
                        input_id,
                        input,
                        children,
                    } => {
                        let lookup = |name: &str| input_name_to_index.get(name).map(|i| *i as u64);
                        let input_id = references::require(
                            "blend_state1d",
                            &Namespace {
                                kind: "input",
                                name_field: "input",
                                index_field: "input_id",
                                lookup: &lookup,
                                check: None,
                            },
                            input.as_deref(),
                            *input_id,
                        )?;
                        objects.push(Box::new(BlendState1DInput { input_id }));
                        if let Some(children) = children {
                            for child in children {
                                append_blend_state_1d_child(
                                    child,
                                    animation_name_to_index,
                                    objects,
                                )?;
                            }
                        }
                    }
                }

                if let Some(transitions) = &layer.transitions {
                    for transition in transitions {
                        if transition.from != user_idx {
                            continue;
                        }
                        let state_to_id = transition.to as u64;
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
                                            objects.push(Box::new(TransitionNumberCondition::new(
                                                input_id, op, value,
                                            )));
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
                                            if let Some(&(view_model_id, property_id)) =
                                                bound_bool_input_paths.get(&condition.input)
                                            {
                                                let vm_op = condition
                                                    .op
                                                    .as_deref()
                                                    .map(parse_condition_op)
                                                    .unwrap_or(0);
                                                objects.push(Box::new(
                                                    TransitionViewModelCondition {
                                                        op_value: vm_op,
                                                    },
                                                ));
                                                objects.push(Box::new(BindablePropertyBoolean {
                                                    property_value: 0,
                                                }));
                                                objects.push(Box::new(DataBindContext::new(
                                                    property_keys::BINDABLE_PROPERTY_BOOLEAN_VALUE
                                                        as u64,
                                                    0,
                                                    encode_id_path(&[view_model_id, property_id]),
                                                )));
                                                objects.push(Box::new(
                                                    TransitionPropertyViewModelComparator,
                                                ));
                                                objects.push(Box::new(
                                                    TransitionValueBooleanComparator { value: *v },
                                                ));
                                            } else {
                                                objects.push(Box::new(
                                                    TransitionBoolCondition::new(input_id, bool_op),
                                                ));
                                            }
                                        }
                                        _ => {
                                            if condition.op.is_some() {
                                                objects.push(Box::new(TransitionValueCondition {
                                                    input_id,
                                                    op,
                                                }));
                                            } else if input_is_trigger(
                                                &condition.input,
                                                state_machine.inputs.as_ref(),
                                            ) {
                                                objects.push(Box::new(
                                                    TransitionTriggerCondition { input_id },
                                                ));
                                            } else {
                                                objects.push(Box::new(TransitionInputCondition {
                                                    input_id,
                                                }));
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
            if !has_any {
                objects.push(Box::new(AnyState));
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
    animation_name_to_index: &HashMap<String, usize>,
    objects: &mut Vec<Box<dyn RiveObject>>,
) -> Result<(), String> {
    let BlendState1DChildSpec::BlendAnimation1D {
        animation_id,
        animation,
        value,
    } = spec;
    let lookup = |name: &str| animation_name_to_index.get(name).map(|i| *i as u64);
    let animation_id = references::require(
        "blend_animation1_d",
        &Namespace {
            kind: "animation",
            name_field: "animation",
            index_field: "animation_id",
            lookup: &lookup,
            check: None,
        },
        animation.as_deref(),
        *animation_id,
    )?;
    objects.push(Box::new(BlendAnimation1D {
        animation_id,
        value: value.unwrap_or(0.0),
    }));
    Ok(())
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
        TransitionChildSpec::TransitionPropertyViewModelComparator => {
            objects.push(Box::new(TransitionPropertyViewModelComparator));
        }
        TransitionChildSpec::TransitionPropertyArtboardComparator => {
            objects.push(Box::new(TransitionPropertyArtboardComparator));
        }
        TransitionChildSpec::TransitionArtboardCondition { op_value } => {
            objects.push(Box::new(TransitionArtboardCondition {
                op_value: op_value.unwrap_or(0),
            }));
        }
        TransitionChildSpec::TransitionSelfComparator => {
            objects.push(Box::new(TransitionSelfComparator));
        }
        TransitionChildSpec::TransitionValueIdComparator { value } => {
            objects.push(Box::new(TransitionValueIdComparator {
                value: value.unwrap_or(0),
            }));
        }
        TransitionChildSpec::TransitionValueAssetComparator { value } => {
            objects.push(Box::new(TransitionValueAssetComparator {
                value: value.unwrap_or(0),
            }));
        }
        TransitionChildSpec::TransitionValueArtboardComparator { value } => {
            objects.push(Box::new(TransitionValueArtboardComparator {
                value: value.unwrap_or(0),
            }));
        }
    }
    Ok(())
}
