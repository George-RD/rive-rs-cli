use std::collections::HashMap;

use crate::objects::artboard::{Artboard, Backboard};
use crate::objects::core::RiveObject;

use super::animations::{build_animations, register_interpolators};
use super::objects::append_object;
use super::spec::{InterpolatorDef, SceneSpec};
use super::state_machines::build_state_machines;
use super::validation::validate_scene_spec;

// Re-export spec types for public API and test visibility via `use super::*`.
#[allow(unused_imports)]
pub use super::spec::{
    ArtboardSpec, AnimationSpec, BlendState1DChildSpec, BlendStateChildSpec,
    BlendStateDirectChildSpec, ConditionSpec, InputSpec, InterpolatorSpec, KeyframeGroupSpec,
    KeyframeSpec, LayerSpec, ListenerActionSpec, ObjectSpec, StateMachineListenerSpec,
    StateMachineSpec, StateSpec, TextModifierGroupChildSpec, TextStyleChildSpec, TransitionChildSpec,
    TransitionSpec,
};

// Re-export parser functions for test visibility via `use super::*`.
#[allow(unused_imports)]
pub(crate) use super::parsers::{
    condition_op_is_valid, input_is_trigger, interpolation_type_from_name, interpolator_def_equals,
    json_value_to_color, json_value_to_f32, json_value_to_string, json_value_to_u64, parse_color,
    parse_condition_op, parse_fill_rule, parse_loop_type, parse_stroke_cap, parse_stroke_join,
    parse_trim_mode, property_key_for_object, property_key_from_name,
    validate_discrete_keyframe_interpolation,
};

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

pub(crate) fn resolve_artboard_dimensions(
    artboard_spec: &ArtboardSpec,
) -> Result<(f32, f32), String> {
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

pub(crate) fn resolve_artboards(spec: &SceneSpec) -> Result<Vec<&ArtboardSpec>, String> {
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
        let mut interpolator_defs: HashMap<String, InterpolatorDef> = HashMap::new();

        if let Some(animations) = &artboard_spec.animations {
            for (animation_list_index, animation) in animations.iter().enumerate() {
                animation_name_to_index.insert(animation.name.clone(), animation_list_index);
            }
        }

        for child in &artboard_spec.children {
            append_object(
                child,
                artboard_start,
                artboard_start,
                &mut objects,
                &mut object_name_to_index,
                &artboard_name_to_index,
                &artboard_spec.name,
                &animation_name_to_index,
            )?;
        }

        if let Some(animations) = &artboard_spec.animations {
            register_interpolators(
                animations,
                artboard_start,
                &mut objects,
                &mut interpolator_name_to_index,
                &mut interpolator_defs,
            )?;

            build_animations(
                animations,
                artboard_start,
                &mut objects,
                &object_name_to_index,
                &mut animation_name_to_index,
                &interpolator_name_to_index,
            )?;
        }

        if let Some(state_machines) = &artboard_spec.state_machines {
            build_state_machines(
                state_machines,
                artboard_start,
                &mut objects,
                &object_name_to_index,
                &animation_name_to_index,
            )?;
        }
    }

    Ok(objects)
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
                            is_visible: None,
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
                    loop_type: Some(serde_json::json!(1)),
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
    fn test_build_scene_with_solo_and_key_frame_id() {
        let spec = SceneSpec {
            scene_format_version: 1,
            artboard: Some(ArtboardSpec {
                name: "Main".to_string(),
                preset: None,
                width: 300.0,
                height: 300.0,
                children: vec![ObjectSpec::Solo {
                    name: "SoloRoot".to_string(),
                    x: None,
                    y: None,
                    active_component: Some("ChildShape".to_string()),
                    children: Some(vec![ObjectSpec::Shape {
                        name: "ChildShape".to_string(),
                        x: None,
                        y: None,
                        children: Some(vec![ObjectSpec::Ellipse {
                            name: "ChildPath".to_string(),
                            width: 40.0,
                            height: 40.0,
                            origin_x: None,
                            origin_y: None,
                        }]),
                    }]),
                }],
                animations: Some(vec![AnimationSpec {
                    name: "switch".to_string(),
                    fps: 60,
                    duration: 60,
                    speed: None,
                    loop_type: None,
                    interpolators: None,
                    keyframes: vec![KeyframeGroupSpec {
                        object: "SoloRoot".to_string(),
                        property: "active_component_id".to_string(),
                        frames: vec![
                            KeyframeSpec {
                                frame: 0,
                                value: serde_json::json!(0),
                                interpolation: None,
                                interpolator: None,
                            },
                            KeyframeSpec {
                                frame: 59,
                                value: serde_json::json!(2),
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
        assert!(objects.iter().any(|o| o.type_key() == type_keys::SOLO));
        assert!(
            objects
                .iter()
                .any(|o| o.type_key() == type_keys::KEY_FRAME_ID)
        );
    }

    #[test]
    fn test_build_scene_with_state_machine_listener() {
        let spec = SceneSpec {
            scene_format_version: 1,
            artboard: Some(ArtboardSpec {
                name: "Main".to_string(),
                preset: None,
                width: 200.0,
                height: 200.0,
                children: vec![ObjectSpec::Shape {
                    name: "Target".to_string(),
                    x: None,
                    y: None,
                    children: Some(vec![ObjectSpec::Ellipse {
                        name: "TargetPath".to_string(),
                        width: 20.0,
                        height: 20.0,
                        origin_x: None,
                        origin_y: None,
                    }]),
                }],
                animations: None,
                state_machines: Some(vec![StateMachineSpec {
                    name: "Logic".to_string(),
                    inputs: Some(vec![InputSpec::Bool {
                        name: "is_on".to_string(),
                        value: false,
                    }]),
                    listeners: Some(vec![StateMachineListenerSpec {
                        target: "Target".to_string(),
                        listener_type_value: Some(1),
                        actions: Some(vec![ListenerActionSpec::BoolChange {
                            input: "is_on".to_string(),
                            value: Some(serde_json::json!(true)),
                        }]),
                    }]),
                    layers: vec![LayerSpec {
                        states: vec![StateSpec::Entry, StateSpec::Exit],
                        transitions: None,
                    }],
                }]),
            }),
            artboards: None,
        };

        let objects = build_scene(&spec).unwrap();
        assert!(
            objects
                .iter()
                .any(|o| o.type_key() == type_keys::STATE_MACHINE_LISTENER)
        );
        assert!(
            objects
                .iter()
                .any(|o| o.type_key() == type_keys::LISTENER_BOOL_CHANGE)
        );
    }

    #[test]
    fn test_build_scene_with_nested_state_machine_object() {
        let spec = SceneSpec {
            scene_format_version: 1,
            artboard: Some(ArtboardSpec {
                name: "Main".to_string(),
                preset: None,
                width: 300.0,
                height: 300.0,
                children: vec![ObjectSpec::NestedStateMachine {
                    name: "NestedSM".to_string(),
                    animation: "switch".to_string(),
                }],
                animations: Some(vec![AnimationSpec {
                    name: "switch".to_string(),
                    fps: 60,
                    duration: 30,
                    speed: None,
                    loop_type: None,
                    interpolators: None,
                    keyframes: vec![],
                }]),
                state_machines: None,
            }),
            artboards: None,
        };

        let objects = build_scene(&spec).unwrap();
        assert!(
            objects
                .iter()
                .any(|o| o.type_key() == type_keys::NESTED_STATE_MACHINE)
        );
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
    fn test_text_style_children_reject_invalid_variant_during_deserialization() {
        let spec = serde_json::json!({
            "scene_format_version": 1,
            "artboard": {
                "name": "Main",
                "width": 100.0,
                "height": 100.0,
                "children": [{
                    "type": "text",
                    "name": "Label",
                    "children": [{
                        "type": "text_style",
                        "name": "Style",
                        "children": [{
                            "type": "image",
                            "name": "Icon",
                            "asset_id": 0
                        }]
                    }]
                }]
            }
        });

        let err = serde_json::from_value::<SceneSpec>(spec).expect_err("expected parse failure");
        assert!(err.to_string().contains("unknown variant `image`"));
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
                    listeners: None,
                    layers: vec![LayerSpec {
                        states: vec![StateSpec::Entry, StateSpec::Exit],
                        transitions: Some(vec![TransitionSpec {
                            from: 0,
                            to: 3,
                            duration: None,
                            conditions: None,
                            children: None,
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
    fn test_build_scene_rejects_non_number_blend_input() {
        let spec = SceneSpec {
            scene_format_version: 1,
            artboard: Some(ArtboardSpec {
                name: "Main".to_string(),
                preset: None,
                width: 100.0,
                height: 100.0,
                children: vec![],
                animations: Some(vec![AnimationSpec {
                    name: "anim".to_string(),
                    fps: 60,
                    duration: 1,
                    speed: None,
                    loop_type: None,
                    interpolators: None,
                    keyframes: vec![],
                }]),
                state_machines: Some(vec![StateMachineSpec {
                    name: "sm".to_string(),
                    inputs: Some(vec![InputSpec::Bool {
                        name: "enabled".to_string(),
                        value: false,
                    }]),
                    listeners: None,
                    layers: vec![LayerSpec {
                        states: vec![
                            StateSpec::Entry,
                            StateSpec::BlendState1d {
                                input_id: Some(0),
                                children: Some(vec![BlendState1DChildSpec::BlendAnimation1D {
                                    animation_id: 0,
                                    value: Some(0.0),
                                }]),
                            },
                            StateSpec::Exit,
                        ],
                        transitions: None,
                    }],
                }]),
            }),
            artboards: None,
        };

        let err = match build_scene(&spec) {
            Ok(_) => panic!("expected non-number blend input error"),
            Err(err) => err,
        };
        assert!(err.contains("must reference a number input, found bool"));
    }

    #[test]
    fn test_build_scene_rejects_invalid_transition_view_model_condition_op_value() {
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
                    listeners: None,
                    layers: vec![LayerSpec {
                        states: vec![StateSpec::Entry, StateSpec::Exit],
                        transitions: Some(vec![TransitionSpec {
                            from: 0,
                            to: 1,
                            duration: None,
                            conditions: None,
                            children: Some(vec![
                                TransitionChildSpec::TransitionViewModelCondition {
                                    op_value: Some(6),
                                },
                            ]),
                        }]),
                    }],
                }]),
            }),
            artboards: None,
        };

        let err = match build_scene(&spec) {
            Ok(_) => panic!("expected invalid transition view model condition error"),
            Err(err) => err,
        };
        assert!(err.contains("transition_view_model_condition op_value 6 out of range"));
    }

    #[test]
    fn test_build_scene_rejects_invalid_condition_operator() {
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
                    inputs: Some(vec![InputSpec::Bool {
                        name: "enabled".to_string(),
                        value: false,
                    }]),
                    listeners: None,
                    layers: vec![LayerSpec {
                        states: vec![StateSpec::Entry, StateSpec::Exit],
                        transitions: Some(vec![TransitionSpec {
                            from: 0,
                            to: 1,
                            duration: None,
                            conditions: Some(vec![ConditionSpec {
                                input: "enabled".to_string(),
                                op: Some("gtee".to_string()),
                                value: Some(serde_json::json!(true)),
                            }]),
                            children: None,
                        }]),
                    }],
                }]),
            }),
            artboards: None,
        };

        let err = match build_scene(&spec) {
            Ok(_) => panic!("expected invalid condition operator error"),
            Err(err) => err,
        };
        assert!(err.contains("unknown condition operator 'gtee'"));
    }

    #[test]
    fn test_build_scene_rejects_missing_view_model_instance_reference() {
        let spec = SceneSpec {
            scene_format_version: 1,
            artboard: Some(ArtboardSpec {
                name: "Main".to_string(),
                preset: None,
                width: 100.0,
                height: 100.0,
                children: vec![ObjectSpec::ViewModelInstance {
                    view_model_id: None,
                }],
                animations: None,
                state_machines: None,
            }),
            artboards: None,
        };

        let err = match build_scene(&spec) {
            Ok(_) => panic!("expected missing view model reference error"),
            Err(err) => err,
        };
        assert!(err.contains("view_model_instance must specify view_model_id"));
    }

    #[test]
    fn test_build_scene_rejects_bool_keyframe_interpolation() {
        let spec = SceneSpec {
            scene_format_version: 1,
            artboard: Some(ArtboardSpec {
                name: "Main".to_string(),
                preset: None,
                width: 100.0,
                height: 100.0,
                children: vec![ObjectSpec::ClippingShape {
                    name: "Clip".to_string(),
                    source: None,
                    fill_rule: None,
                    is_visible: Some(true),
                }],
                animations: Some(vec![AnimationSpec {
                    name: "toggle".to_string(),
                    fps: 60,
                    duration: 1,
                    speed: None,
                    loop_type: None,
                    interpolators: None,
                    keyframes: vec![KeyframeGroupSpec {
                        object: "Clip".to_string(),
                        property: "is_visible".to_string(),
                        frames: vec![KeyframeSpec {
                            frame: 0,
                            value: serde_json::json!(true),
                            interpolation: Some("linear".to_string()),
                            interpolator: None,
                        }],
                    }],
                }]),
                state_machines: None,
            }),
            artboards: None,
        };

        let err = match build_scene(&spec) {
            Ok(_) => panic!("expected bool interpolation error"),
            Err(err) => err,
        };
        assert!(err.contains("unsupported interpolation"));
    }

    #[test]
    fn test_build_scene_rejects_string_keyframe_interpolation() {
        let spec = SceneSpec {
            scene_format_version: 1,
            artboard: Some(ArtboardSpec {
                name: "Main".to_string(),
                preset: None,
                width: 100.0,
                height: 100.0,
                children: vec![ObjectSpec::Text {
                    name: "Label".to_string(),
                    align_value: None,
                    sizing_value: None,
                    overflow_value: None,
                    width: None,
                    height: None,
                    origin_x: None,
                    origin_y: None,
                    paragraph_spacing: None,
                    origin_value: None,
                    children: Some(vec![ObjectSpec::TextValueRun {
                        name: "Run".to_string(),
                        text: "Hello".to_string(),
                        style_id: None,
                    }]),
                }],
                animations: Some(vec![AnimationSpec {
                    name: "rewrite".to_string(),
                    fps: 60,
                    duration: 1,
                    speed: None,
                    loop_type: None,
                    interpolators: None,
                    keyframes: vec![KeyframeGroupSpec {
                        object: "Run".to_string(),
                        property: "text".to_string(),
                        frames: vec![KeyframeSpec {
                            frame: 0,
                            value: serde_json::json!("World"),
                            interpolation: Some("cubic".to_string()),
                            interpolator: None,
                        }],
                    }],
                }]),
                state_machines: None,
            }),
            artboards: None,
        };

        let err = match build_scene(&spec) {
            Ok(_) => panic!("expected string interpolation error"),
            Err(err) => err,
        };
        assert!(err.contains("unsupported interpolation"));
    }

    #[test]
    fn test_text_modifier_group_keyframe_properties_use_text_modifier_keys() {
        assert_eq!(
            property_key_for_object("width", type_keys::TEXT),
            Some(property_keys::TEXT_WIDTH)
        );
        assert_eq!(
            property_key_for_object("height", type_keys::TEXT),
            Some(property_keys::TEXT_HEIGHT)
        );
        assert_eq!(
            property_key_for_object("origin_x", type_keys::TEXT),
            Some(property_keys::TEXT_ORIGIN_X)
        );
        assert_eq!(
            property_key_for_object("origin_y", type_keys::TEXT),
            Some(property_keys::TEXT_ORIGIN_Y)
        );
        assert_eq!(
            property_key_for_object("paragraph_spacing", type_keys::TEXT),
            Some(property_keys::TEXT_PARAGRAPH_SPACING)
        );
        assert_eq!(
            property_key_for_object("width", type_keys::LAYOUT_COMPONENT),
            Some(property_keys::LAYOUT_COMPONENT_WIDTH)
        );
        assert_eq!(
            property_key_for_object("height", type_keys::LAYOUT_COMPONENT),
            Some(property_keys::LAYOUT_COMPONENT_HEIGHT)
        );
        assert_eq!(
            property_key_for_object("font_size", type_keys::TEXT_STYLE),
            Some(property_keys::TEXT_STYLE_FONT_SIZE)
        );
        assert_eq!(
            property_key_for_object("line_height", type_keys::TEXT_STYLE),
            Some(property_keys::TEXT_STYLE_LINE_HEIGHT)
        );
        assert_eq!(
            property_key_for_object("letter_spacing", type_keys::TEXT_STYLE),
            Some(property_keys::TEXT_STYLE_LETTER_SPACING)
        );
        assert_eq!(
            property_key_for_object("origin_x", type_keys::TEXT_MODIFIER_GROUP),
            Some(property_keys::TEXT_MODIFIER_GROUP_ORIGIN_X)
        );
        assert_eq!(
            property_key_for_object("origin_y", type_keys::TEXT_MODIFIER_GROUP),
            Some(property_keys::TEXT_MODIFIER_GROUP_ORIGIN_Y)
        );
        assert_eq!(
            property_key_for_object("opacity", type_keys::TEXT_MODIFIER_GROUP),
            Some(property_keys::TEXT_MODIFIER_GROUP_OPACITY)
        );
        assert_eq!(
            property_key_for_object("x", type_keys::TEXT_MODIFIER_GROUP),
            Some(property_keys::TEXT_MODIFIER_GROUP_X)
        );
        assert_eq!(
            property_key_for_object("y", type_keys::TEXT_MODIFIER_GROUP),
            Some(property_keys::TEXT_MODIFIER_GROUP_Y)
        );
        assert_eq!(
            property_key_for_object("rotation", type_keys::TEXT_MODIFIER_GROUP),
            Some(property_keys::TEXT_MODIFIER_GROUP_ROTATION)
        );
        assert_eq!(
            property_key_for_object("scale_x", type_keys::TEXT_MODIFIER_GROUP),
            Some(property_keys::TEXT_MODIFIER_GROUP_SCALE_X)
        );
        assert_eq!(
            property_key_for_object("scale_y", type_keys::TEXT_MODIFIER_GROUP),
            Some(property_keys::TEXT_MODIFIER_GROUP_SCALE_Y)
        );
    }

    #[test]
    fn test_build_scene_rejects_standalone_text_leaf_variants() {
        let text_modifier_range = SceneSpec {
            scene_format_version: 1,
            artboard: Some(ArtboardSpec {
                name: "Main".to_string(),
                preset: None,
                width: 100.0,
                height: 100.0,
                children: vec![ObjectSpec::TextModifierRange {
                    units_value: None,
                    type_value: None,
                    mode_value: None,
                    modify_from: None,
                    modify_to: None,
                    strength: None,
                    clamp: None,
                    falloff_from: None,
                    falloff_to: None,
                    offset: None,
                    run_id: None,
                }],
                animations: None,
                state_machines: None,
            }),
            artboards: None,
        };
        let err = match build_scene(&text_modifier_range) {
            Ok(_) => panic!("expected standalone text_modifier_range error"),
            Err(err) => err,
        };
        assert!(err.contains("text_modifier_range must be nested"));

        let text_variation_modifier = SceneSpec {
            scene_format_version: 1,
            artboard: Some(ArtboardSpec {
                name: "Main".to_string(),
                preset: None,
                width: 100.0,
                height: 100.0,
                children: vec![ObjectSpec::TextVariationModifier {
                    axis_tag: Some(0),
                    axis_value: Some(0.0),
                }],
                animations: None,
                state_machines: None,
            }),
            artboards: None,
        };
        let err = match build_scene(&text_variation_modifier) {
            Ok(_) => panic!("expected standalone text_variation_modifier error"),
            Err(err) => err,
        };
        assert!(err.contains("text_variation_modifier must be nested"));

        let text_style_feature = SceneSpec {
            scene_format_version: 1,
            artboard: Some(ArtboardSpec {
                name: "Main".to_string(),
                preset: None,
                width: 100.0,
                height: 100.0,
                children: vec![ObjectSpec::TextStyleFeature {
                    tag: Some(0),
                    feature_value: Some(0),
                }],
                animations: None,
                state_machines: None,
            }),
            artboards: None,
        };
        let err = match build_scene(&text_style_feature) {
            Ok(_) => panic!("expected standalone text_style_feature error"),
            Err(err) => err,
        };
        assert!(err.contains("text_style_feature must be nested"));
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
    fn test_build_scene_with_points_path_object() {
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
                    children: Some(vec![ObjectSpec::PointsPath {
                        name: "points_path_1".to_string(),
                        x: None,
                        y: None,
                        is_closed: Some(true),
                        path_flags: Some(3),
                        children: Some(vec![
                            ObjectSpec::StraightVertex {
                                name: "v1".to_string(),
                                x: Some(0.0),
                                y: Some(0.0),
                                radius: None,
                            },
                            ObjectSpec::StraightVertex {
                                name: "v2".to_string(),
                                x: Some(10.0),
                                y: Some(0.0),
                                radius: None,
                            },
                        ]),
                    }]),
                }],
                animations: None,
                state_machines: None,
            }),
            artboards: None,
        };

        let objects = build_scene(&spec).unwrap();
        assert_eq!(objects[3].type_key(), type_keys::POINTS_PATH);
        assert_eq!(objects[4].type_key(), type_keys::STRAIGHT_VERTEX);
        assert_eq!(objects[5].type_key(), type_keys::STRAIGHT_VERTEX);
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
    fn test_parse_stroke_cap_accepts_string_integer_and_rejects_invalid() {
        assert_eq!(parse_stroke_cap(&serde_json::json!("butt")).unwrap(), 0);
        assert_eq!(parse_stroke_cap(&serde_json::json!("round")).unwrap(), 1);
        assert_eq!(parse_stroke_cap(&serde_json::json!("square")).unwrap(), 2);
        assert_eq!(parse_stroke_cap(&serde_json::json!(2)).unwrap(), 2);
        assert!(parse_stroke_cap(&serde_json::json!("flat")).is_err());
        assert!(parse_stroke_cap(&serde_json::json!(true)).is_err());
    }

    #[test]
    fn test_parse_stroke_join_accepts_string_integer_and_rejects_invalid() {
        assert_eq!(parse_stroke_join(&serde_json::json!("miter")).unwrap(), 0);
        assert_eq!(parse_stroke_join(&serde_json::json!("round")).unwrap(), 1);
        assert_eq!(parse_stroke_join(&serde_json::json!("bevel")).unwrap(), 2);
        assert_eq!(parse_stroke_join(&serde_json::json!(1)).unwrap(), 1);
        assert!(parse_stroke_join(&serde_json::json!("sharp")).is_err());
        assert!(parse_stroke_join(&serde_json::json!(false)).is_err());
    }

    #[test]
    fn test_parse_fill_rule_accepts_string_integer_and_rejects_invalid() {
        assert_eq!(parse_fill_rule(&serde_json::json!("nonzero")).unwrap(), 0);
        assert_eq!(parse_fill_rule(&serde_json::json!("evenodd")).unwrap(), 1);
        assert_eq!(parse_fill_rule(&serde_json::json!(1)).unwrap(), 1);
        assert!(parse_fill_rule(&serde_json::json!("winding")).is_err());
        assert!(parse_fill_rule(&serde_json::json!(null)).is_err());
    }

    #[test]
    fn test_parse_loop_type_accepts_string_integer_and_rejects_invalid() {
        assert_eq!(parse_loop_type(&serde_json::json!("oneshot")).unwrap(), 0);
        assert_eq!(parse_loop_type(&serde_json::json!("loop")).unwrap(), 1);
        assert_eq!(parse_loop_type(&serde_json::json!("pingpong")).unwrap(), 2);
        assert_eq!(parse_loop_type(&serde_json::json!(2)).unwrap(), 2);
        assert!(parse_loop_type(&serde_json::json!("repeat")).is_err());
        assert!(parse_loop_type(&serde_json::json!([])).is_err());
    }

    #[test]
    fn test_parse_trim_mode_accepts_string_integer_and_rejects_invalid() {
        assert_eq!(
            parse_trim_mode(&serde_json::json!("sequential")).unwrap(),
            1
        );
        assert_eq!(
            parse_trim_mode(&serde_json::json!("synchronized")).unwrap(),
            2
        );
        assert_eq!(parse_trim_mode(&serde_json::json!(1)).unwrap(), 1);
        assert_eq!(parse_trim_mode(&serde_json::json!(2)).unwrap(), 2);
        assert!(parse_trim_mode(&serde_json::json!(0)).is_err());
        assert!(parse_trim_mode(&serde_json::json!("sync")).is_err());
    }

    #[test]
    fn test_parse_stroke_cap_negative() {
        assert!(parse_stroke_cap(&serde_json::json!(-1)).is_err());
    }

    #[test]
    fn test_parse_stroke_cap_float() {
        assert!(parse_stroke_cap(&serde_json::json!(1.5)).is_err());
    }

    #[test]
    fn test_parse_stroke_cap_out_of_range() {
        assert!(parse_stroke_cap(&serde_json::json!(3)).is_err());
    }

    #[test]
    fn test_parse_stroke_join_negative() {
        assert!(parse_stroke_join(&serde_json::json!(-1)).is_err());
    }

    #[test]
    fn test_parse_stroke_join_float() {
        assert!(parse_stroke_join(&serde_json::json!(1.5)).is_err());
    }

    #[test]
    fn test_parse_stroke_join_out_of_range() {
        assert!(parse_stroke_join(&serde_json::json!(3)).is_err());
    }

    #[test]
    fn test_parse_fill_rule_negative() {
        assert!(parse_fill_rule(&serde_json::json!(-1)).is_err());
    }

    #[test]
    fn test_parse_fill_rule_float() {
        assert!(parse_fill_rule(&serde_json::json!(1.5)).is_err());
    }

    #[test]
    fn test_parse_fill_rule_out_of_range() {
        assert!(parse_fill_rule(&serde_json::json!(2)).is_err());
    }

    #[test]
    fn test_parse_loop_type_negative() {
        assert!(parse_loop_type(&serde_json::json!(-1)).is_err());
    }

    #[test]
    fn test_parse_loop_type_float() {
        assert!(parse_loop_type(&serde_json::json!(1.5)).is_err());
    }

    #[test]
    fn test_parse_loop_type_out_of_range() {
        assert!(parse_loop_type(&serde_json::json!(3)).is_err());
    }

    #[test]
    fn test_parse_trim_mode_negative() {
        assert!(parse_trim_mode(&serde_json::json!(-1)).is_err());
    }

    #[test]
    fn test_parse_trim_mode_float() {
        assert!(parse_trim_mode(&serde_json::json!(1.5)).is_err());
    }

    #[test]
    fn test_parse_color_hash_and_legacy_formats() {
        assert_eq!(parse_color("#FF0000").unwrap(), 0xFFFF_0000);
        assert_eq!(parse_color("FF0000").unwrap(), 0xFFFF_0000);
        assert_eq!(parse_color("#00FF0066").unwrap(), 0x6600_FF00);
        assert_eq!(parse_color("6600FF00").unwrap(), 0x6600_FF00);
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
                        is_visible: None,
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
                                mode: Some(serde_json::json!(1)),
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
    fn test_image_reference_requires_preceding_asset() {
        let spec = SceneSpec {
            scene_format_version: 1,
            artboard: Some(ArtboardSpec {
                name: "Main".to_string(),
                preset: None,
                width: 100.0,
                height: 100.0,
                children: vec![ObjectSpec::Image {
                    name: "sprite_1".to_string(),
                    asset_id: 0,
                    x: Some(10.0),
                    y: Some(20.0),
                }],
                animations: None,
                state_machines: None,
            }),
            artboards: None,
        };

        match build_scene(&spec) {
            Err(err) => assert!(
                err.contains("references image asset index"),
                "unexpected error: {}",
                err
            ),
            Ok(_) => panic!("expected missing image asset error"),
        }
    }

    #[test]
    fn test_image_reference_accepts_preceding_asset() {
        let spec = SceneSpec {
            scene_format_version: 1,
            artboard: Some(ArtboardSpec {
                name: "Main".to_string(),
                preset: None,
                width: 100.0,
                height: 100.0,
                children: vec![
                    ObjectSpec::ImageAsset {
                        name: "img_asset".to_string(),
                        asset_id: Some(100),
                        cdn_base_url: None,
                    },
                    ObjectSpec::Image {
                        name: "sprite_1".to_string(),
                        asset_id: 0,
                        x: Some(10.0),
                        y: Some(20.0),
                    },
                ],
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

    #[test]
    fn test_clipping_shape_visibility_keyframes_use_clipping_property() {
        let spec = SceneSpec {
            scene_format_version: 1,
            artboard: Some(ArtboardSpec {
                name: "Main".to_string(),
                preset: None,
                width: 100.0,
                height: 100.0,
                children: vec![ObjectSpec::ClippingShape {
                    name: "Clip".to_string(),
                    source: None,
                    fill_rule: None,
                    is_visible: Some(true),
                }],
                animations: Some(vec![AnimationSpec {
                    name: "toggle".to_string(),
                    fps: 60,
                    duration: 1,
                    speed: None,
                    loop_type: None,
                    interpolators: None,
                    keyframes: vec![KeyframeGroupSpec {
                        object: "Clip".to_string(),
                        property: "is_visible".to_string(),
                        frames: vec![KeyframeSpec {
                            frame: 0,
                            value: serde_json::json!(true),
                            interpolation: None,
                            interpolator: None,
                        }],
                    }],
                }]),
                state_machines: None,
            }),
            artboards: None,
        };

        let objects = build_scene(&spec).unwrap();
        let keyed_property = objects
            .iter()
            .find(|object| object.type_key() == type_keys::KEYED_PROPERTY)
            .unwrap()
            .properties();
        assert_eq!(keyed_property.len(), 1);
        assert_eq!(
            keyed_property[0].value,
            PropertyValue::UInt(property_keys::CLIPPING_SHAPE_IS_VISIBLE as u64)
        );
    }
}
