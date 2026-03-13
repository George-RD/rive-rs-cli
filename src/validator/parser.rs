use std::collections::HashMap;

use serde::Serialize;

use crate::objects::core::{
    BackingType, is_bool_property, property_backing_type,
};
use crate::objects::generated_registry;

use super::binary_reader::BinaryReader;
use super::inspect::InspectFilter;

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub value: PropertyValueRead,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct RivObject {
    pub object_index: usize,
    pub type_key: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artboard_index: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artboard_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_index: Option<usize>,
    pub properties: Vec<RivProperty>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ParsedRiv {
    pub header: RivHeader,
    pub toc_property_keys: Vec<u16>,
    #[serde(skip)]
    #[allow(dead_code)] // available for callers inspecting ToC details
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
                .or_else(|| generated_registry::property_backing_type_generated(prop_key))
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

            let property_name = generated_registry::property_name(prop_key);
            properties.push(RivProperty {
                key: prop_key,
                name: property_name.map(|s| s.to_string()),
                value,
            });
        }

        let object_type_name = generated_registry::type_name(type_key);
        objects.push(RivObject {
            object_index: objects.len(),
            type_key,
            type_name: object_type_name.map(|s| s.to_string()),
            artboard_index: None,
            artboard_name: None,
            local_index: None,
            properties,
        });
    }

    super::inspect::annotate_object_context(&mut objects);

    let parsed = ParsedRiv {
        header,
        toc_property_keys,
        toc_backing_types,
        objects,
    };

    Ok(super::inspect::apply_inspect_filter(parsed, filter))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::encoder::encode_riv;
    use crate::objects::artboard::{Artboard, Backboard};
    use crate::objects::core::{property_keys, type_keys};
    use crate::objects::shapes::{Ellipse, Fill, Shape, SolidColor};

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
        assert_eq!(parsed.objects[0].object_index, 0);
        assert_eq!(parsed.objects[0].type_key, 23);
        assert!(parsed.objects[0].properties.is_empty());
        assert_eq!(parsed.objects[0].artboard_index, None);
        assert_eq!(parsed.objects[0].local_index, None);
        assert_eq!(parsed.objects[1].object_index, 1);
        assert_eq!(parsed.objects[1].type_key, 1);
        assert_eq!(parsed.objects[1].artboard_index, Some(0));
        assert_eq!(parsed.objects[1].artboard_name.as_deref(), Some("Test"));
        assert_eq!(parsed.objects[1].local_index, Some(0));
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
            !fill_props
                .iter()
                .any(|p| p.key == property_keys::SHAPE_PAINT_IS_VISIBLE)
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
    fn test_apply_inspect_filter_artboard_and_local_indices() {
        let backboard = Backboard;
        let first_artboard = Artboard::new("Screen A".to_string(), 400.0, 300.0);
        let first_shape = Shape::new("shape_a".to_string(), 0);
        let second_artboard = Artboard::new("Screen B".to_string(), 800.0, 600.0);
        let second_shape = Shape::new("shape_b".to_string(), 0);
        let data = encode_riv(
            &[
                &backboard,
                &first_artboard,
                &first_shape,
                &second_artboard,
                &second_shape,
            ],
            0,
        );
        let filter = InspectFilter {
            artboard_names: vec!["screen b".to_string()],
            local_indices: vec![1],
            ..InspectFilter::default()
        };
        let parsed = parse_riv(&data, &filter).unwrap();

        assert_eq!(parsed.objects.len(), 1);
        assert_eq!(parsed.objects[0].object_index, 4);
        assert_eq!(parsed.objects[0].artboard_index, Some(1));
        assert_eq!(parsed.objects[0].artboard_name.as_deref(), Some("Screen B"));
        assert_eq!(parsed.objects[0].local_index, Some(1));
        assert_eq!(parsed.objects[0].type_key, type_keys::SHAPE);
    }

    #[test]
    fn test_apply_inspect_filter_artboard_name_unicode_case_insensitive() {
        let backboard = Backboard;
        let artboard = Artboard::new("Écran".to_string(), 400.0, 300.0);
        let shape = Shape::new("shape".to_string(), 0);
        let data = encode_riv(&[&backboard, &artboard, &shape], 0);
        let filter = InspectFilter {
            artboard_names: vec!["éCRAN".to_string()],
            ..InspectFilter::default()
        };
        let parsed = parse_riv(&data, &filter).unwrap();

        assert_eq!(parsed.objects.len(), 2);
        assert!(
            parsed
                .objects
                .iter()
                .all(|object| object.artboard_name.as_deref() == Some("Écran"))
        );
    }
}
