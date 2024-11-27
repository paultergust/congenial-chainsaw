use std::{fmt::Display, str::FromStr};

pub struct ChunkType {
    ancillary: u8,
    private: u8,
    reserved: u8,
    safe: u8,
}

impl ChunkType {

    pub fn bytes(&self) -> [u8;4] {
        [self.ancillary, self.private, self.reserved, self.safe]
    }

    pub fn is_valid(&self) -> bool {
        self.bytes().iter().all(|&b| b.is_ascii_alphabetic())
    }

    pub fn is_critical(&self) -> bool {
        self.ancillary.is_ascii_uppercase()
    }
    pub fn is_public(&self) -> bool {
        self.private.is_ascii_uppercase()
    }
    pub fn is_reserved_bit_valid(&self) -> bool {
        self.reserved.is_ascii_uppercase()
    }
    pub fn is_safe_to_copy(&self) -> bool {
        self.safe.is_ascii_lowercase()
    }
}

impl TryFrom<[u8;4]> for ChunkType {
    type Error = & 'static str;

    fn try_from(bytes: [u8;4]) -> Result<Self, Self::Error> {
        let ancillary = bytes[0];
        let private = bytes[1];
        let reserved = bytes[2];
        let safe = bytes[3];
        Ok(ChunkType {
                ancillary, private, reserved, safe
            })
    }
}

impl FromStr for ChunkType {
    type Err = & 'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 4 {
            return Err("str length does not match chunk type parameters");
        }
                // Convert each character into a u8 and collect into an array
        let mut bytes = s.bytes();
        Ok(ChunkType {
            ancillary: bytes.next().unwrap(),
            private: bytes.next().unwrap(),
            reserved: bytes.next().unwrap(),
            safe: bytes.next().unwrap(),
        })
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}{}", self.ancillary, self.private, self.reserved, self.safe)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}

