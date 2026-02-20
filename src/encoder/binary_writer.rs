pub struct BinaryWriter {
    buffer: Vec<u8>,
}

impl BinaryWriter {
    pub fn new() -> Self {
        BinaryWriter { buffer: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn write_bytes(&mut self, bytes: &[u8]) {
        self.buffer.extend_from_slice(bytes);
    }

    pub fn write_varuint(&mut self, mut value: u64) {
        loop {
            let mut byte = (value & 0x7F) as u8;
            value >>= 7;
            if value != 0 {
                byte |= 0x80;
            }
            self.buffer.push(byte);
            if value == 0 {
                break;
            }
        }
    }

    pub fn write_float(&mut self, value: f32) {
        let bytes = value.to_le_bytes();
        self.buffer.extend_from_slice(&bytes);
    }

    pub fn write_string(&mut self, value: &str) {
        let bytes = value.as_bytes();
        self.write_varuint(bytes.len() as u64);
        self.buffer.extend_from_slice(bytes);
    }

    pub fn write_color(&mut self, value: u32) {
        let bytes = value.to_le_bytes();
        self.buffer.extend_from_slice(&bytes);
    }

    pub fn write_uint(&mut self, value: u64) {
        self.write_varuint(value);
    }

    pub fn write_bool(&mut self, value: bool) {
        self.buffer.push(if value { 1 } else { 0 });
    }

    pub fn finish(self) -> Vec<u8> {
        self.buffer
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::validator::BinaryReader;
    use proptest::prelude::*;

    #[test]
    fn test_write_varuint_zero() {
        let mut writer = BinaryWriter::new();
        writer.write_varuint(0);
        assert_eq!(writer.finish(), vec![0x00]);
    }

    #[test]
    fn test_write_varuint_127() {
        let mut writer = BinaryWriter::new();
        writer.write_varuint(127);
        assert_eq!(writer.finish(), vec![0x7F]);
    }

    #[test]
    fn test_write_varuint_128() {
        let mut writer = BinaryWriter::new();
        writer.write_varuint(128);
        assert_eq!(writer.finish(), vec![0x80, 0x01]);
    }

    #[test]
    fn test_write_varuint_300() {
        let mut writer = BinaryWriter::new();
        writer.write_varuint(300);
        assert_eq!(writer.finish(), vec![0xAC, 0x02]);
    }

    #[test]
    fn test_write_varuint_16384() {
        let mut writer = BinaryWriter::new();
        writer.write_varuint(16384);
        assert_eq!(writer.finish(), vec![0x80, 0x80, 0x01]);
    }

    #[test]
    fn test_write_float_one() {
        let mut writer = BinaryWriter::new();
        writer.write_float(1.0);
        assert_eq!(writer.finish(), vec![0x00, 0x00, 0x80, 0x3F]);
    }

    #[test]
    fn test_write_float_zero() {
        let mut writer = BinaryWriter::new();
        writer.write_float(0.0);
        assert_eq!(writer.finish(), vec![0x00, 0x00, 0x00, 0x00]);
    }

    #[test]
    fn test_write_string_hello() {
        let mut writer = BinaryWriter::new();
        writer.write_string("hello");
        assert_eq!(writer.finish(), vec![0x05, 0x68, 0x65, 0x6C, 0x6C, 0x6F]);
    }

    #[test]
    fn test_write_string_empty() {
        let mut writer = BinaryWriter::new();
        writer.write_string("");
        assert_eq!(writer.finish(), vec![0x00]);
    }

    #[test]
    fn test_write_color() {
        let mut writer = BinaryWriter::new();
        writer.write_color(0xFF0000FF);
        assert_eq!(writer.finish(), vec![0xFF, 0x00, 0x00, 0xFF]);
    }

    #[test]
    fn test_write_bool_true() {
        let mut writer = BinaryWriter::new();
        writer.write_bool(true);
        assert_eq!(writer.finish(), vec![0x01]);
    }

    #[test]
    fn test_write_bool_false() {
        let mut writer = BinaryWriter::new();
        writer.write_bool(false);
        assert_eq!(writer.finish(), vec![0x00]);
    }

    #[test]
    fn test_write_uint() {
        let mut writer = BinaryWriter::new();
        writer.write_uint(300);
        assert_eq!(writer.finish(), vec![0xAC, 0x02]);
    }

    #[test]
    fn test_write_bytes() {
        let mut writer = BinaryWriter::new();
        writer.write_bytes(&[0x01, 0x02, 0x03]);
        assert_eq!(writer.finish(), vec![0x01, 0x02, 0x03]);
    }

    #[test]
    fn test_len() {
        let mut writer = BinaryWriter::new();
        assert_eq!(writer.len(), 0);
        writer.write_bytes(&[0x01, 0x02]);
        assert_eq!(writer.len(), 2);
        writer.write_varuint(128);
        assert_eq!(writer.len(), 4);
    }

    #[test]
    fn test_multiple_writes() {
        let mut writer = BinaryWriter::new();
        writer.write_varuint(5);
        writer.write_float(1.0);
        writer.write_bool(true);
        let result = writer.finish();
        assert_eq!(result.len(), 6);
        assert_eq!(result[0], 0x05);
        assert_eq!(&result[1..5], &[0x00, 0x00, 0x80, 0x3F]);
        assert_eq!(result[5], 0x01);
    }

    #[test]
    fn test_write_varuint_max_u64() {
        let mut writer = BinaryWriter::new();
        writer.write_varuint(u64::MAX);
        assert_eq!(writer.finish().len(), 10);
    }

    #[test]
    fn test_write_varuint_boundary_values() {
        let mut writer = BinaryWriter::new();
        writer.write_varuint((1 << 7) - 1);
        assert_eq!(writer.finish().len(), 1);

        let mut writer = BinaryWriter::new();
        writer.write_varuint(1 << 7);
        assert_eq!(writer.finish().len(), 2);

        let mut writer = BinaryWriter::new();
        writer.write_varuint((1 << 14) - 1);
        assert_eq!(writer.finish().len(), 2);

        let mut writer = BinaryWriter::new();
        writer.write_varuint(1 << 14);
        assert_eq!(writer.finish().len(), 3);

        let mut writer = BinaryWriter::new();
        writer.write_varuint((1 << 21) - 1);
        assert_eq!(writer.finish().len(), 3);

        let mut writer = BinaryWriter::new();
        writer.write_varuint(1 << 21);
        assert_eq!(writer.finish().len(), 4);

        let mut writer = BinaryWriter::new();
        writer.write_varuint((1 << 28) - 1);
        assert_eq!(writer.finish().len(), 4);

        let mut writer = BinaryWriter::new();
        writer.write_varuint(1 << 28);
        assert_eq!(writer.finish().len(), 5);
    }

    #[test]
    fn test_varuint_roundtrip_edge_cases() {
        let edge_cases = [u64::MAX, u64::MAX - 1, 1, 2, 0xFFFF_FFFF];

        for value in edge_cases {
            let mut writer = BinaryWriter::new();
            writer.write_varuint(value);
            let encoded = writer.finish();
            let mut reader = BinaryReader::new(&encoded);
            assert_eq!(reader.read_varuint(), Some(value));
            assert_eq!(reader.remaining(), 0);
        }
    }

    proptest! {
        #[test]
        fn proptest_varuint_roundtrip(value in any::<u64>()) {
            let mut writer = BinaryWriter::new();
            writer.write_varuint(value);
            let encoded = writer.finish();
            let mut reader = BinaryReader::new(&encoded);
            prop_assert_eq!(reader.read_varuint(), Some(value));
            prop_assert_eq!(reader.remaining(), 0);
        }

        #[test]
        fn proptest_float_roundtrip(value in any::<f32>()) {
            let mut writer = BinaryWriter::new();
            writer.write_float(value);
            let encoded = writer.finish();
            let mut reader = BinaryReader::new(&encoded);
            let decoded = reader.read_float().unwrap();
            prop_assert_eq!(decoded.to_bits(), value.to_bits());
            prop_assert_eq!(reader.remaining(), 0);
        }

        #[test]
        fn proptest_string_roundtrip(value in any::<String>()) {
            let mut writer = BinaryWriter::new();
            writer.write_string(&value);
            let encoded = writer.finish();
            let mut reader = BinaryReader::new(&encoded);
            prop_assert_eq!(reader.read_string(), Some(value));
            prop_assert_eq!(reader.remaining(), 0);
        }

        #[test]
        fn proptest_color_roundtrip(value in any::<u32>()) {
            let mut writer = BinaryWriter::new();
            writer.write_color(value);
            let encoded = writer.finish();
            let mut reader = BinaryReader::new(&encoded);
            prop_assert_eq!(reader.read_color(), Some(value));
            prop_assert_eq!(reader.remaining(), 0);
        }

        #[test]
        fn proptest_bool_roundtrip(value in any::<bool>()) {
            let mut writer = BinaryWriter::new();
            writer.write_bool(value);
            let encoded = writer.finish();
            let mut reader = BinaryReader::new(&encoded);
            let expected = if value { 1 } else { 0 };
            prop_assert_eq!(reader.read_byte(), Some(expected));
            prop_assert_eq!(reader.remaining(), 0);
        }
    }
}
