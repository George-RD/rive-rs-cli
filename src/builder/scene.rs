use std::collections::{HashMap, HashSet};

use serde::Deserialize;

use crate::objects::animation::{
    CubicEaseInterpolator, KeyFrameColor, KeyFrameDouble, KeyedObject, KeyedProperty,
    LinearAnimation,
};
use crate::objects::artboard::{Artboard, Backboard, NestedArtboard};
use crate::objects::assets::{AudioAsset, FontAsset, ImageAsset};
use crate::objects::bones::{Bone, CubicWeight, RootBone, Skin, Tendon, Weight};
use crate::objects::constraints::{
    DistanceConstraint, IKConstraint, RotationConstraint, ScaleConstraint, TransformConstraint,
    TranslationConstraint,
};
use crate::objects::core::{RiveObject, property_keys};
use crate::objects::data_binding::{DataBind, ViewModel, ViewModelProperty};
use crate::objects::layout::{LayoutComponent, LayoutComponentStyle};
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
use crate::objects::text::{Text, TextStyle, TextValueRun};

const SCENE_FORMAT_VERSION: u32 = 1;

const ARTBOARD_PRESET_MOBILE_WIDTH: f32 = 390.0;
const ARTBOARD_PRESET_MOBILE_HEIGHT: f32 = 844.0;
const ARTBOARD_PRESET_TABLET_WIDTH: f32 = 768.0;
const ARTBOARD_PRESET_TABLET_HEIGHT: f32 = 1024.0;
const ARTBOARD_PRESET_DESKTOP_WIDTH: f32 = 1440.0;
const ARTBOARD_PRESET_DESKTOP_HEIGHT: f32 = 900.0;
const ARTBOARD_PRESET_SQUARE_WIDTH: f32 = 500.0;
const ARTBOARD_PRESET_SQUARE_HEIGHT: f32 = 500.0;
const ARTBOARD_PRESET_BANNER_WIDTH: f32 = 728.0;
const ARTBOARD_PRESET_BANNER_HEIGHT: f32 = 90.0;
const ARTBOARD_PRESET_STORY_WIDTH: f32 = 1080.0;
const ARTBOARD_PRESET_STORY_HEIGHT: f32 = 1920.0;

#[derive(Debug, Deserialize)]
pub struct SceneSpec {
    pub scene_format_version: u32,
    #[serde(default)]
    pub artboard: Option<ArtboardSpec>,
    #[serde(default)]
    pub artboards: Option<Vec<ArtboardSpec>>,
}

#[derive(Debug, Deserialize)]
pub struct ArtboardSpec {
    pub name: String,
    #[serde(default)]
    pub preset: Option<String>,
    #[serde(default)]
    pub width: f32,
    #[serde(default)]
    pub height: f32,
    pub children: Vec<ObjectSpec>,
    pub animations: Option<Vec<AnimationSpec>>,
    pub state_machines: Option<Vec<StateMachineSpec>>,
}

#[derive(Clone, Copy)]
pub struct ArtboardPreset {
    pub name: &'static str,
    pub width: f32,
    pub height: f32,
}

const ARTBOARD_PRESETS: [ArtboardPreset; 6] = [
    ArtboardPreset {
        name: "mobile",
        width: ARTBOARD_PRESET_MOBILE_WIDTH,
        height: ARTBOARD_PRESET_MOBILE_HEIGHT,
    },
    ArtboardPreset {
        name: "tablet",
        width: ARTBOARD_PRESET_TABLET_WIDTH,
        height: ARTBOARD_PRESET_TABLET_HEIGHT,
    },
    ArtboardPreset {
        name: "desktop",
        width: ARTBOARD_PRESET_DESKTOP_WIDTH,
        height: ARTBOARD_PRESET_DESKTOP_HEIGHT,
    },
    ArtboardPreset {
        name: "square",
        width: ARTBOARD_PRESET_SQUARE_WIDTH,
        height: ARTBOARD_PRESET_SQUARE_HEIGHT,
    },
    ArtboardPreset {
        name: "banner",
        width: ARTBOARD_PRESET_BANNER_WIDTH,
        height: ARTBOARD_PRESET_BANNER_HEIGHT,
    },
    ArtboardPreset {
        name: "story",
        width: ARTBOARD_PRESET_STORY_WIDTH,
        height: ARTBOARD_PRESET_STORY_HEIGHT,
    },
];

pub fn artboard_presets() -> &'static [ArtboardPreset] {
    &ARTBOARD_PRESETS
}

fn lookup_artboard_preset(name: &str) -> Option<(f32, f32)> {
    ARTBOARD_PRESETS
        .iter()
        .find(|preset| preset.name == name)
        .map(|preset| (preset.width, preset.height))
}

fn resolve_artboard_dimensions(artboard_spec: &ArtboardSpec) -> Result<(f32, f32), String> {
    let preset_dimensions = if let Some(preset_name) = artboard_spec.preset.as_deref() {
        Some(lookup_artboard_preset(preset_name).ok_or_else(|| {
            format!(
                "artboard '{}' has unknown preset '{}'",
                artboard_spec.name, preset_name
            )
        })?)
    } else {
        None
    };

    let width = if artboard_spec.width > 0.0 {
        artboard_spec.width
    } else if let Some((preset_width, _)) = preset_dimensions {
        preset_width
    } else {
        return Err(format!(
            "artboard '{}' must specify non-zero width and height or a preset",
            artboard_spec.name
        ));
    };

    let height = if artboard_spec.height > 0.0 {
        artboard_spec.height
    } else if let Some((_, preset_height)) = preset_dimensions {
        preset_height
    } else {
        return Err(format!(
            "artboard '{}' must specify non-zero width and height or a preset",
            artboard_spec.name
        ));
    };

    Ok((width, height))
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
    NestedArtboard {
        name: String,
        source_artboard: String,
        x: Option<f32>,
        y: Option<f32>,
    },
    Bone {
        name: String,
        length: Option<f32>,
        children: Option<Vec<ObjectSpec>>,
    },
    RootBone {
        name: String,
        x: Option<f32>,
        y: Option<f32>,
        length: Option<f32>,
        children: Option<Vec<ObjectSpec>>,
    },
    Skin {
        name: String,
        xx: Option<f32>,
        yx: Option<f32>,
        xy: Option<f32>,
        yy: Option<f32>,
        tx: Option<f32>,
        ty: Option<f32>,
        children: Option<Vec<ObjectSpec>>,
    },
    Tendon {
        name: String,
        bone: Option<String>,
        xx: Option<f32>,
        yx: Option<f32>,
        xy: Option<f32>,
        yy: Option<f32>,
        tx: Option<f32>,
        ty: Option<f32>,
    },
    Weight {
        name: String,
        values: Option<u64>,
        indices: Option<u64>,
    },
    CubicWeight {
        name: String,
        in_values: Option<u64>,
        in_indices: Option<u64>,
        out_values: Option<u64>,
        out_indices: Option<u64>,
    },
    IkConstraint {
        name: String,
        target: Option<String>,
        strength: Option<f32>,
        invert_direction: Option<bool>,
        parent_bone_count: Option<u64>,
    },
    DistanceConstraint {
        name: String,
        target: Option<String>,
        strength: Option<f32>,
        distance: Option<f32>,
        mode_value: Option<u64>,
    },
    TransformConstraint {
        name: String,
        target: Option<String>,
        strength: Option<f32>,
        source_space_value: Option<u64>,
        dest_space_value: Option<u64>,
        origin_x: Option<f32>,
        origin_y: Option<f32>,
    },
    TranslationConstraint {
        name: String,
        target: Option<String>,
        strength: Option<f32>,
        source_space_value: Option<u64>,
        dest_space_value: Option<u64>,
        copy_factor: Option<f32>,
        min_value: Option<f32>,
        max_value: Option<f32>,
        offset: Option<bool>,
        does_copy: Option<bool>,
        min: Option<bool>,
        max: Option<bool>,
        min_max_space_value: Option<u64>,
        copy_factor_y: Option<f32>,
        min_value_y: Option<f32>,
        max_value_y: Option<f32>,
        does_copy_y: Option<bool>,
        min_y: Option<bool>,
        max_y: Option<bool>,
    },
    ScaleConstraint {
        name: String,
        target: Option<String>,
        strength: Option<f32>,
        source_space_value: Option<u64>,
        dest_space_value: Option<u64>,
        copy_factor: Option<f32>,
        min_value: Option<f32>,
        max_value: Option<f32>,
        offset: Option<bool>,
        does_copy: Option<bool>,
        min: Option<bool>,
        max: Option<bool>,
        min_max_space_value: Option<u64>,
        copy_factor_y: Option<f32>,
        min_value_y: Option<f32>,
        max_value_y: Option<f32>,
        does_copy_y: Option<bool>,
        min_y: Option<bool>,
        max_y: Option<bool>,
    },
    RotationConstraint {
        name: String,
        target: Option<String>,
        strength: Option<f32>,
        source_space_value: Option<u64>,
        dest_space_value: Option<u64>,
        copy_factor: Option<f32>,
        min_value: Option<f32>,
        max_value: Option<f32>,
        offset: Option<bool>,
        does_copy: Option<bool>,
        min: Option<bool>,
        max: Option<bool>,
        min_max_space_value: Option<u64>,
    },
    Text {
        name: String,
        align_value: Option<u64>,
        sizing_value: Option<u64>,
        overflow_value: Option<u64>,
        width: Option<f32>,
        height: Option<f32>,
        origin_x: Option<f32>,
        origin_y: Option<f32>,
        paragraph_spacing: Option<f32>,
        origin_value: Option<u64>,
        children: Option<Vec<ObjectSpec>>,
    },
    TextStyle {
        name: String,
        font_size: Option<f32>,
        line_height: Option<f32>,
        letter_spacing: Option<f32>,
        font_asset_id: Option<u64>,
    },
    TextValueRun {
        name: String,
        text: String,
        style_id: Option<u64>,
    },
    ImageAsset {
        name: String,
        asset_id: Option<u64>,
        cdn_base_url: Option<String>,
    },
    FontAsset {
        name: String,
        asset_id: Option<u64>,
        cdn_base_url: Option<String>,
    },
    AudioAsset {
        name: String,
        asset_id: Option<u64>,
        cdn_base_url: Option<String>,
    },
    LayoutComponent {
        name: String,
        clip: Option<bool>,
        width: Option<f32>,
        height: Option<f32>,
        style_id: Option<u64>,
        fractional_width: Option<f32>,
        fractional_height: Option<f32>,
        children: Option<Vec<ObjectSpec>>,
    },
    LayoutComponentStyle {
        name: String,
        gap_horizontal: Option<f32>,
        gap_vertical: Option<f32>,
        max_width: Option<f32>,
        max_height: Option<f32>,
        min_width: Option<f32>,
        min_height: Option<f32>,
        border_left: Option<f32>,
        border_right: Option<f32>,
        border_top: Option<f32>,
        border_bottom: Option<f32>,
        margin_left: Option<f32>,
        margin_right: Option<f32>,
        margin_top: Option<f32>,
        margin_bottom: Option<f32>,
        padding_left: Option<f32>,
        padding_right: Option<f32>,
        padding_top: Option<f32>,
        padding_bottom: Option<f32>,
        position_left: Option<f32>,
        position_right: Option<f32>,
        position_top: Option<f32>,
        position_bottom: Option<f32>,
        flex_direction: Option<u64>,
        flex_wrap: Option<u64>,
        align_items: Option<u64>,
        align_content: Option<u64>,
        justify_content: Option<u64>,
        display: Option<u64>,
        position_type: Option<u64>,
        overflow: Option<u64>,
        intrinsically_sized: Option<bool>,
        width_units: Option<u64>,
        height_units: Option<u64>,
        flex_grow: Option<f32>,
        flex_shrink: Option<f32>,
        flex_basis: Option<f32>,
        aspect_ratio: Option<f32>,
    },
    ViewModel {
        name: String,
        children: Option<Vec<ObjectSpec>>,
    },
    ViewModelProperty {
        name: String,
        property_type_value: Option<u64>,
    },
    DataBind {
        property_key: u64,
        flags: u64,
        converter_id: Option<u64>,
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

fn resolve_artboards(spec: &SceneSpec) -> Result<Vec<&ArtboardSpec>, String> {
    match (&spec.artboard, &spec.artboards) {
        (Some(ab), None) => Ok(vec![ab]),
        (None, Some(abs)) => {
            if abs.is_empty() {
                return Err("artboards array must not be empty".to_string());
            }
            Ok(abs.iter().collect())
        }
        (Some(_), Some(_)) => Err("specify either 'artboard' or 'artboards', not both".to_string()),
        (None, None) => Err("must specify either 'artboard' or 'artboards'".to_string()),
    }
}

pub fn build_scene(spec: &SceneSpec) -> Result<Vec<Box<dyn RiveObject>>, String> {
    validate_scene_spec(spec)?;

    let artboard_specs = resolve_artboards(spec)?;
    let mut artboard_name_to_index: HashMap<String, usize> = HashMap::new();
    for (artboard_index, artboard_spec) in artboard_specs.iter().enumerate() {
        artboard_name_to_index.insert(artboard_spec.name.clone(), artboard_index);
    }
    let mut objects: Vec<Box<dyn RiveObject>> = Vec::new();

    objects.push(Box::new(Backboard));

    for artboard_spec in &artboard_specs {
        let artboard_start = objects.len();
        let (artboard_width, artboard_height) = resolve_artboard_dimensions(artboard_spec)?;

        let mut artboard =
            Artboard::new(artboard_spec.name.clone(), artboard_width, artboard_height);
        if artboard_spec
            .state_machines
            .as_ref()
            .is_some_and(|sms| !sms.is_empty())
        {
            artboard.default_state_machine_id = Some(0);
        }
        objects.push(Box::new(artboard));

        let mut object_name_to_index: HashMap<String, usize> = HashMap::new();
        let mut animation_name_to_index: HashMap<String, usize> = HashMap::new();
        let mut interpolator_name_to_index: HashMap<String, usize> = HashMap::new();
        let mut interpolator_control_points: HashMap<String, (f32, f32, f32, f32)> = HashMap::new();

        for child in &artboard_spec.children {
            append_object(
                child,
                artboard_start,
                artboard_start,
                &mut objects,
                &mut object_name_to_index,
                &artboard_name_to_index,
                &artboard_spec.name,
            )?;
        }

        if let Some(animations) = &artboard_spec.animations {
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
                            if (stored_x1, stored_y1, stored_x2, stored_y2) != (&x1, &y1, &x2, &y2)
                            {
                                return Err(format!(
                                    "duplicate interpolator '{}' with different control points",
                                    interp.name
                                ));
                            }
                            continue;
                        }

                        let artboard_local_index =
                            objects.len().checked_sub(artboard_start).ok_or(
                                "internal error: interpolator index precedes artboard start"
                                    .to_string(),
                            )?;
                        interpolator_name_to_index
                            .insert(interp.name.clone(), artboard_local_index);
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
                    let object_index =
                        *object_name_to_index.get(&group.object).ok_or_else(|| {
                            format!("unknown object referenced in keyframes: '{}'", group.object)
                        })?;
                    let keyed_object_id = object_index.checked_sub(artboard_start).ok_or(
                        "internal error: keyed object index precedes artboard start".to_string(),
                    )?;
                    objects.push(Box::new(KeyedObject {
                        object_id: keyed_object_id as u64,
                    }));

                    let property_key =
                        property_key_from_name(&group.property).ok_or_else(|| {
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
                                let idx =
                                    *interpolator_name_to_index.get(name).ok_or_else(|| {
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

        if let Some(state_machines) = &artboard_spec.state_machines {
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
                                                Some(serde_json::Value::Bool(_v)) => {
                                                    let bool_op = condition
                                                        .op
                                                        .as_deref()
                                                        .map(parse_condition_op)
                                                        .unwrap_or(0);
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
    artboard_start: usize,
    objects: &mut Vec<Box<dyn RiveObject>>,
    name_to_index: &mut HashMap<String, usize>,
    artboard_name_to_index: &HashMap<String, usize>,
    current_artboard_name: &str,
) -> Result<(), String> {
    let object_index = objects.len();
    let parent_id = parent_index
        .checked_sub(artboard_start)
        .ok_or("internal error: parent index precedes artboard start".to_string())?
        as u64;

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
                    append_object(
                        child,
                        object_index,
                        artboard_start,
                        objects,
                        name_to_index,
                        artboard_name_to_index,
                        current_artboard_name,
                    )?;
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
                    append_object(
                        child,
                        object_index,
                        artboard_start,
                        objects,
                        name_to_index,
                        artboard_name_to_index,
                        current_artboard_name,
                    )?;
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
                    append_object(
                        child,
                        object_index,
                        artboard_start,
                        objects,
                        name_to_index,
                        artboard_name_to_index,
                        current_artboard_name,
                    )?;
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
                    append_object(
                        child,
                        object_index,
                        artboard_start,
                        objects,
                        name_to_index,
                        artboard_name_to_index,
                        current_artboard_name,
                    )?;
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
                    append_object(
                        child,
                        object_index,
                        artboard_start,
                        objects,
                        name_to_index,
                        artboard_name_to_index,
                        current_artboard_name,
                    )?;
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
                trim_path
                    .set_mode(*mode)
                    .map_err(|e| format!("trim_path '{}': {}", name, e))?;
            }
            objects.push(Box::new(trim_path));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::NestedArtboard {
            name,
            source_artboard,
            x,
            y,
        } => {
            if source_artboard == current_artboard_name {
                return Err(format!(
                    "nested artboard '{}' cannot reference its own artboard '{}'",
                    name, source_artboard
                ));
            }
            let source_artboard_index =
                *artboard_name_to_index.get(source_artboard).ok_or_else(|| {
                    format!(
                        "nested artboard '{}' references unknown artboard '{}'",
                        name, source_artboard
                    )
                })?;
            objects.push(Box::new(NestedArtboard {
                name: name.clone(),
                parent_id,
                artboard_id: source_artboard_index as u64,
                x: x.unwrap_or(0.0),
                y: y.unwrap_or(0.0),
            }));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::Bone {
            name,
            length,
            children,
        } => {
            let mut bone = Bone::new(name.clone(), parent_id);
            if let Some(length) = length {
                bone.length = *length;
            }
            objects.push(Box::new(bone));
            name_to_index.insert(name.clone(), object_index);
            if let Some(children) = children {
                for child in children {
                    append_object(
                        child,
                        object_index,
                        artboard_start,
                        objects,
                        name_to_index,
                        artboard_name_to_index,
                        current_artboard_name,
                    )?;
                }
            }
        }
        ObjectSpec::RootBone {
            name,
            x,
            y,
            length,
            children,
        } => {
            let mut root_bone = RootBone::new(name.clone(), parent_id);
            if let Some(x) = x {
                root_bone.x = *x;
            }
            if let Some(y) = y {
                root_bone.y = *y;
            }
            if let Some(length) = length {
                root_bone.length = *length;
            }
            objects.push(Box::new(root_bone));
            name_to_index.insert(name.clone(), object_index);
            if let Some(children) = children {
                for child in children {
                    append_object(
                        child,
                        object_index,
                        artboard_start,
                        objects,
                        name_to_index,
                        artboard_name_to_index,
                        current_artboard_name,
                    )?;
                }
            }
        }
        ObjectSpec::Skin {
            name,
            xx,
            yx,
            xy,
            yy,
            tx,
            ty,
            children,
        } => {
            let mut skin = Skin::new(name.clone(), parent_id);
            if let Some(xx) = xx {
                skin.xx = *xx;
            }
            if let Some(yx) = yx {
                skin.yx = *yx;
            }
            if let Some(xy) = xy {
                skin.xy = *xy;
            }
            if let Some(yy) = yy {
                skin.yy = *yy;
            }
            if let Some(tx) = tx {
                skin.tx = *tx;
            }
            if let Some(ty) = ty {
                skin.ty = *ty;
            }
            objects.push(Box::new(skin));
            name_to_index.insert(name.clone(), object_index);
            if let Some(children) = children {
                for child in children {
                    append_object(
                        child,
                        object_index,
                        artboard_start,
                        objects,
                        name_to_index,
                        artboard_name_to_index,
                        current_artboard_name,
                    )?;
                }
            }
        }
        ObjectSpec::Tendon {
            name,
            bone,
            xx,
            yx,
            xy,
            yy,
            tx,
            ty,
        } => {
            let mut tendon = Tendon::new(name.clone(), parent_id);
            if let Some(bone_name) = bone {
                let bone_global = *name_to_index.get(bone_name).ok_or_else(|| {
                    format!("tendon '{}' references unknown bone '{}'", name, bone_name)
                })?;
                tendon.bone_id = (bone_global - artboard_start) as u64;
            }
            if let Some(xx) = xx {
                tendon.xx = *xx;
            }
            if let Some(yx) = yx {
                tendon.yx = *yx;
            }
            if let Some(xy) = xy {
                tendon.xy = *xy;
            }
            if let Some(yy) = yy {
                tendon.yy = *yy;
            }
            if let Some(tx) = tx {
                tendon.tx = *tx;
            }
            if let Some(ty) = ty {
                tendon.ty = *ty;
            }
            objects.push(Box::new(tendon));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::Weight {
            name,
            values,
            indices,
        } => {
            let mut weight = Weight::new(name.clone(), parent_id);
            if let Some(values) = values {
                weight.values = *values;
            }
            if let Some(indices) = indices {
                weight.indices = *indices;
            }
            objects.push(Box::new(weight));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::CubicWeight {
            name,
            in_values,
            in_indices,
            out_values,
            out_indices,
        } => {
            let mut cubic_weight = CubicWeight::new(name.clone(), parent_id);
            if let Some(in_values) = in_values {
                cubic_weight.in_values = *in_values;
            }
            if let Some(in_indices) = in_indices {
                cubic_weight.in_indices = *in_indices;
            }
            if let Some(out_values) = out_values {
                cubic_weight.out_values = *out_values;
            }
            if let Some(out_indices) = out_indices {
                cubic_weight.out_indices = *out_indices;
            }
            objects.push(Box::new(cubic_weight));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::IkConstraint {
            name,
            target,
            strength,
            invert_direction,
            parent_bone_count,
        } => {
            let mut ik = IKConstraint::new(name.clone(), parent_id);
            if let Some(target_name) = target {
                let target_global = *name_to_index.get(target_name).ok_or_else(|| {
                    format!(
                        "ik_constraint '{}' references unknown target '{}'",
                        name, target_name
                    )
                })?;
                ik.target_id = (target_global - artboard_start) as u64;
            }
            if let Some(s) = strength {
                ik.strength = *s;
            }
            if let Some(inv) = invert_direction {
                ik.invert_direction = *inv;
            }
            if let Some(pbc) = parent_bone_count {
                ik.parent_bone_count = *pbc;
            }
            objects.push(Box::new(ik));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::DistanceConstraint {
            name,
            target,
            strength,
            distance,
            mode_value,
        } => {
            let mut dc = DistanceConstraint::new(name.clone(), parent_id);
            if let Some(target_name) = target {
                let target_global = *name_to_index.get(target_name).ok_or_else(|| {
                    format!(
                        "distance_constraint '{}' references unknown target '{}'",
                        name, target_name
                    )
                })?;
                dc.target_id = (target_global - artboard_start) as u64;
            }
            if let Some(s) = strength {
                dc.strength = *s;
            }
            if let Some(d) = distance {
                dc.distance = *d;
            }
            if let Some(mv) = mode_value {
                dc.mode_value = *mv;
            }
            objects.push(Box::new(dc));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::TransformConstraint {
            name,
            target,
            strength,
            source_space_value,
            dest_space_value,
            origin_x,
            origin_y,
        } => {
            let mut tc = TransformConstraint::new(name.clone(), parent_id);
            if let Some(target_name) = target {
                let target_global = *name_to_index.get(target_name).ok_or_else(|| {
                    format!(
                        "transform_constraint '{}' references unknown target '{}'",
                        name, target_name
                    )
                })?;
                tc.target_id = (target_global - artboard_start) as u64;
            }
            if let Some(s) = strength {
                tc.strength = *s;
            }
            if let Some(ssv) = source_space_value {
                tc.source_space_value = *ssv;
            }
            if let Some(dsv) = dest_space_value {
                tc.dest_space_value = *dsv;
            }
            if let Some(ox) = origin_x {
                tc.origin_x = *ox;
            }
            if let Some(oy) = origin_y {
                tc.origin_y = *oy;
            }
            objects.push(Box::new(tc));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::TranslationConstraint {
            name,
            target,
            strength,
            source_space_value,
            dest_space_value,
            copy_factor,
            min_value,
            max_value,
            offset,
            does_copy,
            min,
            max,
            min_max_space_value,
            copy_factor_y,
            min_value_y,
            max_value_y,
            does_copy_y,
            min_y,
            max_y,
        } => {
            let mut tlc = TranslationConstraint::new(name.clone(), parent_id);
            if let Some(target_name) = target {
                let target_global = *name_to_index.get(target_name).ok_or_else(|| {
                    format!(
                        "translation_constraint '{}' references unknown target '{}'",
                        name, target_name
                    )
                })?;
                tlc.target_id = (target_global - artboard_start) as u64;
            }
            if let Some(s) = strength {
                tlc.strength = *s;
            }
            if let Some(v) = source_space_value {
                tlc.source_space_value = *v;
            }
            if let Some(v) = dest_space_value {
                tlc.dest_space_value = *v;
            }
            if let Some(v) = copy_factor {
                tlc.copy_factor = *v;
            }
            if let Some(v) = min_value {
                tlc.min_value = *v;
            }
            if let Some(v) = max_value {
                tlc.max_value = *v;
            }
            if let Some(v) = offset {
                tlc.offset = *v;
            }
            if let Some(v) = does_copy {
                tlc.does_copy = *v;
            }
            if let Some(v) = min {
                tlc.min = *v;
            }
            if let Some(v) = max {
                tlc.max = *v;
            }
            if let Some(v) = min_max_space_value {
                tlc.min_max_space_value = *v;
            }
            if let Some(v) = copy_factor_y {
                tlc.copy_factor_y = *v;
            }
            if let Some(v) = min_value_y {
                tlc.min_value_y = *v;
            }
            if let Some(v) = max_value_y {
                tlc.max_value_y = *v;
            }
            if let Some(v) = does_copy_y {
                tlc.does_copy_y = *v;
            }
            if let Some(v) = min_y {
                tlc.min_y = *v;
            }
            if let Some(v) = max_y {
                tlc.max_y = *v;
            }
            objects.push(Box::new(tlc));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::ScaleConstraint {
            name,
            target,
            strength,
            source_space_value,
            dest_space_value,
            copy_factor,
            min_value,
            max_value,
            offset,
            does_copy,
            min,
            max,
            min_max_space_value,
            copy_factor_y,
            min_value_y,
            max_value_y,
            does_copy_y,
            min_y,
            max_y,
        } => {
            let mut sc = ScaleConstraint::new(name.clone(), parent_id);
            if let Some(target_name) = target {
                let target_global = *name_to_index.get(target_name).ok_or_else(|| {
                    format!(
                        "scale_constraint '{}' references unknown target '{}'",
                        name, target_name
                    )
                })?;
                sc.target_id = (target_global - artboard_start) as u64;
            }
            if let Some(s) = strength {
                sc.strength = *s;
            }
            if let Some(v) = source_space_value {
                sc.source_space_value = *v;
            }
            if let Some(v) = dest_space_value {
                sc.dest_space_value = *v;
            }
            if let Some(v) = copy_factor {
                sc.copy_factor = *v;
            }
            if let Some(v) = min_value {
                sc.min_value = *v;
            }
            if let Some(v) = max_value {
                sc.max_value = *v;
            }
            if let Some(v) = offset {
                sc.offset = *v;
            }
            if let Some(v) = does_copy {
                sc.does_copy = *v;
            }
            if let Some(v) = min {
                sc.min = *v;
            }
            if let Some(v) = max {
                sc.max = *v;
            }
            if let Some(v) = min_max_space_value {
                sc.min_max_space_value = *v;
            }
            if let Some(v) = copy_factor_y {
                sc.copy_factor_y = *v;
            }
            if let Some(v) = min_value_y {
                sc.min_value_y = *v;
            }
            if let Some(v) = max_value_y {
                sc.max_value_y = *v;
            }
            if let Some(v) = does_copy_y {
                sc.does_copy_y = *v;
            }
            if let Some(v) = min_y {
                sc.min_y = *v;
            }
            if let Some(v) = max_y {
                sc.max_y = *v;
            }
            objects.push(Box::new(sc));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::RotationConstraint {
            name,
            target,
            strength,
            source_space_value,
            dest_space_value,
            copy_factor,
            min_value,
            max_value,
            offset,
            does_copy,
            min,
            max,
            min_max_space_value,
        } => {
            let mut rc = RotationConstraint::new(name.clone(), parent_id);
            if let Some(target_name) = target {
                let target_global = *name_to_index.get(target_name).ok_or_else(|| {
                    format!(
                        "rotation_constraint '{}' references unknown target '{}'",
                        name, target_name
                    )
                })?;
                rc.target_id = (target_global - artboard_start) as u64;
            }
            if let Some(s) = strength {
                rc.strength = *s;
            }
            if let Some(v) = source_space_value {
                rc.source_space_value = *v;
            }
            if let Some(v) = dest_space_value {
                rc.dest_space_value = *v;
            }
            if let Some(v) = copy_factor {
                rc.copy_factor = *v;
            }
            if let Some(v) = min_value {
                rc.min_value = *v;
            }
            if let Some(v) = max_value {
                rc.max_value = *v;
            }
            if let Some(v) = offset {
                rc.offset = *v;
            }
            if let Some(v) = does_copy {
                rc.does_copy = *v;
            }
            if let Some(v) = min {
                rc.min = *v;
            }
            if let Some(v) = max {
                rc.max = *v;
            }
            if let Some(v) = min_max_space_value {
                rc.min_max_space_value = *v;
            }
            objects.push(Box::new(rc));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::Text {
            name,
            align_value,
            sizing_value,
            overflow_value,
            width,
            height,
            origin_x,
            origin_y,
            paragraph_spacing,
            origin_value,
            children,
        } => {
            let mut text = Text::new(name.clone(), parent_id);
            if let Some(v) = align_value {
                text.align_value = *v;
            }
            if let Some(v) = sizing_value {
                text.sizing_value = *v;
            }
            if let Some(v) = overflow_value {
                text.overflow_value = *v;
            }
            if let Some(v) = width {
                text.width = *v;
            }
            if let Some(v) = height {
                text.height = *v;
            }
            if let Some(v) = origin_x {
                text.origin_x = *v;
            }
            if let Some(v) = origin_y {
                text.origin_y = *v;
            }
            if let Some(v) = paragraph_spacing {
                text.paragraph_spacing = *v;
            }
            if let Some(v) = origin_value {
                text.origin_value = *v;
            }
            objects.push(Box::new(text));
            name_to_index.insert(name.clone(), object_index);
            if let Some(children) = children {
                for child in children {
                    append_object(
                        child,
                        object_index,
                        artboard_start,
                        objects,
                        name_to_index,
                        artboard_name_to_index,
                        current_artboard_name,
                    )?;
                }
            }
        }
        ObjectSpec::TextStyle {
            name,
            font_size,
            line_height,
            letter_spacing,
            font_asset_id,
        } => {
            let mut style = TextStyle::new(name.clone(), parent_id);
            if let Some(v) = font_size {
                style.font_size = *v;
            }
            if let Some(v) = line_height {
                style.line_height = *v;
            }
            if let Some(v) = letter_spacing {
                style.letter_spacing = *v;
            }
            if let Some(v) = font_asset_id {
                style.font_asset_id = *v;
            }
            objects.push(Box::new(style));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::TextValueRun {
            name,
            text,
            style_id,
        } => {
            let mut run = TextValueRun::new(name.clone(), parent_id, text.clone());
            if let Some(v) = style_id {
                run.style_id = *v;
            }
            objects.push(Box::new(run));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::ImageAsset {
            name,
            asset_id,
            cdn_base_url,
        } => {
            let mut asset = ImageAsset::new(name.clone());
            if let Some(v) = asset_id {
                asset.asset_id = *v;
            }
            if let Some(v) = cdn_base_url {
                asset.cdn_base_url = v.clone();
            }
            objects.push(Box::new(asset));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::FontAsset {
            name,
            asset_id,
            cdn_base_url,
        } => {
            let mut asset = FontAsset::new(name.clone());
            if let Some(v) = asset_id {
                asset.asset_id = *v;
            }
            if let Some(v) = cdn_base_url {
                asset.cdn_base_url = v.clone();
            }
            objects.push(Box::new(asset));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::AudioAsset {
            name,
            asset_id,
            cdn_base_url,
        } => {
            let mut asset = AudioAsset::new(name.clone());
            if let Some(v) = asset_id {
                asset.asset_id = *v;
            }
            if let Some(v) = cdn_base_url {
                asset.cdn_base_url = v.clone();
            }
            objects.push(Box::new(asset));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::LayoutComponent {
            name,
            clip,
            width,
            height,
            style_id,
            fractional_width,
            fractional_height,
            children,
        } => {
            let mut lc = LayoutComponent::new(name.clone(), parent_id);
            if let Some(v) = clip {
                lc.clip = *v;
            }
            if let Some(v) = width {
                lc.width = *v;
            }
            if let Some(v) = height {
                lc.height = *v;
            }
            if let Some(v) = style_id {
                lc.style_id = *v;
            }
            if let Some(v) = fractional_width {
                lc.fractional_width = *v;
            }
            if let Some(v) = fractional_height {
                lc.fractional_height = *v;
            }
            objects.push(Box::new(lc));
            name_to_index.insert(name.clone(), object_index);
            if let Some(children) = children {
                for child in children {
                    append_object(
                        child,
                        object_index,
                        artboard_start,
                        objects,
                        name_to_index,
                        artboard_name_to_index,
                        current_artboard_name,
                    )?;
                }
            }
        }
        ObjectSpec::LayoutComponentStyle {
            name,
            gap_horizontal,
            gap_vertical,
            max_width,
            max_height,
            min_width,
            min_height,
            border_left,
            border_right,
            border_top,
            border_bottom,
            margin_left,
            margin_right,
            margin_top,
            margin_bottom,
            padding_left,
            padding_right,
            padding_top,
            padding_bottom,
            position_left,
            position_right,
            position_top,
            position_bottom,
            flex_direction,
            flex_wrap,
            align_items,
            align_content,
            justify_content,
            display,
            position_type,
            overflow,
            intrinsically_sized,
            width_units,
            height_units,
            flex_grow,
            flex_shrink,
            flex_basis,
            aspect_ratio,
        } => {
            let mut style = LayoutComponentStyle::new(name.clone(), parent_id);
            if let Some(v) = gap_horizontal {
                style.gap_horizontal = *v;
            }
            if let Some(v) = gap_vertical {
                style.gap_vertical = *v;
            }
            if let Some(v) = max_width {
                style.max_width = *v;
            }
            if let Some(v) = max_height {
                style.max_height = *v;
            }
            if let Some(v) = min_width {
                style.min_width = *v;
            }
            if let Some(v) = min_height {
                style.min_height = *v;
            }
            if let Some(v) = border_left {
                style.border_left = *v;
            }
            if let Some(v) = border_right {
                style.border_right = *v;
            }
            if let Some(v) = border_top {
                style.border_top = *v;
            }
            if let Some(v) = border_bottom {
                style.border_bottom = *v;
            }
            if let Some(v) = margin_left {
                style.margin_left = *v;
            }
            if let Some(v) = margin_right {
                style.margin_right = *v;
            }
            if let Some(v) = margin_top {
                style.margin_top = *v;
            }
            if let Some(v) = margin_bottom {
                style.margin_bottom = *v;
            }
            if let Some(v) = padding_left {
                style.padding_left = *v;
            }
            if let Some(v) = padding_right {
                style.padding_right = *v;
            }
            if let Some(v) = padding_top {
                style.padding_top = *v;
            }
            if let Some(v) = padding_bottom {
                style.padding_bottom = *v;
            }
            if let Some(v) = position_left {
                style.position_left = *v;
            }
            if let Some(v) = position_right {
                style.position_right = *v;
            }
            if let Some(v) = position_top {
                style.position_top = *v;
            }
            if let Some(v) = position_bottom {
                style.position_bottom = *v;
            }
            if let Some(v) = flex_direction {
                style.flex_direction = *v;
            }
            if let Some(v) = flex_wrap {
                style.flex_wrap = *v;
            }
            if let Some(v) = align_items {
                style.align_items = *v;
            }
            if let Some(v) = align_content {
                style.align_content = *v;
            }
            if let Some(v) = justify_content {
                style.justify_content = *v;
            }
            if let Some(v) = display {
                style.display = *v;
            }
            if let Some(v) = position_type {
                style.position_type = *v;
            }
            if let Some(v) = overflow {
                style.overflow = *v;
            }
            if let Some(v) = intrinsically_sized {
                style.intrinsically_sized = *v;
            }
            if let Some(v) = width_units {
                style.width_units = *v;
            }
            if let Some(v) = height_units {
                style.height_units = *v;
            }
            if let Some(v) = flex_grow {
                style.flex_grow = *v;
            }
            if let Some(v) = flex_shrink {
                style.flex_shrink = *v;
            }
            if let Some(v) = flex_basis {
                style.flex_basis = *v;
            }
            if let Some(v) = aspect_ratio {
                style.aspect_ratio = *v;
            }
            objects.push(Box::new(style));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::ViewModel { name, children } => {
            let vm = ViewModel::new(name.clone(), parent_id);
            objects.push(Box::new(vm));
            name_to_index.insert(name.clone(), object_index);
            if let Some(children) = children {
                for child in children {
                    append_object(
                        child,
                        object_index,
                        artboard_start,
                        objects,
                        name_to_index,
                        artboard_name_to_index,
                        current_artboard_name,
                    )?;
                }
            }
        }
        ObjectSpec::ViewModelProperty {
            name,
            property_type_value,
        } => {
            let vmp =
                ViewModelProperty::new(name.clone(), parent_id, property_type_value.unwrap_or(0));
            objects.push(Box::new(vmp));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::DataBind {
            property_key,
            flags,
            converter_id,
        } => {
            let mut db = DataBind::new(*property_key, *flags);
            if let Some(v) = converter_id {
                db.converter_id = *v;
            }
            objects.push(Box::new(db));
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

fn collect_nested_artboard_refs(children: &[ObjectSpec]) -> Vec<String> {
    let mut refs = Vec::new();
    for child in children {
        match child {
            ObjectSpec::NestedArtboard {
                source_artboard, ..
            } => {
                refs.push(source_artboard.clone());
            }
            ObjectSpec::Shape { children, .. }
            | ObjectSpec::Fill { children, .. }
            | ObjectSpec::Stroke { children, .. }
            | ObjectSpec::Bone { children, .. }
            | ObjectSpec::RootBone { children, .. }
            | ObjectSpec::Skin { children, .. }
            | ObjectSpec::Text { children, .. }
            | ObjectSpec::LayoutComponent { children, .. }
            | ObjectSpec::ViewModel { children, .. } => {
                if let Some(kids) = children {
                    refs.extend(collect_nested_artboard_refs(kids));
                }
            }
            _ => {}
        }
    }
    refs
}

fn detect_artboard_cycles(artboard_deps: &HashMap<String, Vec<String>>) -> Result<(), String> {
    for start in artboard_deps.keys() {
        let mut visited: HashSet<&str> = HashSet::new();
        let mut stack = vec![start.as_str()];
        visited.insert(start.as_str());
        while let Some(current) = stack.pop() {
            if let Some(deps) = artboard_deps.get(current) {
                for dep in deps {
                    if dep == start {
                        return Err(format!(
                            "circular nested artboard reference detected: '{}' eventually references itself",
                            start
                        ));
                    }
                    if !visited.contains(dep.as_str()) {
                        visited.insert(dep.as_str());
                        stack.push(dep.as_str());
                    }
                }
            }
        }
    }
    Ok(())
}

fn validate_scene_spec(spec: &SceneSpec) -> Result<(), String> {
    if spec.scene_format_version != SCENE_FORMAT_VERSION {
        return Err(format!(
            "unsupported scene_format_version {} (expected {})",
            spec.scene_format_version, SCENE_FORMAT_VERSION
        ));
    }

    let artboard_specs = resolve_artboards(spec)?;

    let mut artboard_names: HashSet<String> = HashSet::new();
    for artboard_spec in &artboard_specs {
        if artboard_names.contains(&artboard_spec.name) {
            return Err(format!("duplicate artboard name '{}'", artboard_spec.name));
        }
        artboard_names.insert(artboard_spec.name.clone());
        validate_artboard_spec(artboard_spec)?;
    }

    let mut artboard_deps: HashMap<String, Vec<String>> = HashMap::new();
    for artboard_spec in &artboard_specs {
        let refs = collect_nested_artboard_refs(&artboard_spec.children);
        if !refs.is_empty() {
            artboard_deps.insert(artboard_spec.name.clone(), refs);
        }
    }
    detect_artboard_cycles(&artboard_deps)?;

    Ok(())
}

fn validate_artboard_spec(artboard_spec: &ArtboardSpec) -> Result<(), String> {
    if artboard_spec.width < 0.0 {
        return Err(format!(
            "artboard '{}' width must be non-negative",
            artboard_spec.name
        ));
    }
    if artboard_spec.height < 0.0 {
        return Err(format!(
            "artboard '{}' height must be non-negative",
            artboard_spec.name
        ));
    }

    resolve_artboard_dimensions(artboard_spec)?;

    let mut object_names: HashSet<String> = HashSet::new();
    for child in &artboard_spec.children {
        validate_object_spec(child, &mut object_names, &ParentKind::Artboard)?;
    }

    let mut animation_names: HashSet<String> = HashSet::new();
    if let Some(animations) = &artboard_spec.animations {
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

    if let Some(state_machines) = &artboard_spec.state_machines {
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

enum ParentKind {
    Artboard,
    Shape,
    Fill,
    Stroke,
    Gradient,
    Bone,
    Text,
    LayoutComponent,
    ViewModel,
}

fn validate_object_spec(
    spec: &ObjectSpec,
    object_names: &mut HashSet<String>,
    parent_kind: &ParentKind,
) -> Result<(), String> {
    match spec {
        ObjectSpec::Shape { name, children, .. } => {
            ensure_unique_name(name, object_names)?;
            if let Some(children) = children {
                for child in children {
                    validate_object_spec(child, object_names, &ParentKind::Shape)?;
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
                    validate_object_spec(child, object_names, &ParentKind::Fill)?;
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
                    validate_object_spec(child, object_names, &ParentKind::Stroke)?;
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
                    validate_object_spec(child, object_names, &ParentKind::Gradient)?;
                }
            }
        }
        ObjectSpec::RadialGradient { name, children, .. } => {
            ensure_unique_name(name, object_names)?;
            if let Some(children) = children {
                for child in children {
                    validate_object_spec(child, object_names, &ParentKind::Gradient)?;
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
            name,
            start,
            end,
            mode,
            ..
        } => {
            ensure_unique_name(name, object_names)?;
            if !matches!(parent_kind, ParentKind::Fill | ParentKind::Stroke) {
                return Err(format!(
                    "trim_path '{}' must be a child of a fill or stroke, not a shape or artboard",
                    name
                ));
            }
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
        ObjectSpec::NestedArtboard { name, .. } => {
            ensure_unique_name(name, object_names)?;
        }
        ObjectSpec::Bone { name, children, .. } => {
            ensure_unique_name(name, object_names)?;
            if let Some(children) = children {
                for child in children {
                    validate_object_spec(child, object_names, &ParentKind::Bone)?;
                }
            }
        }
        ObjectSpec::RootBone { name, children, .. } => {
            ensure_unique_name(name, object_names)?;
            if let Some(children) = children {
                for child in children {
                    validate_object_spec(child, object_names, &ParentKind::Bone)?;
                }
            }
        }
        ObjectSpec::Skin { name, children, .. } => {
            ensure_unique_name(name, object_names)?;
            if let Some(children) = children {
                for child in children {
                    validate_object_spec(child, object_names, &ParentKind::Bone)?;
                }
            }
        }
        ObjectSpec::Tendon { name, bone, .. } => {
            ensure_unique_name(name, object_names)?;
            if bone.is_none() {
                return Err(format!("tendon '{}' must reference a bone", name));
            }
        }
        ObjectSpec::Weight { name, .. } | ObjectSpec::CubicWeight { name, .. } => {
            ensure_unique_name(name, object_names)?;
        }
        ObjectSpec::IkConstraint { name, target, .. }
        | ObjectSpec::DistanceConstraint { name, target, .. }
        | ObjectSpec::TransformConstraint { name, target, .. }
        | ObjectSpec::TranslationConstraint { name, target, .. }
        | ObjectSpec::ScaleConstraint { name, target, .. }
        | ObjectSpec::RotationConstraint { name, target, .. } => {
            ensure_unique_name(name, object_names)?;
            if target.is_none() {
                return Err(format!("constraint '{}' must specify a target", name));
            }
        }
        ObjectSpec::Text { name, children, .. } => {
            ensure_unique_name(name, object_names)?;
            if let Some(children) = children {
                for child in children {
                    validate_object_spec(child, object_names, &ParentKind::Text)?;
                }
            }
        }
        ObjectSpec::TextStyle { name, .. } | ObjectSpec::TextValueRun { name, .. } => {
            ensure_unique_name(name, object_names)?;
        }
        ObjectSpec::ImageAsset { name, .. }
        | ObjectSpec::FontAsset { name, .. }
        | ObjectSpec::AudioAsset { name, .. } => {
            ensure_unique_name(name, object_names)?;
        }
        ObjectSpec::LayoutComponent { name, children, .. } => {
            ensure_unique_name(name, object_names)?;
            if let Some(children) = children {
                for child in children {
                    validate_object_spec(child, object_names, &ParentKind::LayoutComponent)?;
                }
            }
        }
        ObjectSpec::LayoutComponentStyle { name, .. } => {
            ensure_unique_name(name, object_names)?;
        }
        ObjectSpec::ViewModel { name, children, .. } => {
            ensure_unique_name(name, object_names)?;
            if let Some(children) = children {
                for child in children {
                    validate_object_spec(child, object_names, &ParentKind::ViewModel)?;
                }
            }
        }
        ObjectSpec::ViewModelProperty { name, .. } => {
            ensure_unique_name(name, object_names)?;
        }
        ObjectSpec::DataBind { .. } => {}
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
        let ab = scene.artboard.as_ref().unwrap();
        assert_eq!(ab.name, "Main");
        assert_eq!(ab.width, 500.0);
        assert_eq!(ab.children.len(), 0);
        assert!(ab.animations.is_none());
        assert_eq!(scene.scene_format_version, 1);
    }

    #[test]
    fn test_build_scene_missing_version() {
        let json = r#"{
            "artboard": {
                "name": "Main",
                "width": 500.0,
                "height": 500.0,
                "children": []
            }
        }"#;

        let parsed: Result<SceneSpec, _> = serde_json::from_str(json);
        assert!(parsed.is_err());
    }

    #[test]
    fn test_build_scene_zero_dimensions() {
        let spec = SceneSpec {
            scene_format_version: 1,
            artboard: Some(ArtboardSpec {
                name: "Main".to_string(),
                preset: None,
                width: 0.0,
                height: 0.0,
                children: vec![],
                animations: None,
                state_machines: None,
            }),
            artboards: None,
        };

        let err = match build_scene(&spec) {
            Ok(_) => panic!("expected zero dimensions error"),
            Err(err) => err,
        };
        assert!(err.contains("must specify non-zero width and height or a preset"));
    }

    #[test]
    fn test_build_scene_empty_children() {
        let spec = SceneSpec {
            scene_format_version: 1,
            artboard: Some(ArtboardSpec {
                name: "Main".to_string(),
                preset: None,
                width: 500.0,
                height: 500.0,
                children: vec![],
                animations: None,
                state_machines: None,
            }),
            artboards: None,
        };

        let objects = build_scene(&spec).unwrap();
        assert_eq!(objects.len(), 2);
        assert_eq!(objects[0].type_key(), type_keys::BACKBOARD);
        assert_eq!(objects[1].type_key(), type_keys::ARTBOARD);
    }

    #[test]
    fn test_build_scene_duplicate_names() {
        let spec = SceneSpec {
            scene_format_version: 1,
            artboard: None,
            artboards: Some(vec![
                ArtboardSpec {
                    name: "A".to_string(),
                    preset: None,
                    width: 100.0,
                    height: 100.0,
                    children: vec![ObjectSpec::Shape {
                        name: "dup_shape".to_string(),
                        x: None,
                        y: None,
                        children: None,
                    }],
                    animations: None,
                    state_machines: None,
                },
                ArtboardSpec {
                    name: "B".to_string(),
                    preset: None,
                    width: 100.0,
                    height: 100.0,
                    children: vec![ObjectSpec::Shape {
                        name: "dup_shape".to_string(),
                        x: None,
                        y: None,
                        children: None,
                    }],
                    animations: None,
                    state_machines: None,
                },
            ]),
        };

        let objects = build_scene(&spec).unwrap();
        assert_eq!(objects[0].type_key(), type_keys::BACKBOARD);
        assert_eq!(objects[1].type_key(), type_keys::ARTBOARD);
        assert_eq!(objects[2].type_key(), type_keys::SHAPE);
        assert_eq!(objects[3].type_key(), type_keys::ARTBOARD);
        assert_eq!(objects[4].type_key(), type_keys::SHAPE);
    }

    #[test]
    fn test_reject_unsupported_scene_format_version() {
        let spec = SceneSpec {
            scene_format_version: 2,
            artboard: Some(ArtboardSpec {
                name: "Main".to_string(),
                preset: None,
                width: 500.0,
                height: 500.0,
                children: vec![],
                animations: None,
                state_machines: None,
            }),
            artboards: None,
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
        let ab = scene.artboard.as_ref().unwrap();
        assert_eq!(ab.children.len(), 1);
        match &ab.children[0] {
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
            artboard: Some(ArtboardSpec {
                name: "Main".to_string(),
                preset: None,
                width: 500.0,
                height: 500.0,
                children: vec![],
                animations: None,
                state_machines: None,
            }),
            artboards: None,
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
            artboard: Some(ArtboardSpec {
                name: "Main".to_string(),
                preset: None,
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
            }),
            artboards: None,
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
            artboard: Some(ArtboardSpec {
                name: "Main".to_string(),
                preset: None,
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
            }),
            artboards: None,
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
            artboard: Some(ArtboardSpec {
                name: "Main".to_string(),
                preset: None,
                width: 100.0,
                height: 100.0,
                children: vec![ObjectSpec::SolidColor {
                    name: "bad_color".to_string(),
                    color: "not-a-color".to_string(),
                }],
                animations: None,
                state_machines: None,
            }),
            artboards: None,
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
            artboard: Some(ArtboardSpec {
                name: "Main".to_string(),
                preset: None,
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
            }),
            artboards: None,
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
            artboard: Some(ArtboardSpec {
                name: "Main".to_string(),
                preset: None,
                width: 100.0,
                height: 100.0,
                children: vec![ObjectSpec::Path {
                    name: "path_1".to_string(),
                    path_flags: Some(3),
                }],
                animations: None,
                state_machines: None,
            }),
            artboards: None,
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
            artboard: Some(ArtboardSpec {
                name: "Main".to_string(),
                preset: None,
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
            }),
            artboards: None,
        };

        let objects = build_scene(&spec).unwrap();
        assert!(objects.len() >= 4);
    }

    #[test]
    fn test_json_value_to_color_rejects_overflow() {
        let value = serde_json::json!(4294967296u64);
        assert!(json_value_to_color(&value).is_none());
    }

    #[test]
    fn test_trim_path_rejects_shape_parent() {
        let spec = SceneSpec {
            scene_format_version: 1,
            artboard: Some(ArtboardSpec {
                name: "Main".to_string(),
                preset: None,
                width: 100.0,
                height: 100.0,
                children: vec![ObjectSpec::Shape {
                    name: "shape_1".to_string(),
                    x: None,
                    y: None,
                    children: Some(vec![ObjectSpec::TrimPath {
                        name: "trim_1".to_string(),
                        start: None,
                        end: None,
                        offset: None,
                        mode: None,
                    }]),
                }],
                animations: None,
                state_machines: None,
            }),
            artboards: None,
        };

        match build_scene(&spec) {
            Err(err) => assert!(
                err.contains("must be a child of a fill or stroke"),
                "unexpected error: {}",
                err
            ),
            Ok(_) => panic!("expected error for TrimPath under Shape"),
        }
    }

    #[test]
    fn test_trim_path_accepts_stroke_parent() {
        let spec = SceneSpec {
            scene_format_version: 1,
            artboard: Some(ArtboardSpec {
                name: "Main".to_string(),
                preset: None,
                width: 100.0,
                height: 100.0,
                children: vec![ObjectSpec::Shape {
                    name: "shape_1".to_string(),
                    x: None,
                    y: None,
                    children: Some(vec![ObjectSpec::Stroke {
                        name: "stroke_1".to_string(),
                        thickness: Some(2.0),
                        cap: None,
                        join: None,
                        children: Some(vec![
                            ObjectSpec::SolidColor {
                                name: "color_1".to_string(),
                                color: "FF0000FF".to_string(),
                            },
                            ObjectSpec::TrimPath {
                                name: "trim_1".to_string(),
                                start: None,
                                end: Some(0.75),
                                offset: None,
                                mode: Some(1),
                            },
                        ]),
                    }]),
                }],
                animations: None,
                state_machines: None,
            }),
            artboards: None,
        };

        build_scene(&spec).unwrap();
    }

    #[test]
    fn test_build_multi_artboard_scene() {
        let spec = SceneSpec {
            scene_format_version: 1,
            artboard: None,
            artboards: Some(vec![
                ArtboardSpec {
                    name: "A".to_string(),
                    preset: None,
                    width: 400.0,
                    height: 300.0,
                    children: vec![ObjectSpec::Shape {
                        name: "shape_a".to_string(),
                        x: None,
                        y: None,
                        children: Some(vec![ObjectSpec::Ellipse {
                            name: "ellipse_a".to_string(),
                            width: 50.0,
                            height: 50.0,
                            origin_x: None,
                            origin_y: None,
                        }]),
                    }],
                    animations: None,
                    state_machines: None,
                },
                ArtboardSpec {
                    name: "B".to_string(),
                    preset: None,
                    width: 800.0,
                    height: 600.0,
                    children: vec![ObjectSpec::Shape {
                        name: "shape_b".to_string(),
                        x: None,
                        y: None,
                        children: Some(vec![ObjectSpec::Rectangle {
                            name: "rect_b".to_string(),
                            width: 100.0,
                            height: 80.0,
                            corner_radius: None,
                            origin_x: None,
                            origin_y: None,
                        }]),
                    }],
                    animations: None,
                    state_machines: None,
                },
            ]),
        };

        let objects = build_scene(&spec).unwrap();

        assert_eq!(objects[0].type_key(), type_keys::BACKBOARD);
        assert_eq!(objects[1].type_key(), type_keys::ARTBOARD);
        assert_eq!(objects[2].type_key(), type_keys::SHAPE);
        assert_eq!(objects[3].type_key(), type_keys::ELLIPSE);
        assert_eq!(objects[4].type_key(), type_keys::ARTBOARD);
        assert_eq!(objects[5].type_key(), type_keys::SHAPE);
        assert_eq!(objects[6].type_key(), type_keys::RECTANGLE);

        let shape_a_parent = objects[2]
            .properties()
            .into_iter()
            .find(|p| p.key == property_keys::COMPONENT_PARENT_ID)
            .unwrap();
        assert_eq!(shape_a_parent.value, PropertyValue::UInt(0));

        let ellipse_a_parent = objects[3]
            .properties()
            .into_iter()
            .find(|p| p.key == property_keys::COMPONENT_PARENT_ID)
            .unwrap();
        assert_eq!(ellipse_a_parent.value, PropertyValue::UInt(1));

        let shape_b_parent = objects[5]
            .properties()
            .into_iter()
            .find(|p| p.key == property_keys::COMPONENT_PARENT_ID)
            .unwrap();
        assert_eq!(shape_b_parent.value, PropertyValue::UInt(0));

        let rect_b_parent = objects[6]
            .properties()
            .into_iter()
            .find(|p| p.key == property_keys::COMPONENT_PARENT_ID)
            .unwrap();
        assert_eq!(rect_b_parent.value, PropertyValue::UInt(1));
    }

    #[test]
    fn test_build_scene_with_nested_artboard() {
        let spec = SceneSpec {
            scene_format_version: 1,
            artboard: None,
            artboards: Some(vec![
                ArtboardSpec {
                    name: "Main".to_string(),
                    preset: None,
                    width: 500.0,
                    height: 500.0,
                    children: vec![ObjectSpec::NestedArtboard {
                        name: "embedded_component".to_string(),
                        source_artboard: "Component".to_string(),
                        x: Some(100.0),
                        y: Some(100.0),
                    }],
                    animations: None,
                    state_machines: None,
                },
                ArtboardSpec {
                    name: "Component".to_string(),
                    preset: None,
                    width: 200.0,
                    height: 200.0,
                    children: vec![],
                    animations: None,
                    state_machines: None,
                },
            ]),
        };

        let objects = build_scene(&spec).unwrap();
        assert_eq!(objects[0].type_key(), type_keys::BACKBOARD);
        assert_eq!(objects[1].type_key(), type_keys::ARTBOARD);
        assert_eq!(objects[2].type_key(), type_keys::NESTED_ARTBOARD);
        assert_eq!(objects[3].type_key(), type_keys::ARTBOARD);

        let nested_props = objects[2].properties();
        let nested_parent = nested_props
            .iter()
            .find(|p| p.key == property_keys::COMPONENT_PARENT_ID)
            .unwrap();
        assert_eq!(nested_parent.value, PropertyValue::UInt(0));

        let nested_artboard_id = nested_props
            .iter()
            .find(|p| p.key == property_keys::NESTED_ARTBOARD_ARTBOARD_ID)
            .unwrap();
        assert_eq!(nested_artboard_id.value, PropertyValue::UInt(1));
    }

    #[test]
    fn test_nested_artboard_rejects_unknown_source() {
        let spec = SceneSpec {
            scene_format_version: 1,
            artboard: Some(ArtboardSpec {
                name: "Main".to_string(),
                preset: None,
                width: 500.0,
                height: 500.0,
                children: vec![ObjectSpec::NestedArtboard {
                    name: "embedded_component".to_string(),
                    source_artboard: "DoesNotExist".to_string(),
                    x: None,
                    y: None,
                }],
                animations: None,
                state_machines: None,
            }),
            artboards: None,
        };

        let err = match build_scene(&spec) {
            Ok(_) => panic!("expected unknown source artboard error"),
            Err(err) => err,
        };
        assert!(err.contains(
            "nested artboard 'embedded_component' references unknown artboard 'DoesNotExist'"
        ));
    }

    #[test]
    fn test_nested_artboard_rejects_self_reference() {
        let spec = SceneSpec {
            scene_format_version: 1,
            artboard: Some(ArtboardSpec {
                name: "Main".to_string(),
                preset: None,
                width: 500.0,
                height: 500.0,
                children: vec![ObjectSpec::NestedArtboard {
                    name: "embedded_component".to_string(),
                    source_artboard: "Main".to_string(),
                    x: None,
                    y: None,
                }],
                animations: None,
                state_machines: None,
            }),
            artboards: None,
        };

        let err = match build_scene(&spec) {
            Ok(_) => panic!("expected self-reference error"),
            Err(err) => err,
        };
        assert!(err.contains("circular nested artboard reference detected"));
    }

    #[test]
    fn test_nested_artboard_rejects_indirect_cycle() {
        let spec = SceneSpec {
            scene_format_version: 1,
            artboard: None,
            artboards: Some(vec![
                ArtboardSpec {
                    name: "A".to_string(),
                    preset: None,
                    width: 500.0,
                    height: 500.0,
                    children: vec![ObjectSpec::NestedArtboard {
                        name: "embed_b".to_string(),
                        source_artboard: "B".to_string(),
                        x: None,
                        y: None,
                    }],
                    animations: None,
                    state_machines: None,
                },
                ArtboardSpec {
                    name: "B".to_string(),
                    preset: None,
                    width: 500.0,
                    height: 500.0,
                    children: vec![ObjectSpec::NestedArtboard {
                        name: "embed_a".to_string(),
                        source_artboard: "A".to_string(),
                        x: None,
                        y: None,
                    }],
                    animations: None,
                    state_machines: None,
                },
            ]),
        };

        let err = match build_scene(&spec) {
            Ok(_) => panic!("expected indirect cycle error"),
            Err(err) => err,
        };
        assert!(err.contains("circular nested artboard reference detected"));
    }

    #[test]
    fn test_reject_both_artboard_and_artboards() {
        let spec = SceneSpec {
            scene_format_version: 1,
            artboard: Some(ArtboardSpec {
                name: "A".to_string(),
                preset: None,
                width: 100.0,
                height: 100.0,
                children: vec![],
                animations: None,
                state_machines: None,
            }),
            artboards: Some(vec![ArtboardSpec {
                name: "B".to_string(),
                preset: None,
                width: 100.0,
                height: 100.0,
                children: vec![],
                animations: None,
                state_machines: None,
            }]),
        };

        let err = match build_scene(&spec) {
            Ok(_) => panic!("expected error"),
            Err(e) => e,
        };
        assert!(err.contains("not both"));
    }

    #[test]
    fn test_reject_neither_artboard_nor_artboards() {
        let spec = SceneSpec {
            scene_format_version: 1,
            artboard: None,
            artboards: None,
        };

        let err = match build_scene(&spec) {
            Ok(_) => panic!("expected error"),
            Err(e) => e,
        };
        assert!(err.contains("must specify"));
    }

    #[test]
    fn test_reject_empty_artboards_array() {
        let spec = SceneSpec {
            scene_format_version: 1,
            artboard: None,
            artboards: Some(vec![]),
        };

        let err = match build_scene(&spec) {
            Ok(_) => panic!("expected error"),
            Err(e) => e,
        };
        assert!(err.contains("must not be empty"));
    }

    #[test]
    fn test_reject_duplicate_artboard_names() {
        let spec = SceneSpec {
            scene_format_version: 1,
            artboard: None,
            artboards: Some(vec![
                ArtboardSpec {
                    name: "Main".to_string(),
                    preset: None,
                    width: 100.0,
                    height: 100.0,
                    children: vec![],
                    animations: None,
                    state_machines: None,
                },
                ArtboardSpec {
                    name: "Main".to_string(),
                    preset: None,
                    width: 200.0,
                    height: 200.0,
                    children: vec![],
                    animations: None,
                    state_machines: None,
                },
            ]),
        };

        let err = match build_scene(&spec) {
            Ok(_) => panic!("expected error"),
            Err(e) => e,
        };
        assert!(err.contains("duplicate artboard name"));
    }

    #[test]
    fn test_multi_artboard_with_animation() {
        let spec = SceneSpec {
            scene_format_version: 1,
            artboard: None,
            artboards: Some(vec![
                ArtboardSpec {
                    name: "A".to_string(),
                    preset: None,
                    width: 100.0,
                    height: 100.0,
                    children: vec![ObjectSpec::Shape {
                        name: "s_a".to_string(),
                        x: None,
                        y: None,
                        children: Some(vec![ObjectSpec::Ellipse {
                            name: "e_a".to_string(),
                            width: 50.0,
                            height: 50.0,
                            origin_x: None,
                            origin_y: None,
                        }]),
                    }],
                    animations: Some(vec![AnimationSpec {
                        name: "anim_a".to_string(),
                        fps: 60,
                        duration: 60,
                        speed: None,
                        loop_type: None,
                        interpolators: None,
                        keyframes: vec![KeyframeGroupSpec {
                            object: "e_a".to_string(),
                            property: "width".to_string(),
                            frames: vec![
                                KeyframeSpec {
                                    frame: 0,
                                    value: serde_json::json!(50.0),
                                    interpolation: None,
                                    interpolator: None,
                                },
                                KeyframeSpec {
                                    frame: 59,
                                    value: serde_json::json!(100.0),
                                    interpolation: None,
                                    interpolator: None,
                                },
                            ],
                        }],
                    }]),
                    state_machines: None,
                },
                ArtboardSpec {
                    name: "B".to_string(),
                    preset: None,
                    width: 200.0,
                    height: 200.0,
                    children: vec![ObjectSpec::Shape {
                        name: "s_b".to_string(),
                        x: None,
                        y: None,
                        children: Some(vec![ObjectSpec::Ellipse {
                            name: "e_b".to_string(),
                            width: 80.0,
                            height: 80.0,
                            origin_x: None,
                            origin_y: None,
                        }]),
                    }],
                    animations: None,
                    state_machines: None,
                },
            ]),
        };

        let objects = build_scene(&spec).unwrap();

        let keyed_obj = objects
            .iter()
            .find(|o| o.type_key() == type_keys::KEYED_OBJECT)
            .unwrap();
        let keyed_obj_id = keyed_obj
            .properties()
            .into_iter()
            .find(|p| p.key == property_keys::KEYED_OBJECT_ID)
            .unwrap();
        assert_eq!(keyed_obj_id.value, PropertyValue::UInt(2));

        let artboard_b = objects
            .iter()
            .filter(|o| o.type_key() == type_keys::ARTBOARD)
            .nth(1)
            .unwrap();
        let b_name = artboard_b
            .properties()
            .into_iter()
            .find(|p| p.key == property_keys::COMPONENT_NAME)
            .unwrap();
        assert_eq!(b_name.value, PropertyValue::String("B".to_string()));
    }

    #[test]
    fn test_multi_artboard_keyed_object_id_resets_per_artboard() {
        let spec = SceneSpec {
            scene_format_version: 1,
            artboard: None,
            artboards: Some(vec![
                ArtboardSpec {
                    name: "A".to_string(),
                    preset: None,
                    width: 100.0,
                    height: 100.0,
                    children: vec![ObjectSpec::Shape {
                        name: "s_a".to_string(),
                        x: None,
                        y: None,
                        children: Some(vec![ObjectSpec::Ellipse {
                            name: "e_a".to_string(),
                            width: 50.0,
                            height: 50.0,
                            origin_x: None,
                            origin_y: None,
                        }]),
                    }],
                    animations: Some(vec![AnimationSpec {
                        name: "anim_a".to_string(),
                        fps: 60,
                        duration: 60,
                        speed: None,
                        loop_type: None,
                        interpolators: None,
                        keyframes: vec![KeyframeGroupSpec {
                            object: "e_a".to_string(),
                            property: "width".to_string(),
                            frames: vec![
                                KeyframeSpec {
                                    frame: 0,
                                    value: serde_json::json!(50.0),
                                    interpolation: None,
                                    interpolator: None,
                                },
                                KeyframeSpec {
                                    frame: 59,
                                    value: serde_json::json!(100.0),
                                    interpolation: None,
                                    interpolator: None,
                                },
                            ],
                        }],
                    }]),
                    state_machines: None,
                },
                ArtboardSpec {
                    name: "B".to_string(),
                    preset: None,
                    width: 200.0,
                    height: 200.0,
                    children: vec![ObjectSpec::Shape {
                        name: "s_b".to_string(),
                        x: None,
                        y: None,
                        children: Some(vec![ObjectSpec::Ellipse {
                            name: "e_b".to_string(),
                            width: 80.0,
                            height: 80.0,
                            origin_x: None,
                            origin_y: None,
                        }]),
                    }],
                    animations: Some(vec![AnimationSpec {
                        name: "anim_b".to_string(),
                        fps: 60,
                        duration: 60,
                        speed: None,
                        loop_type: None,
                        interpolators: None,
                        keyframes: vec![KeyframeGroupSpec {
                            object: "e_b".to_string(),
                            property: "height".to_string(),
                            frames: vec![
                                KeyframeSpec {
                                    frame: 0,
                                    value: serde_json::json!(80.0),
                                    interpolation: None,
                                    interpolator: None,
                                },
                                KeyframeSpec {
                                    frame: 59,
                                    value: serde_json::json!(160.0),
                                    interpolation: None,
                                    interpolator: None,
                                },
                            ],
                        }],
                    }]),
                    state_machines: None,
                },
            ]),
        };

        let objects = build_scene(&spec).unwrap();

        let keyed_objects: Vec<_> = objects
            .iter()
            .filter(|o| o.type_key() == type_keys::KEYED_OBJECT)
            .collect();
        assert_eq!(keyed_objects.len(), 2);

        let keyed_a_id = keyed_objects[0]
            .properties()
            .into_iter()
            .find(|p| p.key == property_keys::KEYED_OBJECT_ID)
            .unwrap();
        assert_eq!(keyed_a_id.value, PropertyValue::UInt(2));

        let keyed_b_id = keyed_objects[1]
            .properties()
            .into_iter()
            .find(|p| p.key == property_keys::KEYED_OBJECT_ID)
            .unwrap();
        assert_eq!(keyed_b_id.value, PropertyValue::UInt(2));
    }

    #[test]
    fn test_parse_multi_artboard_json() {
        let json = r#"{
            "scene_format_version": 1,
            "artboards": [
                {
                    "name": "X",
                    "width": 100,
                    "height": 100,
                    "children": []
                },
                {
                    "name": "Y",
                    "width": 200,
                    "height": 200,
                    "children": []
                }
            ]
        }"#;

        let scene: SceneSpec = serde_json::from_str(json).unwrap();
        assert!(scene.artboard.is_none());
        let abs = scene.artboards.as_ref().unwrap();
        assert_eq!(abs.len(), 2);
        assert_eq!(abs[0].name, "X");
        assert_eq!(abs[1].name, "Y");

        let objects = build_scene(&scene).unwrap();
        assert_eq!(objects[0].type_key(), type_keys::BACKBOARD);
        assert_eq!(objects[1].type_key(), type_keys::ARTBOARD);
        assert_eq!(objects[2].type_key(), type_keys::ARTBOARD);
    }

    #[test]
    fn test_single_artboard_json_backward_compatible() {
        let json = r#"{
            "scene_format_version": 1,
            "artboard": {
                "name": "Main",
                "width": 500,
                "height": 500,
                "children": []
            }
        }"#;

        let scene: SceneSpec = serde_json::from_str(json).unwrap();
        assert!(scene.artboard.is_some());
        assert!(scene.artboards.is_none());

        let objects = build_scene(&scene).unwrap();
        assert_eq!(objects.len(), 2);
        assert_eq!(objects[0].type_key(), type_keys::BACKBOARD);
        assert_eq!(objects[1].type_key(), type_keys::ARTBOARD);
    }

    #[test]
    fn test_artboard_preset_resolves_dimensions() {
        let spec = SceneSpec {
            scene_format_version: 1,
            artboard: Some(ArtboardSpec {
                name: "Main".to_string(),
                preset: Some("mobile".to_string()),
                width: 0.0,
                height: 0.0,
                children: vec![],
                animations: None,
                state_machines: None,
            }),
            artboards: None,
        };

        let objects = build_scene(&spec).unwrap();
        let artboard_props = objects[1].properties();
        let width = artboard_props
            .iter()
            .find(|p| p.key == property_keys::LAYOUT_COMPONENT_WIDTH)
            .unwrap();
        let height = artboard_props
            .iter()
            .find(|p| p.key == property_keys::LAYOUT_COMPONENT_HEIGHT)
            .unwrap();

        assert_eq!(
            width.value,
            PropertyValue::Float(ARTBOARD_PRESET_MOBILE_WIDTH)
        );
        assert_eq!(
            height.value,
            PropertyValue::Float(ARTBOARD_PRESET_MOBILE_HEIGHT)
        );
    }

    #[test]
    fn test_artboard_preset_applies_per_dimension_overrides() {
        let spec = SceneSpec {
            scene_format_version: 1,
            artboard: Some(ArtboardSpec {
                name: "Main".to_string(),
                preset: Some("mobile".to_string()),
                width: 800.0,
                height: 0.0,
                children: vec![],
                animations: None,
                state_machines: None,
            }),
            artboards: None,
        };

        let objects = build_scene(&spec).unwrap();
        let artboard_props = objects[1].properties();
        let width = artboard_props
            .iter()
            .find(|p| p.key == property_keys::LAYOUT_COMPONENT_WIDTH)
            .unwrap();
        let height = artboard_props
            .iter()
            .find(|p| p.key == property_keys::LAYOUT_COMPONENT_HEIGHT)
            .unwrap();

        assert_eq!(width.value, PropertyValue::Float(800.0));
        assert_eq!(
            height.value,
            PropertyValue::Float(ARTBOARD_PRESET_MOBILE_HEIGHT)
        );
    }

    #[test]
    fn test_artboard_unknown_preset_rejected() {
        let spec = SceneSpec {
            scene_format_version: 1,
            artboard: Some(ArtboardSpec {
                name: "Main".to_string(),
                preset: Some("watch".to_string()),
                width: 0.0,
                height: 0.0,
                children: vec![],
                animations: None,
                state_machines: None,
            }),
            artboards: None,
        };

        let err = match build_scene(&spec) {
            Ok(_) => panic!("expected unknown preset error"),
            Err(err) => err,
        };
        assert!(err.contains("unknown preset 'watch'"));
    }

    #[test]
    fn test_artboard_requires_dimensions_or_preset() {
        let spec = SceneSpec {
            scene_format_version: 1,
            artboard: Some(ArtboardSpec {
                name: "Main".to_string(),
                preset: None,
                width: 0.0,
                height: 0.0,
                children: vec![],
                animations: None,
                state_machines: None,
            }),
            artboards: None,
        };

        let err = match build_scene(&spec) {
            Ok(_) => panic!("expected missing dimensions error"),
            Err(err) => err,
        };
        assert!(err.contains("must specify non-zero width and height or a preset"));
    }

    #[test]
    fn test_multi_artboard_names_can_overlap_object_names() {
        let spec = SceneSpec {
            scene_format_version: 1,
            artboard: None,
            artboards: Some(vec![
                ArtboardSpec {
                    name: "A".to_string(),
                    preset: None,
                    width: 100.0,
                    height: 100.0,
                    children: vec![ObjectSpec::Node {
                        name: "node_1".to_string(),
                        x: None,
                        y: None,
                    }],
                    animations: None,
                    state_machines: None,
                },
                ArtboardSpec {
                    name: "B".to_string(),
                    preset: None,
                    width: 100.0,
                    height: 100.0,
                    children: vec![ObjectSpec::Node {
                        name: "node_1".to_string(),
                        x: None,
                        y: None,
                    }],
                    animations: None,
                    state_machines: None,
                },
            ]),
        };

        let objects = build_scene(&spec).unwrap();
        assert_eq!(objects.len(), 5);
    }
}
