use std::collections::HashMap;

use serde::Serialize;

use crate::objects::core::{
    BackingType, is_bool_property, property_backing_type, property_keys, type_keys,
};

#[derive(Debug, Clone, Default)]
pub struct InspectFilter {
    pub type_keys: Vec<u16>,
    pub type_names: Vec<String>,
    pub object_indices: Vec<usize>,
    pub property_keys: Vec<u16>,
}

impl InspectFilter {
    fn is_active(&self) -> bool {
        !self.type_keys.is_empty()
            || !self.type_names.is_empty()
            || !self.object_indices.is_empty()
            || !self.property_keys.is_empty()
    }
}

pub struct BinaryReader<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> BinaryReader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        BinaryReader { data, pos: 0 }
    }

    pub fn read_byte(&mut self) -> Option<u8> {
        if self.pos < self.data.len() {
            let b = self.data[self.pos];
            self.pos += 1;
            Some(b)
        } else {
            None
        }
    }

    pub fn read_varuint(&mut self) -> Option<u64> {
        let mut result: u64 = 0;
        let mut shift: u32 = 0;
        loop {
            let byte = self.read_byte()?;
            result |= ((byte & 0x7F) as u64) << shift;
            if byte & 0x80 == 0 {
                return Some(result);
            }
            shift += 7;
            if shift >= 64 {
                return None;
            }
        }
    }

    pub fn read_float(&mut self) -> Option<f32> {
        let bytes = self.read_bytes(4)?;
        Some(f32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }

    pub fn read_string(&mut self) -> Option<String> {
        let len = self.read_varuint()? as usize;
        let bytes = self.read_bytes(len)?;
        String::from_utf8(bytes.to_vec()).ok()
    }

    pub fn read_color(&mut self) -> Option<u32> {
        let bytes = self.read_bytes(4)?;
        Some(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }

    pub fn read_bytes(&mut self, n: usize) -> Option<&'a [u8]> {
        let end = self.pos.checked_add(n)?;
        if end <= self.data.len() {
            let slice = &self.data[self.pos..end];
            self.pos = end;
            Some(slice)
        } else {
            None
        }
    }

    pub fn remaining(&self) -> usize {
        self.data.len() - self.pos
    }

    pub fn position(&self) -> usize {
        self.pos
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct RivHeader {
    pub major_version: u64,
    pub minor_version: u64,
    pub file_id: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum PropertyValueRead {
    UInt(u64),
    String(String),
    Float(f32),
    Color(u32),
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct RivProperty {
    pub key: u16,
    pub value: PropertyValueRead,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct RivObject {
    pub type_key: u16,
    pub properties: Vec<RivProperty>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ParsedRiv {
    pub header: RivHeader,
    pub toc_property_keys: Vec<u16>,
    #[serde(skip)]
    pub toc_backing_types: Vec<BackingType>,
    pub objects: Vec<RivObject>,
}

pub fn parse_riv(data: &[u8], filter: &InspectFilter) -> Result<ParsedRiv, String> {
    let mut reader = BinaryReader::new(data);

    let fingerprint = reader
        .read_bytes(4)
        .ok_or_else(|| "unexpected end of data reading fingerprint".to_string())?;
    if fingerprint != [0x52, 0x49, 0x56, 0x45] {
        return Err(format!(
            "invalid fingerprint: {:?}, expected RIVE",
            fingerprint
        ));
    }

    let major_version = reader
        .read_varuint()
        .ok_or_else(|| "unexpected end of data reading major version".to_string())?;
    let minor_version = reader
        .read_varuint()
        .ok_or_else(|| "unexpected end of data reading minor version".to_string())?;
    let file_id = reader
        .read_varuint()
        .ok_or_else(|| "unexpected end of data reading file ID".to_string())?;

    let header = RivHeader {
        major_version,
        minor_version,
        file_id,
    };

    let mut toc_property_keys: Vec<u16> = Vec::new();
    loop {
        let key = reader
            .read_varuint()
            .ok_or_else(|| "unexpected end of data reading ToC property key".to_string())?;
        if key == 0 {
            break;
        }
        toc_property_keys.push(key as u16);
    }

    let mut toc_backing_types: Vec<BackingType> = Vec::new();
    let mut toc_map: HashMap<u16, BackingType> = HashMap::new();

    if !toc_property_keys.is_empty() {
        let mut current_u32: u32 = 0;
        let mut bit_pos: usize = 32;

        for (i, &key) in toc_property_keys.iter().enumerate() {
            if i % 16 == 0 {
                let bytes = reader.read_bytes(4).ok_or_else(|| {
                    "unexpected end of data reading ToC backing type uint32".to_string()
                })?;
                current_u32 = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
                bit_pos = 0;
            }
            let bits = (current_u32 >> bit_pos) & 0x03;
            let backing = match bits {
                0 => BackingType::UInt,
                1 => BackingType::String,
                2 => BackingType::Float,
                3 => BackingType::Color,
                _ => unreachable!(),
            };
            toc_backing_types.push(backing);
            toc_map.insert(key, backing);
            bit_pos += 2;
        }
    }

    let mut objects: Vec<RivObject> = Vec::new();
    while reader.remaining() > 0 {
        let type_key = reader
            .read_varuint()
            .ok_or_else(|| "unexpected end of data reading object type key".to_string())?
            as u16;

        let mut properties: Vec<RivProperty> = Vec::new();
        loop {
            let prop_key = reader
                .read_varuint()
                .ok_or_else(|| "unexpected end of data reading property key".to_string())?;
            if prop_key == 0 {
                break;
            }
            let prop_key = prop_key as u16;

            let backing = toc_map
                .get(&prop_key)
                .copied()
                .or_else(|| property_backing_type(prop_key))
                .ok_or_else(|| {
                    format!(
                        "unknown backing type for property key {} in object type {}",
                        prop_key, type_key
                    )
                })?;

            let value = match backing {
                BackingType::UInt => {
                    if is_bool_property(prop_key) {
                        let v = reader.read_byte().ok_or_else(|| {
                            format!("unexpected end of data reading bool property {}", prop_key)
                        })?;
                        PropertyValueRead::UInt(v as u64)
                    } else {
                        let v = reader.read_varuint().ok_or_else(|| {
                            format!("unexpected end of data reading uint property {}", prop_key)
                        })?;
                        PropertyValueRead::UInt(v)
                    }
                }
                BackingType::String => {
                    let v = reader.read_string().ok_or_else(|| {
                        format!(
                            "unexpected end of data reading string property {}",
                            prop_key
                        )
                    })?;
                    PropertyValueRead::String(v)
                }
                BackingType::Float => {
                    let v = reader.read_float().ok_or_else(|| {
                        format!("unexpected end of data reading float property {}", prop_key)
                    })?;
                    PropertyValueRead::Float(v)
                }
                BackingType::Color => {
                    let v = reader.read_color().ok_or_else(|| {
                        format!("unexpected end of data reading color property {}", prop_key)
                    })?;
                    PropertyValueRead::Color(v)
                }
            };

            properties.push(RivProperty {
                key: prop_key,
                value,
            });
        }

        objects.push(RivObject {
            type_key,
            properties,
        });
    }

    let parsed = ParsedRiv {
        header,
        toc_property_keys,
        toc_backing_types,
        objects,
    };

    Ok(apply_inspect_filter(parsed, filter))
}

#[derive(Debug, Clone, Serialize)]
pub struct ValidationReport {
    pub header: RivHeader,
    pub object_count: usize,
    pub type_counts: HashMap<u16, usize>,
    pub errors: Vec<String>,
    pub valid: bool,
}

pub fn validate_riv(data: &[u8]) -> Result<ValidationReport, String> {
    let parsed = parse_riv(data, &InspectFilter::default())?;

    let mut type_counts: HashMap<u16, usize> = HashMap::new();
    for obj in &parsed.objects {
        *type_counts.entry(obj.type_key).or_insert(0) += 1;
    }

    let mut errors: Vec<String> = Vec::new();

    if !type_counts.contains_key(&23) {
        errors.push("missing Backboard (type 23)".to_string());
    }

    if !type_counts.contains_key(&1) {
        errors.push("missing Artboard (type 1)".to_string());
    }

    if !parsed.objects.is_empty() && parsed.objects[0].type_key != 23 {
        errors.push(format!(
            "first object should be Backboard (type 23), got type {}",
            parsed.objects[0].type_key
        ));
    }

    let valid = errors.is_empty();

    Ok(ValidationReport {
        header: parsed.header,
        object_count: parsed.objects.len(),
        type_counts,
        errors,
        valid,
    })
}

fn type_name(key: u16) -> &'static str {
    match key {
        type_keys::ARTBOARD => "Artboard",
        type_keys::NODE => "Node",
        type_keys::SHAPE => "Shape",
        type_keys::ELLIPSE => "Ellipse",
        type_keys::RECTANGLE => "Rectangle",
        type_keys::COMPONENT => "Component",
        type_keys::CONTAINER_COMPONENT => "ContainerComponent",
        type_keys::PATH => "Path",
        type_keys::DRAWABLE => "Drawable",
        type_keys::PARAMETRIC_PATH => "ParametricPath",
        type_keys::RADIAL_GRADIENT => "RadialGradient",
        type_keys::SOLID_COLOR => "SolidColor",
        type_keys::GRADIENT_STOP => "GradientStop",
        type_keys::FILL => "Fill",
        type_keys::SHAPE_PAINT => "ShapePaint",
        type_keys::LINEAR_GRADIENT => "LinearGradient",
        type_keys::BACKBOARD => "Backboard",
        type_keys::STROKE => "Stroke",
        type_keys::KEYED_OBJECT => "KeyedObject",
        type_keys::KEYED_PROPERTY => "KeyedProperty",
        type_keys::ANIMATION => "Animation",
        type_keys::CUBIC_EASE_INTERPOLATOR => "CubicEaseInterpolator",
        type_keys::KEY_FRAME => "KeyFrame",
        type_keys::KEY_FRAME_DOUBLE => "KeyFrameDouble",
        type_keys::LINEAR_ANIMATION => "LinearAnimation",
        type_keys::KEY_FRAME_COLOR => "KeyFrameColor",
        type_keys::TRANSFORM_COMPONENT => "TransformComponent",
        type_keys::TRIM_PATH => "TrimPath",
        type_keys::STATE_MACHINE => "StateMachine",
        type_keys::STATE_MACHINE_COMPONENT => "StateMachineComponent",
        type_keys::STATE_MACHINE_INPUT => "StateMachineInput",
        type_keys::STATE_MACHINE_NUMBER => "StateMachineNumber",
        type_keys::STATE_MACHINE_LAYER => "StateMachineLayer",
        type_keys::STATE_MACHINE_TRIGGER => "StateMachineTrigger",
        type_keys::STATE_MACHINE_BOOL => "StateMachineBool",
        type_keys::LAYER_STATE => "LayerState",
        type_keys::ANIMATION_STATE => "AnimationState",
        type_keys::ANY_STATE => "AnyState",
        type_keys::ENTRY_STATE => "EntryState",
        type_keys::EXIT_STATE => "ExitState",
        type_keys::STATE_TRANSITION => "StateTransition",
        type_keys::TRANSITION_INPUT_CONDITION => "TransitionInputCondition",
        type_keys::TRANSITION_TRIGGER_CONDITION => "TransitionTriggerCondition",
        type_keys::TRANSITION_VALUE_CONDITION => "TransitionValueCondition",
        type_keys::TRANSITION_NUMBER_CONDITION => "TransitionNumberCondition",
        type_keys::TRANSITION_BOOL_CONDITION => "TransitionBoolCondition",
        type_keys::WORLD_TRANSFORM_COMPONENT => "WorldTransformComponent",
        type_keys::NESTED_ARTBOARD => "NestedArtboard",
        type_keys::CUBIC_VALUE_INTERPOLATOR => "CubicValueInterpolator",
        type_keys::CUBIC_INTERPOLATOR => "CubicInterpolator",
        type_keys::INTERPOLATING_KEY_FRAME => "InterpolatingKeyFrame",
        type_keys::KEYFRAME_INTERPOLATOR => "KeyFrameInterpolator",
        type_keys::LAYOUT_COMPONENT => "LayoutComponent",
        type_keys::TRANSITION_CONDITION => "TransitionCondition",
        _ => "Unknown",
    }
}

fn matches_object_filter(filter: &InspectFilter, index: usize, object: &RivObject) -> bool {
    let type_key_match = filter.type_keys.is_empty() || filter.type_keys.contains(&object.type_key);
    let type_name_match = filter.type_names.is_empty()
        || filter
            .type_names
            .iter()
            .any(|name| name.eq_ignore_ascii_case(type_name(object.type_key)));
    let index_match = filter.object_indices.is_empty() || filter.object_indices.contains(&index);

    type_key_match && type_name_match && index_match
}

fn apply_inspect_filter(mut parsed: ParsedRiv, filter: &InspectFilter) -> ParsedRiv {
    let mut filtered_objects = Vec::new();

    for (index, mut object) in parsed.objects.into_iter().enumerate() {
        if !matches_object_filter(filter, index, &object) {
            continue;
        }

        if !filter.property_keys.is_empty() {
            object
                .properties
                .retain(|property| filter.property_keys.contains(&property.key));
        }

        filtered_objects.push(object);
    }

    parsed.objects = filtered_objects;
    parsed
}

pub fn inspect_riv(data: &[u8], filter: &InspectFilter) -> Result<String, String> {
    let parsed = parse_riv(data, filter)?;
    let mut out = std::string::String::new();

    let artboard_count = parsed
        .objects
        .iter()
        .filter(|o| o.type_key == type_keys::ARTBOARD)
        .count();

    out.push_str(&format!(
        "RIVE v{}.{} file_id={}\n",
        parsed.header.major_version, parsed.header.minor_version, parsed.header.file_id
    ));
    out.push_str(&format!(
        "ToC: {} properties\n",
        parsed.toc_property_keys.len()
    ));
    out.push_str(&format!("Objects: {}\n", parsed.objects.len()));
    if parsed.objects.is_empty() && filter.is_active() {
        out.push_str("No objects matched the provided filters.\n");
        return Ok(out);
    }
    if artboard_count > 1 {
        out.push_str(&format!("Artboards: {}\n", artboard_count));
    }

    let mut artboard_idx = 0;
    let mut local_idx: usize = 0;
    for (i, obj) in parsed.objects.iter().enumerate() {
        if obj.type_key == type_keys::ARTBOARD {
            if artboard_count > 1 {
                let name = obj
                    .properties
                    .iter()
                    .find(|p| p.key == property_keys::COMPONENT_NAME)
                    .and_then(|p| match &p.value {
                        PropertyValueRead::String(s) => Some(s.as_str()),
                        _ => None,
                    })
                    .unwrap_or("unnamed");
                out.push_str(&format!("--- Artboard {} ({}) ---\n", artboard_idx, name));
            }
            artboard_idx += 1;
            local_idx = 0;
        }
        if artboard_count > 1 {
            out.push_str(&format!(
                "[{}:{}] type={} ({})\n",
                i,
                local_idx,
                obj.type_key,
                type_name(obj.type_key)
            ));
        } else {
            out.push_str(&format!(
                "[{}] type={} ({})\n",
                i,
                obj.type_key,
                type_name(obj.type_key)
            ));
        }
        local_idx += 1;
        for prop in &obj.properties {
            let val_str = match &prop.value {
                PropertyValueRead::UInt(v) => format!("uint({})", v),
                PropertyValueRead::String(v) => format!("string({:?})", v),
                PropertyValueRead::Float(v) => format!("float({})", v),
                PropertyValueRead::Color(v) => format!("color(0x{:08X})", v),
            };
            out.push_str(&format!("  key={} {}\n", prop.key, val_str));
        }
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::encoder::encode_riv;
    use crate::objects::artboard::{Artboard, Backboard};
    use crate::objects::core::{property_keys, type_keys};
    use crate::objects::shapes::{Ellipse, Fill, Shape, SolidColor};

    #[test]
    fn test_binary_reader_varuint() {
        let mut reader = BinaryReader::new(&[0x00]);
        assert_eq!(reader.read_varuint(), Some(0));

        let mut reader = BinaryReader::new(&[0x7F]);
        assert_eq!(reader.read_varuint(), Some(127));

        let mut reader = BinaryReader::new(&[0x80, 0x01]);
        assert_eq!(reader.read_varuint(), Some(128));

        let mut reader = BinaryReader::new(&[0xAC, 0x02]);
        assert_eq!(reader.read_varuint(), Some(300));

        let mut reader = BinaryReader::new(&[0x80, 0x80, 0x01]);
        assert_eq!(reader.read_varuint(), Some(16384));
    }

    #[test]
    fn test_binary_reader_varuint_max() {
        let mut reader =
            BinaryReader::new(&[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01]);
        assert_eq!(reader.read_varuint(), Some(u64::MAX));
        assert_eq!(reader.remaining(), 0);
    }

    #[test]
    fn test_binary_reader_varuint_overflow() {
        let mut reader = BinaryReader::new(&[
            0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x00,
        ]);
        assert_eq!(reader.read_varuint(), None);
    }

    #[test]
    fn test_binary_reader_truncated() {
        let mut reader = BinaryReader::new(&[]);
        assert_eq!(reader.read_varuint(), None);

        let mut reader = BinaryReader::new(&[0x80]);
        assert_eq!(reader.read_varuint(), None);
    }

    #[test]
    fn test_binary_reader_float() {
        let mut reader = BinaryReader::new(&[0x00, 0x00, 0x80, 0x3F]);
        assert_eq!(reader.read_float(), Some(1.0));

        let mut reader = BinaryReader::new(&[0x00, 0x00, 0x00, 0x00]);
        assert_eq!(reader.read_float(), Some(0.0));
    }

    #[test]
    fn test_binary_reader_string() {
        let mut reader = BinaryReader::new(&[0x05, 0x68, 0x65, 0x6C, 0x6C, 0x6F]);
        assert_eq!(reader.read_string(), Some("hello".to_string()));

        let mut reader = BinaryReader::new(&[0x00]);
        assert_eq!(reader.read_string(), Some("".to_string()));
    }

    #[test]
    fn test_parse_riv_empty() {
        let data = encode_riv(&[], 0);
        let parsed = parse_riv(&data, &InspectFilter::default()).unwrap();
        assert_eq!(parsed.header.major_version, 7);
        assert_eq!(parsed.header.minor_version, 0);
        assert_eq!(parsed.header.file_id, 0);
        assert!(parsed.toc_property_keys.is_empty());
        assert!(parsed.toc_backing_types.is_empty());
        assert!(parsed.objects.is_empty());
    }

    #[test]
    fn test_parse_riv_minimal() {
        let backboard = Backboard;
        let artboard = Artboard::new("Test".to_string(), 500.0, 500.0);
        let data = encode_riv(&[&backboard, &artboard], 42);
        let parsed = parse_riv(&data, &InspectFilter::default()).unwrap();

        assert_eq!(parsed.header.major_version, 7);
        assert_eq!(parsed.header.minor_version, 0);
        assert_eq!(parsed.header.file_id, 42);
        assert_eq!(parsed.objects.len(), 2);
        assert_eq!(parsed.objects[0].type_key, 23);
        assert!(parsed.objects[0].properties.is_empty());
        assert_eq!(parsed.objects[1].type_key, 1);
        assert_eq!(parsed.objects[1].properties.len(), 3);

        let width_prop = &parsed.objects[1].properties[0];
        assert_eq!(width_prop.key, 7);
        assert_eq!(width_prop.value, PropertyValueRead::Float(500.0));

        let height_prop = &parsed.objects[1].properties[1];
        assert_eq!(height_prop.key, 8);
        assert_eq!(height_prop.value, PropertyValueRead::Float(500.0));

        let name_prop = &parsed.objects[1].properties[2];
        assert_eq!(name_prop.key, 4);
        assert_eq!(
            name_prop.value,
            PropertyValueRead::String("Test".to_string())
        );
    }

    #[test]
    fn test_roundtrip_shapes() {
        let backboard = Backboard;
        let artboard = Artboard::new("Shapes".to_string(), 500.0, 500.0);
        let shape = Shape::new("shape-1".to_string(), 0);
        let ellipse = Ellipse::new("ellipse-1".to_string(), 1, 100.0, 80.0);
        let fill = Fill::new("fill-1".to_string(), 1);
        let solid = SolidColor::new("solid-1".to_string(), 3, 0xFF0000FF);

        let data = encode_riv(&[&backboard, &artboard, &shape, &ellipse, &fill, &solid], 7);
        let parsed = parse_riv(&data, &InspectFilter::default()).unwrap();

        assert_eq!(parsed.header.file_id, 7);
        assert_eq!(parsed.objects.len(), 6);
        assert_eq!(parsed.objects[0].type_key, type_keys::BACKBOARD);
        assert_eq!(parsed.objects[1].type_key, type_keys::ARTBOARD);
        assert_eq!(parsed.objects[2].type_key, type_keys::SHAPE);
        assert_eq!(parsed.objects[3].type_key, type_keys::ELLIPSE);
        assert_eq!(parsed.objects[4].type_key, type_keys::FILL);
        assert_eq!(parsed.objects[5].type_key, type_keys::SOLID_COLOR);

        let shape_props = &parsed.objects[2].properties;
        assert!(
            shape_props
                .iter()
                .any(|p| p.key == property_keys::COMPONENT_NAME
                    && p.value == PropertyValueRead::String("shape-1".to_string()))
        );
        assert!(
            shape_props
                .iter()
                .any(|p| p.key == property_keys::COMPONENT_PARENT_ID
                    && p.value == PropertyValueRead::UInt(0))
        );

        let ellipse_props = &parsed.objects[3].properties;
        assert!(
            ellipse_props
                .iter()
                .any(|p| p.key == property_keys::COMPONENT_NAME
                    && p.value == PropertyValueRead::String("ellipse-1".to_string()))
        );
        assert!(
            ellipse_props
                .iter()
                .any(|p| p.key == property_keys::COMPONENT_PARENT_ID
                    && p.value == PropertyValueRead::UInt(1))
        );
        assert!(
            ellipse_props
                .iter()
                .any(|p| p.key == property_keys::PARAMETRIC_PATH_WIDTH
                    && p.value == PropertyValueRead::Float(100.0))
        );
        assert!(
            ellipse_props
                .iter()
                .any(|p| p.key == property_keys::PARAMETRIC_PATH_HEIGHT
                    && p.value == PropertyValueRead::Float(80.0))
        );

        let fill_props = &parsed.objects[4].properties;
        assert!(
            fill_props
                .iter()
                .any(|p| p.key == property_keys::COMPONENT_NAME
                    && p.value == PropertyValueRead::String("fill-1".to_string()))
        );
        assert!(
            fill_props
                .iter()
                .any(|p| p.key == property_keys::COMPONENT_PARENT_ID
                    && p.value == PropertyValueRead::UInt(1))
        );
        assert!(
            fill_props
                .iter()
                .any(|p| p.key == property_keys::SHAPE_PAINT_IS_VISIBLE
                    && p.value == PropertyValueRead::UInt(1))
        );

        let solid_props = &parsed.objects[5].properties;
        assert!(
            solid_props
                .iter()
                .any(|p| p.key == property_keys::COMPONENT_NAME
                    && p.value == PropertyValueRead::String("solid-1".to_string()))
        );
        assert!(
            solid_props
                .iter()
                .any(|p| p.key == property_keys::COMPONENT_PARENT_ID
                    && p.value == PropertyValueRead::UInt(3))
        );
        assert!(
            solid_props
                .iter()
                .any(|p| p.key == property_keys::SOLID_COLOR_VALUE
                    && p.value == PropertyValueRead::Color(0xFF0000FF))
        );
    }

    #[test]
    fn test_validate_riv_minimal() {
        let backboard = Backboard;
        let artboard = Artboard::new("Test".to_string(), 500.0, 500.0);
        let data = encode_riv(&[&backboard, &artboard], 0);
        let report = validate_riv(&data).unwrap();

        assert!(report.valid);
        assert!(report.errors.is_empty());
        assert_eq!(report.object_count, 2);
        assert_eq!(report.type_counts.get(&23), Some(&1));
        assert_eq!(report.type_counts.get(&1), Some(&1));
    }

    #[test]
    fn test_inspect_riv_minimal() {
        let backboard = Backboard;
        let artboard = Artboard::new("Test".to_string(), 500.0, 500.0);
        let data = encode_riv(&[&backboard, &artboard], 0);
        let output = inspect_riv(&data, &InspectFilter::default()).unwrap();

        assert!(output.contains("Backboard"));
        assert!(output.contains("Artboard"));
        assert!(output.contains("RIVE v7.0"));
        assert!(output.contains("Objects: 2"));
    }
}
