use super::binary_writer::BinaryWriter;
use crate::objects::core::property_backing_type;

pub fn encode_toc(property_keys: &[u16]) -> Vec<u8> {
    let mut writer = BinaryWriter::new();

    for &key in property_keys {
        writer.write_varuint(key as u64);
    }
    writer.write_varuint(0);

    if !property_keys.is_empty() {
        let num_uint32s = (property_keys.len() + 15) / 16;
        for chunk in 0..num_uint32s {
            let mut val: u32 = 0;
            for i in 0..16 {
                let idx = chunk * 16 + i;
                if idx < property_keys.len() {
                    let backing = property_backing_type(property_keys[idx])
                        .unwrap_or_else(|| panic!("unknown property key: {}", property_keys[idx]));
                    val |= (backing as u32) << (i * 2);
                }
            }
            writer.write_bytes(&val.to_le_bytes());
        }
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
}
