use super::binary_writer::BinaryWriter;
use crate::objects::core::{BackingType, property_backing_type};

fn backing_bits(backing_type: BackingType) -> u32 {
    match backing_type {
        BackingType::UInt => 0,
        BackingType::String => 1,
        BackingType::Float => 2,
        BackingType::Color => 3,
    }
}

pub(crate) const TOC_CODES_PER_WORD: usize = 4;

pub(crate) fn encode_toc(property_keys: &[u16]) -> Vec<u8> {
    let mut writer = BinaryWriter::new();

    for &key in property_keys {
        writer.write_varuint(key as u64);
    }
    writer.write_varuint(0);

    for chunk in property_keys.chunks(TOC_CODES_PER_WORD) {
        let mut val: u32 = 0;
        for (i, &key) in chunk.iter().enumerate() {
            let backing = property_backing_type(key).unwrap_or_else(|| {
                panic!(
                    "property key {} has no known backing type — register it in core::property_backing_type()",
                    key
                )
            });
            val |= backing_bits(backing) << (i * 2);
        }
        writer.write_bytes(&val.to_le_bytes());
    }

    writer.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_keys() {
        assert_eq!(encode_toc(&[]), vec![0x00]);
    }

    #[test]
    fn test_single_string_property() {
        let result = encode_toc(&[4]);
        assert_eq!(result, vec![0x04, 0x00, 0x01, 0x00, 0x00, 0x00]);
    }

    #[test]
    fn test_data_bind_context_source_path_has_string_backing() {
        let key = crate::objects::core::property_keys::DATA_BIND_CONTEXT_SOURCE_PATH_IDS;
        let result = encode_toc(&[key]);
        let key_len = if key < 128 { 1 } else { 2 };
        let backing_offset = key_len + 1;
        assert_eq!(result[backing_offset] & 0x03, 0b01);
        assert!(crate::objects::core::is_bytes_property(key));
    }

    #[test]
    fn test_two_properties_color_and_float() {
        let result = encode_toc(&[37, 7]);
        assert_eq!(result, vec![0x25, 0x07, 0x00, 0x0B, 0x00, 0x00, 0x00]);
    }

    #[test]
    fn test_four_properties_fill_one_uint32() {
        let result = encode_toc(&[4, 7, 37, 5]);
        let toc_keys_len = 5;
        assert_eq!(result.len(), toc_keys_len + 4);
        let uint32_bytes = &result[toc_keys_len..];
        let val = u32::from_le_bytes([
            uint32_bytes[0],
            uint32_bytes[1],
            uint32_bytes[2],
            uint32_bytes[3],
        ]);
        assert_eq!(val & 0xFF, 0b00_11_10_01);
    }

    #[test]
    fn test_five_properties_span_two_uint32_words() {
        let keys = [4u16, 7, 37, 5, 8];
        let result = encode_toc(&keys);
        let keys_len = keys.len() + 1;
        assert_eq!(
            result.len(),
            keys_len + 8,
            "five keys must emit two uint32 words: the runtime reloads a word every {} codes",
            TOC_CODES_PER_WORD
        );
        let second = u32::from_le_bytes([
            result[keys_len + 4],
            result[keys_len + 5],
            result[keys_len + 6],
            result[keys_len + 7],
        ]);
        assert_eq!(second & 0x03, 0b10, "key 8 is float and starts a new word");
    }

    #[test]
    fn test_eight_properties_emit_two_words() {
        let keys = [4u16, 7, 37, 5, 8, 13, 14, 18];
        let result = encode_toc(&keys);
        assert_eq!(result.len(), keys.len() + 1 + 8);
    }
}
