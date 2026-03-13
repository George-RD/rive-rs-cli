use std::collections::HashMap;

use crate::objects::animation::{
    CubicEaseInterpolator, ElasticInterpolator, KeyFrameBool, KeyFrameCallback, KeyFrameColor,
    KeyFrameDouble, KeyFrameId, KeyFrameString, KeyedObject, KeyedProperty, LinearAnimation,
};
use crate::objects::core::{
    BackingType, RiveObject, is_bool_property, property_backing_type, property_keys,
};

use super::parsers::{
    interpolation_type_from_name, interpolator_def_equals, json_value_to_color, json_value_to_f32,
    json_value_to_string, json_value_to_u64, parse_loop_type, property_key_for_object,
    validate_discrete_keyframe_interpolation,
};
use super::spec::{AnimationSpec, InterpolatorDef};

/// Registers named interpolators from all animations into the object list.
/// Populates `interpolator_name_to_index` and `interpolator_defs` maps.
pub(crate) fn register_interpolators(
    animations: &[AnimationSpec],
    artboard_start: usize,
    objects: &mut Vec<Box<dyn RiveObject>>,
    interpolator_name_to_index: &mut HashMap<String, usize>,
    interpolator_defs: &mut HashMap<String, InterpolatorDef>,
) -> Result<(), String> {
    for animation in animations {
        if let Some(interpolators) = &animation.interpolators {
            for interp in interpolators {
                let interp_def = match interp.interpolation_type.as_deref().unwrap_or("cubic") {
                    "cubic" => InterpolatorDef::Cubic {
                        x1: interp.x1.unwrap_or(0.42),
                        y1: interp.y1.unwrap_or(0.0),
                        x2: interp.x2.unwrap_or(0.58),
                        y2: interp.y2.unwrap_or(1.0),
                    },
                    "elastic" => InterpolatorDef::Elastic {
                        easing_value: interp.easing_value.unwrap_or(1),
                        amplitude: interp.amplitude.unwrap_or(1.0),
                        period: interp.period.unwrap_or(1.0),
                    },
                    other => {
                        return Err(format!(
                            "unknown interpolator type '{}' for '{}'",
                            other, interp.name
                        ));
                    }
                };

                if let Some(stored_def) = interpolator_defs.get(&interp.name) {
                    if !interpolator_def_equals(*stored_def, interp_def) {
                        return Err(format!(
                            "duplicate interpolator '{}' with different parameters",
                            interp.name
                        ));
                    }
                    continue;
                }

                let artboard_local_index = objects.len().checked_sub(artboard_start).ok_or(
                    "internal error: interpolator index precedes artboard start".to_string(),
                )?;
                interpolator_name_to_index.insert(interp.name.clone(), artboard_local_index);
                interpolator_defs.insert(interp.name.clone(), interp_def);
                match interp_def {
                    InterpolatorDef::Cubic { x1, y1, x2, y2 } => {
                        objects.push(Box::new(CubicEaseInterpolator::new(x1, y1, x2, y2)));
                    }
                    InterpolatorDef::Elastic {
                        easing_value,
                        amplitude,
                        period,
                    } => {
                        objects.push(Box::new(ElasticInterpolator {
                            easing_value,
                            amplitude,
                            period,
                        }));
                    }
                }
            }
        }
    }
    Ok(())
}

/// Builds LinearAnimation objects with their KeyedObject/KeyedProperty/KeyFrame children.
pub(crate) fn build_animations(
    animations: &[AnimationSpec],
    artboard_start: usize,
    objects: &mut Vec<Box<dyn RiveObject>>,
    object_name_to_index: &HashMap<String, usize>,
    animation_name_to_index: &mut HashMap<String, usize>,
    interpolator_name_to_index: &HashMap<String, usize>,
) -> Result<(), String> {
    for (animation_list_index, animation) in animations.iter().enumerate() {
        let mut linear =
            LinearAnimation::new(animation.name.clone(), animation.fps, animation.duration);
        if let Some(speed) = animation.speed {
            linear.speed = speed;
        }
        if let Some(loop_type) = &animation.loop_type {
            linear.loop_type = parse_loop_type(loop_type)?;
        }

        objects.push(Box::new(linear));
        animation_name_to_index.insert(animation.name.clone(), animation_list_index);

        for group in &animation.keyframes {
            let object_index = *object_name_to_index.get(&group.object).ok_or_else(|| {
                format!("unknown object referenced in keyframes: '{}'", group.object)
            })?;
            let keyed_object_id = object_index
                .checked_sub(artboard_start)
                .ok_or("internal error: keyed object index precedes artboard start".to_string())?;
            objects.push(Box::new(KeyedObject {
                object_id: keyed_object_id as u64,
            }));

            let property_key =
                property_key_for_object(&group.property, objects[object_index].type_key())
                    .ok_or_else(|| {
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

                match property_backing_type(property_key) {
                    Some(BackingType::Color) => {
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
                    }
                    Some(BackingType::String) => {
                        validate_discrete_keyframe_interpolation(
                            &group.object,
                            &group.property,
                            frame.frame,
                            frame.interpolation.as_deref(),
                            frame.interpolator.as_deref(),
                            interp_type,
                            interp_id,
                        )?;
                        let value = json_value_to_string(&frame.value).ok_or_else(|| {
                            format!(
                                "invalid string keyframe value for object '{}' property '{}' at frame {}",
                                group.object, group.property, frame.frame
                            )
                        })?;
                        objects.push(Box::new(KeyFrameString {
                            frame: frame.frame,
                            value,
                        }));
                    }
                    Some(BackingType::UInt) => {
                        if property_key == property_keys::EVENT_TRIGGER {
                            objects.push(Box::new(KeyFrameCallback { frame: frame.frame }));
                            continue;
                        }
                        let value = json_value_to_u64(&frame.value).ok_or_else(|| {
                            format!(
                                "invalid integer keyframe value for object '{}' property '{}' at frame {}",
                                group.object, group.property, frame.frame
                            )
                        })?;
                        if is_bool_property(property_key) {
                            validate_discrete_keyframe_interpolation(
                                &group.object,
                                &group.property,
                                frame.frame,
                                frame.interpolation.as_deref(),
                                frame.interpolator.as_deref(),
                                interp_type,
                                interp_id,
                            )?;
                            objects.push(Box::new(KeyFrameBool {
                                frame: frame.frame,
                                value: value != 0,
                            }));
                        } else {
                            let mut kf = KeyFrameId::new(frame.frame, value);
                            kf.interpolation_type = interp_type;
                            kf.interpolator_id = interp_id;
                            objects.push(Box::new(kf));
                        }
                    }
                    _ => {
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
    Ok(())
}
