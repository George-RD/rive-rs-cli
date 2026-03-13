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

pub(crate) fn property_key_from_name(name: &str) -> Option<u16> {
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
        "trigger" => Some(property_keys::EVENT_TRIGGER),
        "active_component_id" => Some(property_keys::SOLO_ACTIVE_COMPONENT_ID),
        _ => None,
    }
}

pub(crate) fn property_key_for_object(name: &str, object_type_key: u16) -> Option<u16> {
    if object_type_key == type_keys::TEXT {
        return match name {
            "width" => Some(property_keys::TEXT_WIDTH),
            "height" => Some(property_keys::TEXT_HEIGHT),
            "origin_x" => Some(property_keys::TEXT_ORIGIN_X),
            "origin_y" => Some(property_keys::TEXT_ORIGIN_Y),
            "paragraph_spacing" => Some(property_keys::TEXT_PARAGRAPH_SPACING),
            _ => property_key_from_name(name),
        };
    }

    if object_type_key == type_keys::LAYOUT_COMPONENT {
        return match name {
            "width" => Some(property_keys::LAYOUT_COMPONENT_WIDTH),
            "height" => Some(property_keys::LAYOUT_COMPONENT_HEIGHT),
            _ => property_key_from_name(name),
        };
    }

    if object_type_key == type_keys::TEXT_STYLE {
        return match name {
            "font_size" => Some(property_keys::TEXT_STYLE_FONT_SIZE),
            "line_height" => Some(property_keys::TEXT_STYLE_LINE_HEIGHT),
            "letter_spacing" => Some(property_keys::TEXT_STYLE_LETTER_SPACING),
            _ => None,
        };
    }

    if object_type_key == type_keys::TEXT_MODIFIER_GROUP {
        return match name {
            "modifier_flags" => Some(property_keys::TEXT_MODIFIER_GROUP_MODIFIER_FLAGS),
            "origin_x" => Some(property_keys::TEXT_MODIFIER_GROUP_ORIGIN_X),
            "origin_y" => Some(property_keys::TEXT_MODIFIER_GROUP_ORIGIN_Y),
            "opacity" => Some(property_keys::TEXT_MODIFIER_GROUP_OPACITY),
            "x" => Some(property_keys::TEXT_MODIFIER_GROUP_X),
            "y" => Some(property_keys::TEXT_MODIFIER_GROUP_Y),
            "rotation" => Some(property_keys::TEXT_MODIFIER_GROUP_ROTATION),
            "scale_x" => Some(property_keys::TEXT_MODIFIER_GROUP_SCALE_X),
            "scale_y" => Some(property_keys::TEXT_MODIFIER_GROUP_SCALE_Y),
            _ => None,
        };
    }

    match name {
        "text" if object_type_key == type_keys::TEXT_VALUE_RUN => {
            Some(property_keys::TEXT_VALUE_RUN_TEXT)
        }
        "is_visible" if object_type_key == type_keys::CLIPPING_SHAPE => {
            Some(property_keys::CLIPPING_SHAPE_IS_VISIBLE)
        }
        "is_visible" if matches!(object_type_key, type_keys::FILL | type_keys::STROKE) => {
            Some(property_keys::SHAPE_PAINT_IS_VISIBLE)
        }
        "text" | "is_visible" => None,
        _ => property_key_from_name(name),
    }
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
