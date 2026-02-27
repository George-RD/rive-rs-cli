pub mod binary_writer;
pub mod header;
pub mod toc;

use crate::objects::core::{PropertyValue, RiveObject, is_bool_property};
use binary_writer::BinaryWriter;

pub fn encode_object(object: &dyn RiveObject) -> Vec<u8> {
    let mut writer = BinaryWriter::new();
    writer.write_varuint(object.type_key() as u64);
    for prop in object.properties() {
        writer.write_varuint(prop.key as u64);
        match prop.value {
            PropertyValue::UInt(v) => {
                if is_bool_property(prop.key) {
                    writer.write_bool(v != 0);
                } else {
                    writer.write_varuint(v);
                }
            }
            PropertyValue::Bool(v) => writer.write_bool(v),
            PropertyValue::String(s) => writer.write_string(&s),
            PropertyValue::Float(f) => writer.write_float(f),
            PropertyValue::Color(c) => writer.write_color(c),
        }
    }
    writer.write_varuint(0);
    writer.finish()
}

use crate::objects::core::property_backing_type;

pub fn encode_riv(objects: &[&dyn RiveObject], file_id: u64) -> Vec<u8> {
    let header_bytes = header::encode_header(file_id);

    let mut toc_keys: Vec<u16> = Vec::new();
    for obj in objects {
        for prop in obj.properties() {
            if property_backing_type(prop.key).is_none() && !toc_keys.contains(&prop.key) {
                toc_keys.push(prop.key);
            }
        }
    }

    let toc_bytes = toc::encode_toc(&toc_keys);

    let mut result = Vec::new();
    result.extend_from_slice(&header_bytes);
    result.extend_from_slice(&toc_bytes);

    for obj in objects {
        result.extend_from_slice(&encode_object(*obj));
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::objects::artboard::{Artboard, Backboard};

    #[test]
    fn test_encode_empty_scene() {
        let result = encode_riv(&[], 0);
        let mut expected = Vec::new();
        expected.extend_from_slice(&header::encode_header(0));
        expected.extend_from_slice(&toc::encode_toc(&[]));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_encode_backboard_only() {
        let backboard = Backboard;
        let result = encode_riv(&[&backboard], 0);
        let mut expected = Vec::new();
        expected.extend_from_slice(&header::encode_header(0));
        expected.extend_from_slice(&toc::encode_toc(&[]));
        expected.extend_from_slice(&[0x17, 0x00]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_encode_minimal_artboard() {
        let backboard = Backboard;
        let artboard = Artboard::new("Test".to_string(), 500.0, 500.0);
        let result = encode_riv(&[&backboard, &artboard], 0);

        assert_eq!(&result[0..4], &[0x52, 0x49, 0x56, 0x45]);

        let header_bytes = header::encode_header(0);
        let toc_bytes = toc::encode_toc(&[]);
        let toc_start = header_bytes.len();
        let toc_end = toc_start + toc_bytes.len();
        assert_eq!(&result[toc_start..toc_end], &toc_bytes);

        let objects_start = toc_end;
        assert_eq!(result[objects_start], 0x17);
        assert_eq!(result[objects_start + 1], 0x00);
        assert_eq!(result[objects_start + 2], 0x01);
    }

    #[test]
    fn test_encode_object_backboard() {
        let backboard = Backboard;
        let result = encode_object(&backboard);
        assert_eq!(result, vec![0x17, 0x00]);
    }

    #[test]
    fn test_encode_bool_property_as_raw_byte() {
        use crate::objects::core::{Property, PropertyValue};

        struct BoolTestObject;
        impl RiveObject for BoolTestObject {
            fn type_key(&self) -> u16 {
                20
            }
            fn properties(&self) -> Vec<Property> {
                vec![
                    Property {
                        key: 5,
                        value: PropertyValue::UInt(0),
                    },
                    Property {
                        key: 41,
                        value: PropertyValue::UInt(1),
                    },
                ]
            }
        }

        let result = encode_object(&BoolTestObject);
        assert_eq!(result[0], 20);
        assert_eq!(result[1], 5);
        assert_eq!(result[2], 0);
        assert_eq!(result[3], 41);
        assert_eq!(result[4], 1);
        assert_eq!(result[5], 0);
    }

    #[test]
    fn test_toc_only_includes_unknown_properties() {
        let artboard = Artboard::new("Test".to_string(), 500.0, 500.0);
        let data = encode_riv(&[&Backboard, &artboard], 0);
        let parsed =
            crate::validator::parse_riv(&data, &crate::validator::InspectFilter::default())
                .unwrap();
        for &key in &parsed.toc_property_keys {
            assert!(
                crate::objects::core::property_backing_type(key).is_none(),
                "known property key {} should not be in ToC",
                key
            );
        }
    }

    #[test]
    fn test_encode_object_artboard() {
        let artboard = Artboard::new("Test".to_string(), 500.0, 500.0);
        let result = encode_object(&artboard);

        assert_eq!(result[0], 0x01);
        assert_eq!(*result.last().unwrap(), 0x00);

        let mut writer = BinaryWriter::new();
        writer.write_varuint(1);
        writer.write_varuint(7);
        writer.write_float(500.0);
        writer.write_varuint(8);
        writer.write_float(500.0);
        writer.write_varuint(4);
        writer.write_string("Test");
        writer.write_varuint(0);
        assert_eq!(result, writer.finish());
    }
}
