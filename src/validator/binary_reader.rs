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
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
