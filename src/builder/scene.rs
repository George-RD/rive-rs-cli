use std::collections::{HashMap, HashSet};

use serde::Deserialize;

use crate::objects::animation::{
    CubicEaseInterpolator, KeyFrameColor, KeyFrameDouble, KeyedObject, KeyedProperty,
    LinearAnimation,
};
use crate::objects::artboard::{Artboard, Backboard};
use crate::objects::core::{property_keys, RiveObject};
use crate::objects::shapes::{
    Ellipse, Fill, GradientStop, LinearGradient, Node, PathObject, RadialGradient, Rectangle,
    Shape, SolidColor, Stroke, TrimPath,
};
use crate::objects::state_machine::{
    AnimationState, AnyState, EntryState, ExitState, StateMachine, StateMachineBool,
    StateMachineLayer, StateMachineNumber, StateMachineTrigger, StateTransition,
    TransitionBoolCondition, TransitionInputCondition, TransitionNumberCondition,
    TransitionTriggerCondition, TransitionValueCondition,
};

const SCENE_FORMAT_VERSION: u32 = 1;

#[derive(Debug, Deserialize)]
pub struct SceneSpec {
    pub scene_format_version: u32,
    pub artboard: ArtboardSpec,
}

#[derive(Debug, Deserialize)]
pub struct ArtboardSpec {
    pub name: String,
    pub width: f32,
    pub height: f32,
    pub children: Vec<ObjectSpec>,
    pub animations: Option<Vec<AnimationSpec>>,
    pub state_machines: Option<Vec<StateMachineSpec>>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ObjectSpec {
    Shape {
        name: String,
        x: Option<f32>,
        y: Option<f32>,
        children: Option<Vec<ObjectSpec>>,
    },
    Ellipse {
        name: String,
        width: f32,
        height: f32,
        origin_x: Option<f32>,
        origin_y: Option<f32>,
    },
    Rectangle {
        name: String,
        width: f32,
        height: f32,
        corner_radius: Option<f32>,
        origin_x: Option<f32>,
        origin_y: Option<f32>,
    },
    Fill {
        name: String,
        fill_rule: Option<u64>,
        children: Option<Vec<ObjectSpec>>,
    },
    Stroke {
        name: String,
        thickness: Option<f32>,
        cap: Option<u64>,
        join: Option<u64>,
        children: Option<Vec<ObjectSpec>>,
    },
    SolidColor {
        name: String,
        color: String,
    },
    LinearGradient {
        name: String,
        start_x: f32,
        start_y: f32,
        end_x: f32,
        end_y: f32,
        children: Option<Vec<ObjectSpec>>,
    },
    RadialGradient {
        name: String,
        start_x: f32,
        start_y: f32,
        end_x: f32,
        end_y: f32,
        children: Option<Vec<ObjectSpec>>,
    },
    GradientStop {
        name: Option<String>,
        color: String,
        position: f32,
    },
    Node {
        name: String,
        x: Option<f32>,
        y: Option<f32>,
    },
    Path {
        name: String,
        path_flags: Option<u64>,
    },
    TrimPath {
        name: String,
        start: Option<f32>,
        end: Option<f32>,
        offset: Option<f32>,
        mode: Option<u64>,
    },
}

#[derive(Debug, Deserialize)]
pub struct InterpolatorSpec {
    pub name: String,
    pub x1: Option<f32>,
    pub y1: Option<f32>,
    pub x2: Option<f32>,
    pub y2: Option<f32>,
}

#[derive(Debug, Deserialize)]
pub struct AnimationSpec {
    pub name: String,
    pub fps: u64,
    pub duration: u64,
    pub speed: Option<f32>,
    pub loop_type: Option<u64>,
    pub interpolators: Option<Vec<InterpolatorSpec>>,
    pub keyframes: Vec<KeyframeGroupSpec>,
}

#[derive(Debug, Deserialize)]
pub struct KeyframeGroupSpec {
    pub object: String,
    pub property: String,
    pub frames: Vec<KeyframeSpec>,
}

#[derive(Debug, Deserialize)]
pub struct KeyframeSpec {
    pub frame: u64,
    pub value: serde_json::Value,
    pub interpolation: Option<String>,
    pub interpolator: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct StateMachineSpec {
    pub name: String,
    pub inputs: Option<Vec<InputSpec>>,
    pub layers: Vec<LayerSpec>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum InputSpec {
    Number { name: String, value: f32 },
    Bool { name: String, value: bool },
    Trigger { name: String },
}

#[derive(Debug, Deserialize)]
pub struct LayerSpec {
    pub states: Vec<StateSpec>,
    pub transitions: Option<Vec<TransitionSpec>>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum StateSpec {
    Entry,
    Exit,
    Any,
    Animation { animation: String },
}

#[derive(Debug, Deserialize)]
pub struct TransitionSpec {
    pub from: usize,
    pub to: usize,
    pub duration: Option<u64>,
    pub conditions: Option<Vec<ConditionSpec>>,
}

#[derive(Debug, Deserialize)]
pub struct ConditionSpec {
    pub input: String,
    pub op: Option<String>,
    pub value: Option<serde_json::Value>,
}

pub fn build_scene(spec: &SceneSpec) -> Result<Vec<Box<dyn RiveObject>>, String> {
    validate_scene_spec(spec)?;

    let mut objects: Vec<Box<dyn RiveObject>> = Vec::new();
    let mut object_name_to_index: HashMap<String, usize> = HashMap::new();
    let mut animation_name_to_index: HashMap<String, usize> = HashMap::new();

    objects.push(Box::new(Backboard));
    let mut artboard = Artboard::new(
        spec.artboard.name.clone(),
        spec.artboard.width,
        spec.artboard.height,
    );
    if spec
        .artboard
        .state_machines
        .as_ref()
        .is_some_and(|sms| !sms.is_empty())
    {
        artboard.default_state_machine_id = Some(0);
    }
    objects.push(Box::new(artboard));

    for child in &spec.artboard.children {
        append_object(child, 1, &mut objects, &mut object_name_to_index)?;
    }

    let mut interpolator_name_to_index: HashMap<String, usize> = HashMap::new();
    let mut interpolator_control_points: HashMap<String, (f32, f32, f32, f32)> = HashMap::new();

    if let Some(animations) = &spec.artboard.animations {
        for animation in animations {
            if let Some(interpolators) = &animation.interpolators {
                for interp in interpolators {
                    let x1 = interp.x1.unwrap_or(0.42);
                    let y1 = interp.y1.unwrap_or(0.0);
                    let x2 = interp.x2.unwrap_or(0.58);
                    let y2 = interp.y2.unwrap_or(1.0);

                    if let Some((stored_x1, stored_y1, stored_x2, stored_y2)) =
                        interpolator_control_points.get(&interp.name)
                    {
                        if (stored_x1, stored_y1, stored_x2, stored_y2) != (&x1, &y1, &x2, &y2) {
                            return Err(format!(
                                "duplicate interpolator '{}' with different control points",
                                interp.name
                            ));
                        }
                        continue;
                    }

                    let artboard_local_index = objects.len() - 1;
                    interpolator_name_to_index.insert(interp.name.clone(), artboard_local_index);
                    interpolator_control_points.insert(interp.name.clone(), (x1, y1, x2, y2));
                    objects.push(Box::new(CubicEaseInterpolator::new(x1, y1, x2, y2)));
                }
            }
        }

        for (animation_list_index, animation) in animations.iter().enumerate() {
            let mut linear =
                LinearAnimation::new(animation.name.clone(), animation.fps, animation.duration);
            if let Some(speed) = animation.speed {
                linear.speed = speed;
            }
            if let Some(loop_type) = animation.loop_type {
                linear.loop_type = loop_type;
            }

            objects.push(Box::new(linear));
            animation_name_to_index.insert(animation.name.clone(), animation_list_index);

            for group in &animation.keyframes {
                let object_index = *object_name_to_index.get(&group.object).ok_or_else(|| {
                    format!("unknown object referenced in keyframes: '{}'", group.object)
                })?;
                objects.push(Box::new(KeyedObject {
                    object_id: (object_index - 1) as u64,
                }));

                let property_key = property_key_from_name(&group.property).ok_or_else(|| {
                    format!(
                        "unknown property referenced in keyframes: '{}'",
                        group.property
                    )
                })?;
                objects.push(Box::new(KeyedProperty {
                    property_key: property_key as u64,
                }));

                for frame in &group.frames {
                    let interp_type = match &frame.interpolation {
                        Some(name) => interpolation_type_from_name(name)?,
                        None => 1,
                    };
                    let interp_id = match &frame.interpolator {
                        Some(name) => {
                            let idx = *interpolator_name_to_index.get(name).ok_or_else(|| {
                                format!("unknown interpolator referenced: '{}'", name)
                            })?;
                            idx as u64
                        }
                        None => u32::MAX as u64,
                    };

                    if property_key == property_keys::SOLID_COLOR_VALUE {
                        let color = json_value_to_color(&frame.value).ok_or_else(|| {
                            format!(
                                "invalid color keyframe value for object '{}' property '{}' at frame {}",
                                group.object, group.property, frame.frame
                            )
                        })?;
                        let mut kf = KeyFrameColor::new(frame.frame, color);
                        kf.interpolation_type = interp_type;
                        kf.interpolator_id = interp_id;
                        objects.push(Box::new(kf));
                    } else {
                        let value = json_value_to_f32(&frame.value).ok_or_else(|| {
                            format!(
                                "invalid numeric keyframe value for object '{}' property '{}' at frame {}",
                                group.object, group.property, frame.frame
                            )
                        })?;
                        let mut kf = KeyFrameDouble::new(frame.frame, value);
                        kf.interpolation_type = interp_type;
                        kf.interpolator_id = interp_id;
                        objects.push(Box::new(kf));
                    }
                }
            }
        }
    }

    if let Some(state_machines) = &spec.artboard.state_machines {
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
                    }

                    if let Some(transitions) = &layer.transitions {
                        for transition in transitions {
                            if transition.from != user_idx {
                                continue;
                            }
                            let state_to_id = *user_to_final.get(transition.to).ok_or_else(|| {
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
                                            Some(serde_json::Value::Bool(_v)) => {
                                                let bool_op = condition
                                                    .op
                                                    .as_deref()
                                                    .map(parse_condition_op)
                                                    .unwrap_or(0);
                                                objects.push(Box::new(
                                                    TransitionBoolCondition::new(input_id, bool_op),
                                                ));
                                            }
                                            _ => {
                                                if condition.op.is_some() {
                                                    objects.push(Box::new(
                                                        TransitionValueCondition { input_id, op },
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
                        }
                    }
                }
            }
        }
    }

    Ok(objects)
}

fn append_object(
    spec: &ObjectSpec,
    parent_index: usize,
    objects: &mut Vec<Box<dyn RiveObject>>,
    name_to_index: &mut HashMap<String, usize>,
) -> Result<(), String> {
    let object_index = objects.len();
    let parent_id = (parent_index - 1) as u64;

    match spec {
        ObjectSpec::Shape {
            name,
            x,
            y,
            children,
        } => {
            let mut shape = Shape::new(name.clone(), parent_id);
            if let Some(x) = x {
                shape.x = *x;
            }
            if let Some(y) = y {
                shape.y = *y;
            }
            objects.push(Box::new(shape));
            name_to_index.insert(name.clone(), object_index);
            if let Some(children) = children {
                for child in children {
                    append_object(child, object_index, objects, name_to_index)?;
                }
            }
        }
        ObjectSpec::Ellipse {
            name,
            width,
            height,
            origin_x,
            origin_y,
        } => {
            let mut ellipse = Ellipse::new(name.clone(), parent_id, *width, *height);
            if let Some(origin_x) = origin_x {
                ellipse.origin_x = *origin_x;
            }
            if let Some(origin_y) = origin_y {
                ellipse.origin_y = *origin_y;
            }
            objects.push(Box::new(ellipse));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::Rectangle {
            name,
            width,
            height,
            corner_radius,
            origin_x,
            origin_y,
        } => {
            let mut rectangle = Rectangle::new(name.clone(), parent_id, *width, *height);
            if let Some(origin_x) = origin_x {
                rectangle.origin_x = *origin_x;
            }
            if let Some(origin_y) = origin_y {
                rectangle.origin_y = *origin_y;
            }
            if let Some(corner_radius) = corner_radius {
                rectangle.corner_radius_tl = *corner_radius;
                rectangle.corner_radius_tr = *corner_radius;
                rectangle.corner_radius_bl = *corner_radius;
                rectangle.corner_radius_br = *corner_radius;
                rectangle.link_corner_radius = 1;
            }
            objects.push(Box::new(rectangle));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::Fill {
            name,
            fill_rule,
            children,
        } => {
            let mut fill = Fill::new(name.clone(), parent_id);
            if let Some(fill_rule) = fill_rule {
                fill.fill_rule = *fill_rule;
            }
            objects.push(Box::new(fill));
            name_to_index.insert(name.clone(), object_index);
            if let Some(children) = children {
                for child in children {
                    append_object(child, object_index, objects, name_to_index)?;
                }
            }
        }
        ObjectSpec::Stroke {
            name,
            thickness,
            cap,
            join,
            children,
        } => {
            let mut stroke = Stroke::new(name.clone(), parent_id, thickness.unwrap_or(1.0));
            if let Some(cap) = cap {
                stroke.cap = *cap;
            }
            if let Some(join) = join {
                stroke.join = *join;
            }
            objects.push(Box::new(stroke));
            name_to_index.insert(name.clone(), object_index);
            if let Some(children) = children {
                for child in children {
                    append_object(child, object_index, objects, name_to_index)?;
                }
            }
        }
        ObjectSpec::SolidColor { name, color } => {
            let color_value = parse_color(color)?;
            objects.push(Box::new(SolidColor::new(
                name.clone(),
                parent_id,
                color_value,
            )));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::LinearGradient {
            name,
            start_x,
            start_y,
            end_x,
            end_y,
            children,
        } => {
            objects.push(Box::new(LinearGradient {
                name: name.clone(),
                parent_id,
                start_x: *start_x,
                start_y: *start_y,
                end_x: *end_x,
                end_y: *end_y,
                opacity: 1.0,
            }));
            name_to_index.insert(name.clone(), object_index);
            if let Some(children) = children {
                for child in children {
                    append_object(child, object_index, objects, name_to_index)?;
                }
            }
        }
        ObjectSpec::RadialGradient {
            name,
            start_x,
            start_y,
            end_x,
            end_y,
            children,
        } => {
            objects.push(Box::new(RadialGradient {
                name: name.clone(),
                parent_id,
                start_x: *start_x,
                start_y: *start_y,
                end_x: *end_x,
                end_y: *end_y,
                opacity: 1.0,
            }));
            name_to_index.insert(name.clone(), object_index);
            if let Some(children) = children {
                for child in children {
                    append_object(child, object_index, objects, name_to_index)?;
                }
            }
        }
        ObjectSpec::GradientStop {
            name,
            color,
            position,
        } => {
            let generated_name = name
                .clone()
                .unwrap_or_else(|| format!("gradient_stop_{}", name_to_index.len()));
            objects.push(Box::new(GradientStop {
                name: generated_name.clone(),
                parent_id,
                color: parse_color(color)?,
                position: *position,
            }));
            name_to_index.insert(generated_name, object_index);
        }
        ObjectSpec::Node { name, x, y } => {
            objects.push(Box::new(Node {
                name: name.clone(),
                parent_id,
                x: x.unwrap_or(0.0),
                y: y.unwrap_or(0.0),
            }));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::Path { name, path_flags } => {
            objects.push(Box::new(PathObject {
                name: name.clone(),
                parent_id,
                path_flags: path_flags.unwrap_or(0),
            }));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::TrimPath {
            name,
            start,
            end,
            offset,
            mode,
        } => {
            let mut trim_path = TrimPath::new(name.clone(), parent_id);
            if let Some(start) = start {
                trim_path.start = *start;
            }
            if let Some(end) = end {
                trim_path.end = *end;
            }
            if let Some(offset) = offset {
                trim_path.offset = *offset;
            }
            if let Some(mode) = mode {
                trim_path.set_mode(*mode).map_err(|e| format!("trim_path '{}': {}", name, e))?;
            }
            objects.push(Box::new(trim_path));
            name_to_index.insert(name.clone(), object_index);
        }
    }

    Ok(())
}

fn property_key_from_name(name: &str) -> Option<u16> {
    match name {
        "x" => Some(property_keys::NODE_X),
        "y" => Some(property_keys::NODE_Y),
        "rotation" => Some(property_keys::TRANSFORM_ROTATION),
        "scale_x" => Some(property_keys::TRANSFORM_SCALE_X),
        "scale_y" => Some(property_keys::TRANSFORM_SCALE_Y),
        "opacity" => Some(property_keys::WORLD_TRANSFORM_OPACITY),
        "width" => Some(property_keys::PARAMETRIC_PATH_WIDTH),
        "height" => Some(property_keys::PARAMETRIC_PATH_HEIGHT),
        "color" => Some(property_keys::SOLID_COLOR_VALUE),
        "trim_start" => Some(property_keys::TRIM_PATH_START),
        "trim_end" => Some(property_keys::TRIM_PATH_END),
        "trim_offset" => Some(property_keys::TRIM_PATH_OFFSET),
        _ => None,
    }
}

fn interpolation_type_from_name(name: &str) -> Result<u64, String> {
    match name {
        "hold" => Ok(0),
        "linear" => Ok(1),
        "cubic" => Ok(2),
        _ => Err(format!("unknown interpolation type: '{}'", name)),
    }
}

fn parse_color(color: &str) -> Result<u32, String> {
    let hex = color.trim_start_matches('#');
    if hex.len() == 8 {
        return u32::from_str_radix(hex, 16)
            .map_err(|_| format!("invalid 8-digit color literal: '{}'", color));
    }

    if hex.len() == 6 {
        return u32::from_str_radix(hex, 16)
            .map(|rgb| 0xFF00_0000 | rgb)
            .map_err(|_| format!("invalid 6-digit color literal: '{}'", color));
    }

    Err(format!(
        "invalid color literal '{}' (expected 6 or 8 hex digits)",
        color
    ))
}

fn json_value_to_f32(value: &serde_json::Value) -> Option<f32> {
    match value {
        serde_json::Value::Number(number) => number.as_f64().map(|v| v as f32),
        _ => None,
    }
}

fn json_value_to_color(value: &serde_json::Value) -> Option<u32> {
    match value {
        serde_json::Value::String(s) => parse_color(s).ok(),
        serde_json::Value::Number(n) => n
            .as_u64()
            .filter(|&v| v <= u32::MAX as u64)
            .map(|v| v as u32),
        _ => None,
    }
}

fn validate_scene_spec(spec: &SceneSpec) -> Result<(), String> {
    if spec.scene_format_version != SCENE_FORMAT_VERSION {
        return Err(format!(
            "unsupported scene_format_version {} (expected {})",
            spec.scene_format_version, SCENE_FORMAT_VERSION
        ));
    }

    if spec.artboard.width < 0.0 {
        return Err("artboard width must be non-negative".to_string());
    }
    if spec.artboard.height < 0.0 {
        return Err("artboard height must be non-negative".to_string());
    }

    let mut object_names: HashSet<String> = HashSet::new();
    for child in &spec.artboard.children {
        validate_object_spec(child, &mut object_names)?;
    }

    let mut animation_names: HashSet<String> = HashSet::new();
    if let Some(animations) = &spec.artboard.animations {
        for animation in animations {
            if animation.duration == 0 {
                return Err(format!(
                    "animation '{}' duration must be greater than 0",
                    animation.name
                ));
            }
            if animation_names.contains(&animation.name) {
                return Err(format!("duplicate animation name '{}'", animation.name));
            }
            animation_names.insert(animation.name.clone());

            let mut interp_names: HashSet<String> = HashSet::new();
            if let Some(interpolators) = &animation.interpolators {
                for interp in interpolators {
                    if interp_names.contains(&interp.name) {
                        return Err(format!(
                            "duplicate interpolator name '{}' in animation '{}'",
                            interp.name, animation.name
                        ));
                    }
                    interp_names.insert(interp.name.clone());
                }
            }

            for group in &animation.keyframes {
                if !object_names.contains(&group.object) {
                    return Err(format!(
                        "unknown object referenced in keyframes: '{}'",
                        group.object
                    ));
                }
                let property_key = property_key_from_name(&group.property).ok_or_else(|| {
                    format!(
                        "unknown property referenced in keyframes: '{}'",
                        group.property
                    )
                })?;

                for frame in &group.frames {
                    if let Some(interp_name) = &frame.interpolator
                        && !interp_names.contains(interp_name)
                    {
                        return Err(format!(
                            "unknown interpolator '{}' referenced in keyframe",
                            interp_name
                        ));
                    }
                    if let Some(interp_type) = &frame.interpolation {
                        interpolation_type_from_name(interp_type)?;
                    }

                    if property_key == property_keys::SOLID_COLOR_VALUE {
                        if json_value_to_color(&frame.value).is_none() {
                            return Err(format!(
                                "invalid color keyframe value for object '{}' property '{}' at frame {}",
                                group.object, group.property, frame.frame
                            ));
                        }
                    } else if json_value_to_f32(&frame.value).is_none() {
                        return Err(format!(
                            "invalid numeric keyframe value for object '{}' property '{}' at frame {}",
                            group.object, group.property, frame.frame
                        ));
                    }
                }
            }
        }
    }

    if let Some(state_machines) = &spec.artboard.state_machines {
        for state_machine in state_machines {
            let mut input_names: HashSet<String> = HashSet::new();
            if let Some(inputs) = &state_machine.inputs {
                for input in inputs {
                    let name = match input {
                        InputSpec::Number { name, .. } => name,
                        InputSpec::Bool { name, .. } => name,
                        InputSpec::Trigger { name } => name,
                    };
                    if input_names.contains(name) {
                        return Err(format!(
                            "duplicate state machine input '{}' in '{}'",
                            name, state_machine.name
                        ));
                    }
                    input_names.insert(name.clone());
                }
            }

            for layer in &state_machine.layers {
                for state in &layer.states {
                    if let StateSpec::Animation { animation } = state
                        && !animation_names.contains(animation)
                    {
                        return Err(format!(
                            "unknown animation referenced in state machine '{}': '{}'",
                            state_machine.name, animation
                        ));
                    }
                }

                if let Some(transitions) = &layer.transitions {
                    for transition in transitions {
                        if transition.from >= layer.states.len() {
                            return Err(format!(
                                "transition source index {} out of bounds (layer has {} states)",
                                transition.from,
                                layer.states.len()
                            ));
                        }
                        if transition.to >= layer.states.len() {
                            return Err(format!(
                                "transition target index {} out of bounds (layer has {} states)",
                                transition.to,
                                layer.states.len()
                            ));
                        }

                        if let Some(conditions) = &transition.conditions {
                            for condition in conditions {
                                if !input_names.contains(&condition.input) {
                                    return Err(format!(
                                        "unknown input referenced in condition: '{}'",
                                        condition.input
                                    ));
                                }
                                if let Some(serde_json::Value::String(color)) =
                                    condition.value.as_ref()
                                {
                                    parse_color(color)?;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

fn validate_object_spec(
    spec: &ObjectSpec,
    object_names: &mut HashSet<String>,
) -> Result<(), String> {
    match spec {
        ObjectSpec::Shape { name, children, .. } => {
            ensure_unique_name(name, object_names)?;
            if let Some(children) = children {
                for child in children {
                    validate_object_spec(child, object_names)?;
                }
            }
        }
        ObjectSpec::Ellipse {
            name,
            width,
            height,
            ..
        } => {
            ensure_unique_name(name, object_names)?;
            if *width < 0.0 {
                return Err(format!("ellipse '{}' width must be non-negative", name));
            }
            if *height < 0.0 {
                return Err(format!("ellipse '{}' height must be non-negative", name));
            }
        }
        ObjectSpec::Rectangle {
            name,
            width,
            height,
            corner_radius,
            ..
        } => {
            ensure_unique_name(name, object_names)?;
            if *width < 0.0 {
                return Err(format!("rectangle '{}' width must be non-negative", name));
            }
            if *height < 0.0 {
                return Err(format!("rectangle '{}' height must be non-negative", name));
            }
            if let Some(corner_radius) = corner_radius
                && *corner_radius < 0.0
            {
                return Err(format!(
                    "rectangle '{}' corner_radius must be non-negative",
                    name
                ));
            }
        }
        ObjectSpec::Fill { name, children, .. } => {
            ensure_unique_name(name, object_names)?;
            if let Some(children) = children {
                for child in children {
                    validate_object_spec(child, object_names)?;
                }
            }
        }
        ObjectSpec::Stroke {
            name,
            thickness,
            children,
            ..
        } => {
            ensure_unique_name(name, object_names)?;
            if let Some(thickness) = thickness
                && *thickness < 0.0
            {
                return Err(format!("stroke '{}' thickness must be non-negative", name));
            }
            if let Some(children) = children {
                for child in children {
                    validate_object_spec(child, object_names)?;
                }
            }
        }
        ObjectSpec::SolidColor { name, color } => {
            ensure_unique_name(name, object_names)?;
            parse_color(color)?;
        }
        ObjectSpec::LinearGradient { name, children, .. } => {
            ensure_unique_name(name, object_names)?;
            if let Some(children) = children {
                for child in children {
                    validate_object_spec(child, object_names)?;
                }
            }
        }
        ObjectSpec::RadialGradient { name, children, .. } => {
            ensure_unique_name(name, object_names)?;
            if let Some(children) = children {
                for child in children {
                    validate_object_spec(child, object_names)?;
                }
            }
        }
        ObjectSpec::GradientStop {
            name,
            color,
            position,
        } => {
            let effective_name = name
                .clone()
                .unwrap_or_else(|| format!("gradient_stop_{}", object_names.len()));
            ensure_unique_name(&effective_name, object_names)?;
            parse_color(color)?;
            if !(0.0..=1.0).contains(position) {
                return Err(format!(
                    "gradient stop '{}' position must be between 0 and 1",
                    effective_name
                ));
            }
        }
        ObjectSpec::Node { name, .. } => {
            ensure_unique_name(name, object_names)?;
        }
        ObjectSpec::Path { name, .. } => {
            ensure_unique_name(name, object_names)?;
        }
        ObjectSpec::TrimPath {
            name, start, end, mode, ..
        } => {
            ensure_unique_name(name, object_names)?;
            if let Some(start) = start
                && *start < 0.0
            {
                return Err(format!("trim_path '{}' start must be non-negative", name));
            }
            if let Some(end) = end
                && *end < 0.0
            {
                return Err(format!("trim_path '{}' end must be non-negative", name));
            }
            if let Some(mode) = mode
                && *mode != 1
                && *mode != 2
            {
                return Err(format!(
                    "trim_path '{}' mode must be 1 (sequential) or 2 (synchronized)",
                    name
                ));
            }
        }
    }

    Ok(())
}

fn ensure_unique_name(name: &str, object_names: &mut HashSet<String>) -> Result<(), String> {
    if object_names.contains(name) {
        return Err(format!("duplicate object name '{}'", name));
    }
    object_names.insert(name.to_string());
    Ok(())
}

fn parse_condition_op(op: &str) -> u64 {
    match op {
        "==" | "eq" => 0,
        "!=" | "ne" => 1,
        ">" | "gt" => 2,
        ">=" | "gte" => 3,
        "<" | "lt" => 4,
        "<=" | "lte" => 5,
        _ => 0,
    }
}

fn input_is_trigger(input_name: &str, inputs: Option<&Vec<InputSpec>>) -> bool {
    if let Some(inputs) = inputs {
        for input in inputs {
            if let InputSpec::Trigger { name } = input
                && name == input_name
            {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::objects::core::{PropertyValue, property_keys, type_keys};

    #[test]
    fn test_parse_minimal_json() {
        let json = r#"{
            "scene_format_version": 1,
            "artboard": {
                "name": "Main",
                "width": 500.0,
                "height": 500.0,
                "children": []
            }
        }"#;

        let scene: SceneSpec = serde_json::from_str(json).unwrap();
        assert_eq!(scene.artboard.name, "Main");
        assert_eq!(scene.artboard.width, 500.0);
        assert_eq!(scene.artboard.children.len(), 0);
        assert!(scene.artboard.animations.is_none());
        assert_eq!(scene.scene_format_version, 1);
    }

    #[test]
    fn test_reject_unsupported_scene_format_version() {
        let spec = SceneSpec {
            scene_format_version: 2,
            artboard: ArtboardSpec {
                name: "Main".to_string(),
                width: 500.0,
                height: 500.0,
                children: vec![],
                animations: None,
                state_machines: None,
            },
        };

        let err = match build_scene(&spec) {
            Ok(_) => panic!("expected scene format version error"),
            Err(err) => err,
        };
        assert!(err.contains("unsupported scene_format_version 2"));
    }

    #[test]
    fn test_parse_shape_with_fill() {
        let json = r#"{
            "scene_format_version": 1,
            "artboard": {
                "name": "Main",
                "width": 100.0,
                "height": 100.0,
                "children": [
                    {
                        "type": "shape",
                        "name": "shape_1",
                        "children": [
                            {
                                "type": "ellipse",
                                "name": "ellipse_1",
                                "width": 40.0,
                                "height": 40.0
                            },
                            {
                                "type": "fill",
                                "name": "fill_1",
                                "children": [
                                    {
                                        "type": "solid_color",
                                        "name": "color_1",
                                        "color": "FFFF0000"
                                    }
                                ]
                            }
                        ]
                    }
                ]
            }
        }"#;

        let scene: SceneSpec = serde_json::from_str(json).unwrap();
        assert_eq!(scene.artboard.children.len(), 1);
        match &scene.artboard.children[0] {
            ObjectSpec::Shape { name, children, .. } => {
                assert_eq!(name, "shape_1");
                assert_eq!(children.as_ref().unwrap().len(), 2);
            }
            _ => panic!("expected shape"),
        }
    }

    #[test]
    fn test_build_minimal_scene() {
        let spec = SceneSpec {
            scene_format_version: 1,
            artboard: ArtboardSpec {
                name: "Main".to_string(),
                width: 500.0,
                height: 500.0,
                children: vec![],
                animations: None,
                state_machines: None,
            },
        };

        let objects = build_scene(&spec).unwrap();
        assert_eq!(objects.len(), 2);
        assert_eq!(objects[0].type_key(), type_keys::BACKBOARD);
        assert_eq!(objects[1].type_key(), type_keys::ARTBOARD);

        let artboard_props = objects[1].properties();
        assert!(
            !artboard_props
                .iter()
                .any(|p| p.key == property_keys::COMPONENT_PARENT_ID)
        );
    }

    #[test]
    fn test_build_scene_with_shape() {
        let spec = SceneSpec {
            scene_format_version: 1,
            artboard: ArtboardSpec {
                name: "Main".to_string(),
                width: 500.0,
                height: 500.0,
                children: vec![ObjectSpec::Shape {
                    name: "shape_1".to_string(),
                    x: None,
                    y: None,
                    children: Some(vec![
                        ObjectSpec::Ellipse {
                            name: "ellipse_1".to_string(),
                            width: 120.0,
                            height: 80.0,
                            origin_x: None,
                            origin_y: None,
                        },
                        ObjectSpec::Fill {
                            name: "fill_1".to_string(),
                            fill_rule: None,
                            children: Some(vec![ObjectSpec::SolidColor {
                                name: "color_1".to_string(),
                                color: "FFFF0000".to_string(),
                            }]),
                        },
                    ]),
                }],
                animations: None,
                state_machines: None,
            },
        };

        let objects = build_scene(&spec).unwrap();
        assert_eq!(objects.len(), 6);
        assert_eq!(objects[0].type_key(), type_keys::BACKBOARD);
        assert_eq!(objects[1].type_key(), type_keys::ARTBOARD);
        assert_eq!(objects[2].type_key(), type_keys::SHAPE);
        assert_eq!(objects[3].type_key(), type_keys::ELLIPSE);
        assert_eq!(objects[4].type_key(), type_keys::FILL);
        assert_eq!(objects[5].type_key(), type_keys::SOLID_COLOR);

        let shape_parent = objects[2]
            .properties()
            .into_iter()
            .find(|p| p.key == property_keys::COMPONENT_PARENT_ID)
            .unwrap();
        assert_eq!(shape_parent.value, PropertyValue::UInt(0));

        let ellipse_parent = objects[3]
            .properties()
            .into_iter()
            .find(|p| p.key == property_keys::COMPONENT_PARENT_ID)
            .unwrap();
        assert_eq!(ellipse_parent.value, PropertyValue::UInt(1));

        let fill_parent = objects[4]
            .properties()
            .into_iter()
            .find(|p| p.key == property_keys::COMPONENT_PARENT_ID)
            .unwrap();
        assert_eq!(fill_parent.value, PropertyValue::UInt(1));

        let color_parent = objects[5]
            .properties()
            .into_iter()
            .find(|p| p.key == property_keys::COMPONENT_PARENT_ID)
            .unwrap();
        assert_eq!(color_parent.value, PropertyValue::UInt(3));
    }

    #[test]
    fn test_build_scene_with_animation() {
        let spec = SceneSpec {
            scene_format_version: 1,
            artboard: ArtboardSpec {
                name: "Main".to_string(),
                width: 500.0,
                height: 500.0,
                children: vec![ObjectSpec::Shape {
                    name: "shape_1".to_string(),
                    x: None,
                    y: None,
                    children: Some(vec![ObjectSpec::Ellipse {
                        name: "ellipse_1".to_string(),
                        width: 120.0,
                        height: 80.0,
                        origin_x: None,
                        origin_y: None,
                    }]),
                }],
                animations: Some(vec![AnimationSpec {
                    name: "grow".to_string(),
                    fps: 60,
                    duration: 120,
                    speed: Some(1.0),
                    loop_type: Some(1),
                    interpolators: None,
                    keyframes: vec![KeyframeGroupSpec {
                        object: "ellipse_1".to_string(),
                        property: "width".to_string(),
                        frames: vec![
                            KeyframeSpec {
                                frame: 0,
                                value: serde_json::json!(120.0),
                                interpolation: None,
                                interpolator: None,
                            },
                            KeyframeSpec {
                                frame: 60,
                                value: serde_json::json!(200.0),
                                interpolation: None,
                                interpolator: None,
                            },
                        ],
                    }],
                }]),
                state_machines: None,
            },
        };

        let objects = build_scene(&spec).unwrap();
        assert_eq!(objects[0].type_key(), type_keys::BACKBOARD);
        assert_eq!(objects[1].type_key(), type_keys::ARTBOARD);
        assert_eq!(objects[2].type_key(), type_keys::SHAPE);
        assert_eq!(objects[3].type_key(), type_keys::ELLIPSE);
        assert_eq!(objects[4].type_key(), type_keys::LINEAR_ANIMATION);
        assert_eq!(objects[5].type_key(), type_keys::KEYED_OBJECT);
        assert_eq!(objects[6].type_key(), type_keys::KEYED_PROPERTY);
        assert_eq!(objects[7].type_key(), type_keys::KEY_FRAME_DOUBLE);
        assert_eq!(objects[8].type_key(), type_keys::KEY_FRAME_DOUBLE);

        let keyed_object_id = objects[5]
            .properties()
            .into_iter()
            .find(|p| p.key == property_keys::KEYED_OBJECT_ID)
            .unwrap();
        assert_eq!(keyed_object_id.value, PropertyValue::UInt(2));
    }

    #[test]
    fn test_build_scene_rejects_invalid_color() {
        let spec = SceneSpec {
            scene_format_version: 1,
            artboard: ArtboardSpec {
                name: "Main".to_string(),
                width: 100.0,
                height: 100.0,
                children: vec![ObjectSpec::SolidColor {
                    name: "bad_color".to_string(),
                    color: "not-a-color".to_string(),
                }],
                animations: None,
                state_machines: None,
            },
        };

        let err = match build_scene(&spec) {
            Ok(_) => panic!("expected invalid color error"),
            Err(err) => err,
        };
        assert!(err.contains("invalid color literal"));
    }

    #[test]
    fn test_build_scene_rejects_oob_transition_index() {
        let spec = SceneSpec {
            scene_format_version: 1,
            artboard: ArtboardSpec {
                name: "Main".to_string(),
                width: 100.0,
                height: 100.0,
                children: vec![],
                animations: None,
                state_machines: Some(vec![StateMachineSpec {
                    name: "sm".to_string(),
                    inputs: None,
                    layers: vec![LayerSpec {
                        states: vec![StateSpec::Entry, StateSpec::Exit],
                        transitions: Some(vec![TransitionSpec {
                            from: 0,
                            to: 3,
                            duration: None,
                            conditions: None,
                        }]),
                    }],
                }]),
            },
        };

        let err = match build_scene(&spec) {
            Ok(_) => panic!("expected out-of-bounds transition error"),
            Err(err) => err,
        };
        assert!(err.contains("transition target index 3 out of bounds"));
    }

    #[test]
    fn test_build_scene_with_path_object() {
        let spec = SceneSpec {
            scene_format_version: 1,
            artboard: ArtboardSpec {
                name: "Main".to_string(),
                width: 100.0,
                height: 100.0,
                children: vec![ObjectSpec::Path {
                    name: "path_1".to_string(),
                    path_flags: Some(3),
                }],
                animations: None,
                state_machines: None,
            },
        };

        let objects = build_scene(&spec).unwrap();
        assert_eq!(objects[2].type_key(), type_keys::PATH);
        let path_flags = objects[2]
            .properties()
            .into_iter()
            .find(|p| p.key == property_keys::PATH_FLAGS)
            .unwrap();
        assert_eq!(path_flags.value, PropertyValue::UInt(3));
    }

    #[test]
    fn test_gradient_stop_generated_name_alignment() {
        let spec = SceneSpec {
            scene_format_version: 1,
            artboard: ArtboardSpec {
                name: "Main".to_string(),
                width: 100.0,
                height: 100.0,
                children: vec![ObjectSpec::Shape {
                    name: "shape_1".to_string(),
                    x: None,
                    y: None,
                    children: Some(vec![
                        ObjectSpec::GradientStop {
                            name: Some("gradient_stop_1".to_string()),
                            color: "FFFFFFFF".to_string(),
                            position: 0.0,
                        },
                        ObjectSpec::GradientStop {
                            name: None,
                            color: "FF000000".to_string(),
                            position: 1.0,
                        },
                    ]),
                }],
                animations: None,
                state_machines: None,
            },
        };

        let objects = build_scene(&spec).unwrap();
        assert!(objects.len() >= 4);
    }

    #[test]
    fn test_json_value_to_color_rejects_overflow() {
        let value = serde_json::json!(4294967296u64);
        assert!(json_value_to_color(&value).is_none());
    }
}
