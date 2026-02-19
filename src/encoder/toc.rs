use super::binary_writer::BinaryWriter;
use crate::objects::core::property_backing_type;

pub fn encode_toc(property_keys: &[u16]) -> Vec<u8> {
    let mut writer = BinaryWriter::new();

    for &key in property_keys {
        writer.write_varuint(key as u64);
    }
    writer.write_varuint(0);

    if !property_keys.is_empty() {
        let num_bytes = (property_keys.len() + 3) / 4;
        let mut bit_array = vec![0u8; num_bytes];
        for (i, &key) in property_keys.iter().enumerate() {
            let backing = property_backing_type(key)
                .unwrap_or_else(|| panic!("unknown property key: {}", key));
            let bits = backing as u8;
            let byte_index = i / 4;
            let bit_offset = (i % 4) * 2;
            bit_array[byte_index] |= bits << bit_offset;
        }
        writer.write_bytes(&bit_array);
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
        assert_eq!(result, vec![0x04, 0x00, 0x01]);
    }

    #[test]
    fn test_two_properties_color_and_float() {
        let result = encode_toc(&[37, 7]);
        assert_eq!(result, vec![0x25, 0x07, 0x00, 0x0B]);
    }

    #[test]
    fn test_four_properties_fill_one_byte() {
        let result = encode_toc(&[4, 7, 37, 5]);
        assert_eq!(result.last().copied(), Some(0b00_11_10_01u8));
        assert_eq!(result.len(), 6);
    }
}
