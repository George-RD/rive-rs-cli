use super::binary_writer::BinaryWriter;

pub fn encode_header(file_id: u64) -> Vec<u8> {
    let mut writer = BinaryWriter::new();
    writer.write_bytes(&[0x52, 0x49, 0x56, 0x45]);
    writer.write_varuint(7);
    writer.write_varuint(0);
    writer.write_varuint(file_id);
    writer.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_header_file_id_zero() {
        let result = encode_header(0);
        assert_eq!(result, vec![0x52, 0x49, 0x56, 0x45, 0x07, 0x00, 0x00]);
    }

    #[test]
    fn test_encode_header_file_id_one() {
        let result = encode_header(1);
        assert_eq!(result, vec![0x52, 0x49, 0x56, 0x45, 0x07, 0x00, 0x01]);
    }

    #[test]
    fn test_encode_header_file_id_300() {
        let result = encode_header(300);
        assert_eq!(result, vec![0x52, 0x49, 0x56, 0x45, 0x07, 0x00, 0xAC, 0x02]);
    }
}
