use std::collections::HashMap;

use serde::Deserialize;

use crate::objects::animation::{
    KeyFrameColor, KeyFrameDouble, KeyedObject, KeyedProperty, LinearAnimation,
};
use crate::objects::artboard::{Artboard, Backboard};
use crate::objects::core::RiveObject;
use crate::objects::shapes::{
    Ellipse, Fill, GradientStop, LinearGradient, Node, RadialGradient, Rectangle, Shape,
    SolidColor, Stroke,
};
use crate::objects::state_machine::{
    AnimationState, AnyState, EntryState, ExitState, StateMachine, StateMachineBool,
    StateMachineLayer, StateMachineNumber, StateMachineTrigger, StateTransition,
    TransitionBoolCondition, TransitionInputCondition, TransitionNumberCondition,
    TransitionTriggerCondition, TransitionValueCondition,
};

#[derive(Debug, Deserialize)]
pub struct SceneSpec {
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
}

#[derive(Debug, Deserialize)]
pub struct AnimationSpec {
    pub name: String,
    pub fps: u64,
    pub duration: u64,
    pub speed: Option<f32>,
    pub loop_type: Option<u64>,
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

pub fn build_scene(spec: &SceneSpec) -> Vec<Box<dyn RiveObject>> {
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
        append_object(child, 1, &mut objects, &mut object_name_to_index);
    }

    if let Some(animations) = &spec.artboard.animations {
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
                if let Some(&object_index) = object_name_to_index.get(&group.object) {
                    objects.push(Box::new(KeyedObject {
                        object_id: (object_index - 1) as u64,
                    }));

                    if let Some(property_key) = property_key_from_name(&group.property) {
                        objects.push(Box::new(KeyedProperty {
                            property_key: property_key as u64,
                        }));

                        for frame in &group.frames {
                            if property_key == 37 {
                                if let Some(color) = json_value_to_color(&frame.value) {
                                    objects.push(Box::new(KeyFrameColor::new(frame.frame, color)));
                                }
                            } else if let Some(value) = json_value_to_f32(&frame.value) {
                                objects.push(Box::new(KeyFrameDouble::new(frame.frame, value)));
                            }
                        }
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
                                *animation_name_to_index.get(animation).unwrap_or_else(|| {
                                    panic!("unknown animation referenced: '{}'", animation)
                                }) as u64;
                            objects.push(Box::new(AnimationState::new(animation_id)));
                        }
                    }

                    if let Some(transitions) = &layer.transitions {
                        for transition in transitions {
                            if transition.from != user_idx {
                                continue;
                            }
                            let state_to_id =
                                *user_to_final.get(transition.to).unwrap_or_else(|| {
                                    panic!(
                                        "transition target index {} out of bounds (layer has {} states)",
                                        transition.to,
                                        user_to_final.len()
                                    )
                                }) as u64;
                            let mut state_transition = StateTransition::new(state_to_id);
                            if let Some(duration) = transition.duration {
                                state_transition.duration = duration;
                            }
                            objects.push(Box::new(state_transition));

                            if let Some(conditions) = &transition.conditions {
                                for condition in conditions {
                                    if let Some(&input_index) =
                                        input_name_to_index.get(&condition.input)
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
                                                    .unwrap_or(0.0);
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

    objects
}

fn append_object(
    spec: &ObjectSpec,
    parent_index: usize,
    objects: &mut Vec<Box<dyn RiveObject>>,
    name_to_index: &mut HashMap<String, usize>,
) {
    let object_index = objects.len();
    let parent_id = (parent_index - 1) as u64;

    match spec {
        ObjectSpec::Shape { name, children } => {
            objects.push(Box::new(Shape::new(name.clone(), parent_id)));
            name_to_index.insert(name.clone(), object_index);
            if let Some(children) = children {
                for child in children {
                    append_object(child, object_index, objects, name_to_index);
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
                    append_object(child, object_index, objects, name_to_index);
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
                    append_object(child, object_index, objects, name_to_index);
                }
            }
        }
        ObjectSpec::SolidColor { name, color } => {
            let color_value = parse_color(color).unwrap_or(0);
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
                    append_object(child, object_index, objects, name_to_index);
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
                    append_object(child, object_index, objects, name_to_index);
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
                .unwrap_or_else(|| format!("gradient_stop_{}", object_index));
            objects.push(Box::new(GradientStop {
                name: generated_name.clone(),
                parent_id,
                color: parse_color(color).unwrap_or(0),
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
    }
}

fn property_key_from_name(name: &str) -> Option<u16> {
    match name {
        "x" => Some(13),
        "y" => Some(14),
        "rotation" => Some(15),
        "scale_x" => Some(16),
        "scale_y" => Some(17),
        "opacity" => Some(18),
        "width" => Some(20),
        "height" => Some(21),
        "color" => Some(37),
        _ => None,
    }
}

fn parse_color(color: &str) -> Option<u32> {
    let hex = color.trim_start_matches('#');
    u32::from_str_radix(hex, 16).ok()
}

fn json_value_to_f32(value: &serde_json::Value) -> Option<f32> {
    match value {
        serde_json::Value::Number(number) => number.as_f64().map(|v| v as f32),
        _ => None,
    }
}

fn json_value_to_color(value: &serde_json::Value) -> Option<u32> {
    match value {
        serde_json::Value::String(s) => parse_color(s),
        serde_json::Value::Number(n) => n.as_u64().map(|v| v as u32),
        _ => None,
    }
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
            if let InputSpec::Trigger { name } = input {
                if name == input_name {
                    return true;
                }
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::objects::core::{property_keys, type_keys, PropertyValue};

    #[test]
    fn test_parse_minimal_json() {
        let json = r#"{
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
    }

    #[test]
    fn test_parse_shape_with_fill() {
        let json = r#"{
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
            ObjectSpec::Shape { name, children } => {
                assert_eq!(name, "shape_1");
                assert_eq!(children.as_ref().unwrap().len(), 2);
            }
            _ => panic!("expected shape"),
        }
    }

    #[test]
    fn test_build_minimal_scene() {
        let spec = SceneSpec {
            artboard: ArtboardSpec {
                name: "Main".to_string(),
                width: 500.0,
                height: 500.0,
                children: vec![],
                animations: None,
                state_machines: None,
            },
        };

        let objects = build_scene(&spec);
        assert_eq!(objects.len(), 2);
        assert_eq!(objects[0].type_key(), type_keys::BACKBOARD);
        assert_eq!(objects[1].type_key(), type_keys::ARTBOARD);

        let artboard_props = objects[1].properties();
        assert!(!artboard_props
            .iter()
            .any(|p| p.key == property_keys::COMPONENT_PARENT_ID));
    }

    #[test]
    fn test_build_scene_with_shape() {
        let spec = SceneSpec {
            artboard: ArtboardSpec {
                name: "Main".to_string(),
                width: 500.0,
                height: 500.0,
                children: vec![ObjectSpec::Shape {
                    name: "shape_1".to_string(),
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

        let objects = build_scene(&spec);
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
            artboard: ArtboardSpec {
                name: "Main".to_string(),
                width: 500.0,
                height: 500.0,
                children: vec![ObjectSpec::Shape {
                    name: "shape_1".to_string(),
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
                    keyframes: vec![KeyframeGroupSpec {
                        object: "ellipse_1".to_string(),
                        property: "width".to_string(),
                        frames: vec![
                            KeyframeSpec {
                                frame: 0,
                                value: serde_json::json!(120.0),
                            },
                            KeyframeSpec {
                                frame: 60,
                                value: serde_json::json!(200.0),
                            },
                        ],
                    }],
                }]),
                state_machines: None,
            },
        };

        let objects = build_scene(&spec);
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
}
