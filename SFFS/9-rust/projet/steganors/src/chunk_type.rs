use crate::{Error, Result};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(PartialEq, Eq, Debug)]
pub struct ChunkType {
    chunk_type: [u8; 4],
}

impl ChunkType {
    fn is_valid_for_creation(&self) -> Result<bool> {
        for value in self.chunk_type {
            if !value.is_ascii_alphabetic() {
                return Err(Box::new(ChunkTypeDecodingError::BadByte(value)));
            }
        }
        Ok(true)
    }

    pub fn bytes(&self) -> [u8; 4] {
        self.chunk_type
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;

    fn try_from(value: [u8; 4]) -> Result<Self> {
        let chunk = ChunkType { chunk_type: value };
        match chunk.is_valid_for_creation() {
            Ok(_) => Ok(chunk),
            Err(e) => Err(e),
        }
    }
}

impl FromStr for ChunkType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let values = s.as_bytes();
        if s.len() != 4 {
            return Err(Box::new(ChunkTypeDecodingError::BadLength(s.len())));
        }
        let chunk = ChunkType {
            chunk_type: [values[0], values[1], values[2], values[3]],
        };
        match chunk.is_valid_for_creation() {
            Ok(_) => Ok(chunk),
            Err(e) => Err(e),
        }
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", str::from_utf8(&self.chunk_type).unwrap())
    }
}

#[derive(Debug)]
pub enum ChunkTypeDecodingError {
    BadByte(u8),
    BadLength(usize),
}

impl std::error::Error for ChunkTypeDecodingError {}

impl Display for ChunkTypeDecodingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ChunkTypeDecodingError::BadByte(byte) => {
                write!(f, "Un mauvais octet fut trouvé: 0x{byte:01X}")
            }
            ChunkTypeDecodingError::BadLength(length) => {
                write!(f, "Mauvaise taille, taille trouvée: {length}")
            }
        }
    }
}

// Tests pré-écrits, inutile d'y toucher
#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunktype_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunktype_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunktype_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid_for_creation().unwrap());

        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(chunk.is_valid_for_creation().unwrap());
    }

    #[test]
    pub fn test_chunktype_invalid_chunk_is_invalid() {
        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunktype_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunktype_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
