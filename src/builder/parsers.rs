use std::collections::HashSet;

use crate::objects::core::{property_keys, type_keys};

use super::spec::{InputSpec, InterpolatorDef};

pub(crate) fn parse_color(color: &str) -> Result<u32, String> {
    let has_hash = color.starts_with('#');
    let hex = color.trim_start_matches('#');
    if hex.len() == 8 {
        let raw = u32::from_str_radix(hex, 16)
            .map_err(|_| format!("invalid 8-digit color literal: '{}'", color))?;
        if has_hash {
            let r = (raw >> 24) & 0xFF;
            let g = (raw >> 16) & 0xFF;
            let b = (raw >> 8) & 0xFF;
            let a = raw & 0xFF;
            return Ok((a << 24) | (r << 16) | (g << 8) | b);
        }
        return Ok(raw);
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

pub(crate) fn parse_stroke_cap(v: &serde_json::Value) -> Result<u64, String> {
    match v {
        serde_json::Value::Number(n) => {
            let val = n
                .as_u64()
                .ok_or_else(|| format!("invalid cap value: {}", v))?;
            if val > 2 {
                return Err(format!("cap must be 0-2, got {}", val));
            }
            Ok(val)
        }
        serde_json::Value::String(s) => match s.to_lowercase().as_str() {
            "butt" => Ok(0),
            "round" => Ok(1),
            "square" => Ok(2),
            _ => Err(format!(
                "unknown cap type: '{}' (expected butt, round, or square)",
                s
            )),
        },
        _ => Err(format!("cap must be a string or integer, got: {}", v)),
    }
}

pub(crate) fn parse_stroke_join(v: &serde_json::Value) -> Result<u64, String> {
    match v {
        serde_json::Value::Number(n) => {
            let val = n
                .as_u64()
                .ok_or_else(|| format!("invalid join value: {}", v))?;
            if val > 2 {
                return Err(format!("join must be 0-2, got {}", val));
            }
            Ok(val)
        }
        serde_json::Value::String(s) => match s.to_lowercase().as_str() {
            "miter" => Ok(0),
            "round" => Ok(1),
            "bevel" => Ok(2),
            _ => Err(format!(
                "unknown join type: '{}' (expected miter, round, or bevel)",
                s
            )),
        },
        _ => Err(format!("join must be a string or integer, got: {}", v)),
    }
}

pub(crate) fn parse_fill_rule(v: &serde_json::Value) -> Result<u64, String> {
    match v {
        serde_json::Value::Number(n) => {
            let val = n
                .as_u64()
                .ok_or_else(|| format!("invalid fill_rule value: {}", v))?;
            if val > 1 {
                return Err(format!("fill_rule must be 0-1, got {}", val));
            }
            Ok(val)
        }
        serde_json::Value::String(s) => match s.to_lowercase().as_str() {
            "nonzero" => Ok(0),
            "evenodd" => Ok(1),
            _ => Err(format!(
                "unknown fill_rule: '{}' (expected nonzero or evenodd)",
                s
            )),
        },
        _ => Err(format!("fill_rule must be a string or integer, got: {}", v)),
    }
}

pub(crate) fn parse_loop_type(v: &serde_json::Value) -> Result<u64, String> {
    match v {
        serde_json::Value::Number(n) => {
            let val = n
                .as_u64()
                .ok_or_else(|| format!("invalid loop_type value: {}", v))?;
            if val > 2 {
                return Err(format!("loop_type must be 0-2, got {}", val));
            }
            Ok(val)
        }
        serde_json::Value::String(s) => match s.to_lowercase().as_str() {
            "oneshot" => Ok(0),
            "loop" => Ok(1),
            "pingpong" => Ok(2),
            _ => Err(format!(
                "unknown loop_type: '{}' (expected oneshot, loop, or pingpong)",
                s
            )),
        },
        _ => Err(format!("loop_type must be a string or integer, got: {}", v)),
    }
}

pub(crate) fn parse_trim_mode(v: &serde_json::Value) -> Result<u64, String> {
    match v {
        serde_json::Value::Number(n) => {
            let val = n
                .as_u64()
                .ok_or_else(|| format!("invalid mode value: {}", v))?;
            if val != 1 && val != 2 {
                return Err(format!(
                    "mode must be 1 (sequential) or 2 (synchronized), got {}",
                    val
                ));
            }
            Ok(val)
        }
        serde_json::Value::String(s) => match s.to_lowercase().as_str() {
            "sequential" => Ok(1),
            "synchronized" => Ok(2),
            _ => Err(format!(
                "unknown trim mode: '{}' (expected sequential or synchronized)",
                s
            )),
        },
        _ => Err(format!("mode must be a string or integer, got: {}", v)),
    }
}

pub(crate) fn json_value_to_f32(value: &serde_json::Value) -> Option<f32> {
    match value {
        serde_json::Value::Number(number) => number.as_f64().map(|v| v as f32),
        _ => None,
    }
}

pub(crate) fn json_value_to_string(value: &serde_json::Value) -> Option<String> {
    match value {
        serde_json::Value::String(s) => Some(s.clone()),
        _ => None,
    }
}

pub(crate) fn json_value_to_u64(value: &serde_json::Value) -> Option<u64> {
    match value {
        serde_json::Value::Number(number) => number.as_u64(),
        serde_json::Value::Bool(v) => Some(if *v { 1 } else { 0 }),
        _ => None,
    }
}

pub(crate) fn json_value_to_color(value: &serde_json::Value) -> Option<u32> {
    match value {
        serde_json::Value::String(s) => parse_color(s).ok(),
        serde_json::Value::Number(n) => n
            .as_u64()
            .filter(|&v| v <= u32::MAX as u64)
            .map(|v| v as u32),
        _ => None,
    }
}

pub(crate) fn interpolation_type_from_name(name: &str) -> Result<u64, String> {
    match name {
        "hold" => Ok(0),
        "linear" => Ok(1),
        "cubic" => Ok(2),
        _ => Err(format!("unknown interpolation type: '{}'", name)),
    }
}

pub(crate) fn interpolator_def_equals(left: InterpolatorDef, right: InterpolatorDef) -> bool {
    match (left, right) {
        (
            InterpolatorDef::Cubic {
                x1: lx1,
                y1: ly1,
                x2: lx2,
                y2: ly2,
            },
            InterpolatorDef::Cubic {
                x1: rx1,
                y1: ry1,
                x2: rx2,
                y2: ry2,
            },
        ) => lx1 == rx1 && ly1 == ry1 && lx2 == rx2 && ly2 == ry2,
        (
            InterpolatorDef::Elastic {
                easing_value: le,
                amplitude: la,
                period: lp,
            },
            InterpolatorDef::Elastic {
                easing_value: re,
                amplitude: ra,
                period: rp,
            },
        ) => le == re && la == ra && lp == rp,
        _ => false,
    }
}

const TRANSFORM_ANIMATABLE_PROPERTIES: &[(&str, u16)] = &[
    ("x", property_keys::NODE_X),
    ("y", property_keys::NODE_Y),
    ("rotation", property_keys::TRANSFORM_ROTATION),
    ("scale_x", property_keys::TRANSFORM_SCALE_X),
    ("scale_y", property_keys::TRANSFORM_SCALE_Y),
    ("opacity", property_keys::WORLD_TRANSFORM_OPACITY),
];

const PARAMETRIC_ANIMATABLE_PROPERTIES: &[(&str, u16)] = &[
    ("width", property_keys::PARAMETRIC_PATH_WIDTH),
    ("height", property_keys::PARAMETRIC_PATH_HEIGHT),
];

const SOLID_COLOR_ANIMATABLE_PROPERTIES: &[(&str, u16)] =
    &[("color", property_keys::SOLID_COLOR_VALUE)];
const GRADIENT_STOP_ANIMATABLE_PROPERTIES: &[(&str, u16)] =
    &[("color", property_keys::GRADIENT_STOP_COLOR)];

const GRADIENT_ANIMATABLE_PROPERTIES: &[(&str, u16)] = &[
    ("start_x", property_keys::LINEAR_GRADIENT_START_X),
    ("start_y", property_keys::LINEAR_GRADIENT_START_Y),
    ("end_x", property_keys::LINEAR_GRADIENT_END_X),
    ("end_y", property_keys::LINEAR_GRADIENT_END_Y),
    ("opacity", property_keys::LINEAR_GRADIENT_OPACITY),
];
const TRIM_ANIMATABLE_PROPERTIES: &[(&str, u16)] = &[
    ("trim_start", property_keys::TRIM_PATH_START),
    ("trim_end", property_keys::TRIM_PATH_END),
    ("trim_offset", property_keys::TRIM_PATH_OFFSET),
];

const EVENT_ANIMATABLE_PROPERTIES: &[(&str, u16)] = &[("trigger", property_keys::EVENT_TRIGGER)];
const SOLO_ANIMATABLE_PROPERTIES: &[(&str, u16)] = &[(
    "active_component_id",
    property_keys::SOLO_ACTIVE_COMPONENT_ID,
)];

const TEXT_ANIMATABLE_PROPERTIES: &[(&str, u16)] = &[
    ("width", property_keys::TEXT_WIDTH),
    ("height", property_keys::TEXT_HEIGHT),
    ("origin_x", property_keys::TEXT_ORIGIN_X),
    ("origin_y", property_keys::TEXT_ORIGIN_Y),
    ("paragraph_spacing", property_keys::TEXT_PARAGRAPH_SPACING),
];

const TEXT_STYLE_ANIMATABLE_PROPERTIES: &[(&str, u16)] = &[
    ("font_size", property_keys::TEXT_STYLE_FONT_SIZE),
    ("line_height", property_keys::TEXT_STYLE_LINE_HEIGHT),
    ("letter_spacing", property_keys::TEXT_STYLE_LETTER_SPACING),
];

const TEXT_MODIFIER_GROUP_ANIMATABLE_PROPERTIES: &[(&str, u16)] = &[
    (
        "modifier_flags",
        property_keys::TEXT_MODIFIER_GROUP_MODIFIER_FLAGS,
    ),
    ("origin_x", property_keys::TEXT_MODIFIER_GROUP_ORIGIN_X),
    ("origin_y", property_keys::TEXT_MODIFIER_GROUP_ORIGIN_Y),
    ("opacity", property_keys::TEXT_MODIFIER_GROUP_OPACITY),
    ("x", property_keys::TEXT_MODIFIER_GROUP_X),
    ("y", property_keys::TEXT_MODIFIER_GROUP_Y),
    ("rotation", property_keys::TEXT_MODIFIER_GROUP_ROTATION),
    ("scale_x", property_keys::TEXT_MODIFIER_GROUP_SCALE_X),
    ("scale_y", property_keys::TEXT_MODIFIER_GROUP_SCALE_Y),
];

const TEXT_VALUE_RUN_ANIMATABLE_PROPERTIES: &[&str] = &["text"];
const VISIBILITY_ANIMATABLE_PROPERTIES: &[&str] = &["is_visible"];

fn property_key_from(properties: &[(&str, u16)], name: &str) -> Option<u16> {
    properties
        .iter()
        .find_map(|(property, key)| (*property == name).then_some(*key))
}

pub(crate) fn property_key_from_name(name: &str) -> Option<u16> {
    property_key_from(TRANSFORM_ANIMATABLE_PROPERTIES, name)
}

fn property_names(properties: &'static [(&'static str, u16)]) -> Vec<&'static str> {
    properties.iter().map(|(name, _)| *name).collect()
}

fn transform_property_names() -> Vec<&'static str> {
    property_names(TRANSFORM_ANIMATABLE_PROPERTIES)
}

fn extend_property_names(
    target: &mut Vec<&'static str>,
    properties: &'static [(&'static str, u16)],
) {
    target.extend(properties.iter().map(|(name, _)| *name));
}

fn is_parametric_type(type_name: &str) -> bool {
    matches!(
        type_name,
        "ellipse" | "rectangle" | "triangle" | "polygon" | "star"
    )
}

pub(crate) fn animatable_properties_for_object_type(type_name: &str) -> Vec<&'static str> {
    let mut properties = match type_name {
        "text_style" => property_names(TEXT_STYLE_ANIMATABLE_PROPERTIES),
        "text_modifier_group" => property_names(TEXT_MODIFIER_GROUP_ANIMATABLE_PROPERTIES),
        "text" => {
            let mut properties = transform_property_names();
            extend_property_names(&mut properties, TEXT_ANIMATABLE_PROPERTIES);
            properties
        }
        "layout_component" => {
            let mut properties = transform_property_names();
            properties.extend(["width", "height"]);
            properties
        }
        "text_value_run" => TEXT_VALUE_RUN_ANIMATABLE_PROPERTIES.to_vec(),
        "clipping_shape" => VISIBILITY_ANIMATABLE_PROPERTIES.to_vec(),
        "fill" | "stroke" => VISIBILITY_ANIMATABLE_PROPERTIES.to_vec(),
        "solid_color" => property_names(SOLID_COLOR_ANIMATABLE_PROPERTIES),
        "linear_gradient" | "radial_gradient" => property_names(GRADIENT_ANIMATABLE_PROPERTIES),
        "gradient_stop" => {
            let mut properties = property_names(GRADIENT_STOP_ANIMATABLE_PROPERTIES);
            properties.push("position");
            properties
        }
        "trim_path" => property_names(TRIM_ANIMATABLE_PROPERTIES),
        "event" => property_names(EVENT_ANIMATABLE_PROPERTIES),
        "solo" => property_names(SOLO_ANIMATABLE_PROPERTIES),
        _ if is_parametric_type(type_name) => {
            let mut properties = transform_property_names();
            extend_property_names(&mut properties, PARAMETRIC_ANIMATABLE_PROPERTIES);
            properties
        }
        "shape"
        | "node"
        | "image"
        | "nested_artboard"
        | "nested_artboard_leaf"
        | "nested_artboard_layout"
        | "root_bone"
        | "n_sliced_node" => transform_property_names(),
        _ => Vec::new(),
    };
    let mut seen = HashSet::new();
    properties.retain(|name| seen.insert(*name));
    properties
}

pub(crate) fn animatable_property_key_for_object_type(
    type_name: &str,
    property_name: &str,
) -> Option<u16> {
    if !animatable_properties_for_object_type(type_name).contains(&property_name) {
        return None;
    }
    match type_name {
        "text" => property_key_from(TEXT_ANIMATABLE_PROPERTIES, property_name)
            .or_else(|| property_key_from(TRANSFORM_ANIMATABLE_PROPERTIES, property_name)),
        "layout_component" => match property_name {
            "width" => Some(property_keys::LAYOUT_COMPONENT_WIDTH),
            "height" => Some(property_keys::LAYOUT_COMPONENT_HEIGHT),
            _ => property_key_from(TRANSFORM_ANIMATABLE_PROPERTIES, property_name),
        },
        "text_style" => property_key_from(TEXT_STYLE_ANIMATABLE_PROPERTIES, property_name),
        "text_modifier_group" => {
            property_key_from(TEXT_MODIFIER_GROUP_ANIMATABLE_PROPERTIES, property_name)
        }
        "text_value_run" => Some(property_keys::TEXT_VALUE_RUN_TEXT),
        "clipping_shape" => Some(property_keys::CLIPPING_SHAPE_IS_VISIBLE),
        "fill" | "stroke" => Some(property_keys::SHAPE_PAINT_IS_VISIBLE),
        "solid_color" => property_key_from(SOLID_COLOR_ANIMATABLE_PROPERTIES, property_name),
        "linear_gradient" | "radial_gradient" => {
            property_key_from(GRADIENT_ANIMATABLE_PROPERTIES, property_name)
        }
        "gradient_stop" => match property_name {
            "position" => Some(property_keys::GRADIENT_STOP_POSITION),
            _ => property_key_from(GRADIENT_STOP_ANIMATABLE_PROPERTIES, property_name),
        },
        "trim_path" => property_key_from(TRIM_ANIMATABLE_PROPERTIES, property_name),
        "event" => property_key_from(EVENT_ANIMATABLE_PROPERTIES, property_name),
        "solo" => property_key_from(SOLO_ANIMATABLE_PROPERTIES, property_name),
        _ if is_parametric_type(type_name) => {
            property_key_from(PARAMETRIC_ANIMATABLE_PROPERTIES, property_name)
                .or_else(|| property_key_from(TRANSFORM_ANIMATABLE_PROPERTIES, property_name))
        }
        _ => property_key_from_name(property_name),
    }
}
pub(crate) fn object_type_name_for_key(object_type_key: u16) -> &'static str {
    match object_type_key {
        type_keys::TEXT => "text",
        type_keys::LAYOUT_COMPONENT => "layout_component",
        type_keys::TEXT_STYLE => "text_style",
        type_keys::TEXT_MODIFIER_GROUP => "text_modifier_group",
        type_keys::TEXT_VALUE_RUN => "text_value_run",
        type_keys::CLIPPING_SHAPE => "clipping_shape",
        type_keys::FILL => "fill",
        type_keys::STROKE => "stroke",
        type_keys::SOLID_COLOR => "solid_color",
        type_keys::LINEAR_GRADIENT => "linear_gradient",
        type_keys::RADIAL_GRADIENT => "radial_gradient",
        type_keys::GRADIENT_STOP => "gradient_stop",
        type_keys::TRIM_PATH => "trim_path",
        type_keys::EVENT => "event",
        type_keys::SOLO => "solo",
        type_keys::ELLIPSE => "ellipse",
        type_keys::RECTANGLE => "rectangle",
        type_keys::TRIANGLE => "triangle",
        type_keys::POLYGON => "polygon",
        type_keys::STAR => "star",
        type_keys::SHAPE => "shape",
        type_keys::NODE => "node",
        type_keys::IMAGE => "image",
        _ => "object",
    }
}

pub(crate) fn property_key_for_object(name: &str, object_type_key: u16) -> Option<u16> {
    animatable_property_key_for_object_type(object_type_name_for_key(object_type_key), name)
}

pub(crate) fn invalid_animatable_property_error(
    object_name: &str,
    object_type_name: &str,
    property_name: &str,
) -> Option<String> {
    if matches!(property_name, "width" | "height")
        && !is_parametric_type(object_type_name)
        && object_type_name != "text"
        && object_type_name != "layout_component"
    {
        let transform_property = if property_name == "width" {
            "scale_x"
        } else {
            "scale_y"
        };
        return Some(format!(
            "keyframe targets '{}' ({}) with property '{}', which a {} does not have; '{}' animates the parametric geometry - target the child ellipse/rectangle instead, or use {}",
            object_name,
            object_type_name,
            property_name,
            object_type_name,
            property_name,
            transform_property
        ));
    }
    None
}

pub(crate) fn generic_animatable_property_names() -> Vec<&'static str> {
    transform_property_names()
}

pub(crate) fn parse_condition_op(op: &str) -> u64 {
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

pub(crate) fn condition_op_is_valid(op: &str) -> bool {
    matches!(
        op,
        "==" | "eq" | "!=" | "ne" | ">" | "gt" | ">=" | "gte" | "<" | "lt" | "<=" | "lte"
    )
}

pub(crate) fn input_is_trigger(input_name: &str, inputs: Option<&Vec<InputSpec>>) -> bool {
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

pub(crate) fn required_u64_field(
    value: Option<u64>,
    object_type: &str,
    field_name: &str,
) -> Result<u64, String> {
    value.ok_or_else(|| format!("{} must specify {}", object_type, field_name))
}

pub(crate) fn validate_discrete_keyframe_interpolation(
    object_name: &str,
    property_name: &str,
    frame: u64,
    interpolation_name: Option<&str>,
    interpolator_name: Option<&str>,
    interpolation_type: u64,
    interpolator_id: u64,
) -> Result<(), String> {
    if interpolation_name.is_none() && interpolator_name.is_none() {
        return Ok(());
    }
    if interpolation_type != 0 || interpolator_id != u32::MAX as u64 {
        return Err(format!(
            "unsupported interpolation for object '{}' property '{}' at frame {}",
            object_name, property_name, frame
        ));
    }
    Ok(())
}
