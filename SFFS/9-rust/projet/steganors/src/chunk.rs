use std::fmt::{Display, Formatter};

use crate::chunk_type::ChunkType;
use crate::{Error, Result};

pub struct Chunk {
    length: u32,
    chunk_type: ChunkType,
    data: Vec<u8>,
    crc: u32,
}

impl TryFrom<&[u8]> for Chunk {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self> {
        let (int_bytes, rest) = value.split_at(size_of::<u32>());
        let length = u32::from_be_bytes(int_bytes.try_into()?);

        let (chunk_type_slice, rest) = rest.split_at(4);
        let chunk_type_slice: [u8; 4] = chunk_type_slice.try_into()?;
        let chunk_type = ChunkType::try_from(chunk_type_slice)?;

        let (data_slice, rest) = rest.split_at(length as usize);
        let data = data_slice.to_vec();

        let (crc_bytes, _useless) = rest.split_at(size_of::<u32>());
        let crc = u32::from_be_bytes(crc_bytes.try_into()?);

        let checksum = crc32fast::hash(&[&chunk_type.bytes(), data.as_slice()].concat());
        if crc == checksum {
            Ok(Self {
                length,
                chunk_type,
                data,
                crc,
            })
        } else {
            Err(Box::new(ChunkError::InvalidChecksum(crc, checksum)))
        }
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", str::from_utf8(self.data.as_slice()).unwrap())
    }
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let crc = crc32fast::hash(&[&chunk_type.bytes(), data.as_slice()].concat());
        Chunk {
            length: data.len() as u32,
            chunk_type,
            data,
            crc,
        }
    }

    pub fn length(&self) -> u32 {
        self.length
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    fn crc(&self) -> u32 {
        self.crc
    }

    pub fn data_as_string(&self) -> Result<String> {
        let string = str::from_utf8(self.data.as_slice());
        match string {
            Ok(text) => Ok(text.to_string()),
            Err(_) => Err(Box::new(ChunkError::DataToStringError)),
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut result = Vec::with_capacity(4 + 4 + self.data.len() + 4);
        result.extend(&self.length().to_be_bytes());
        result.extend(&self.chunk_type.bytes());
        result.extend(&self.data);
        result.extend(&self.crc.to_be_bytes());
        result
    }
}

#[derive(Debug)]
pub enum ChunkError {
    DataToStringError,
    InvalidChecksum(u32, u32),
}

impl std::error::Error for ChunkError {}

impl Display for ChunkError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ChunkError::DataToStringError => write!(
                f,
                "Une erreur est survenue lors de la conversion des données en String."
            ),
            ChunkError::InvalidChecksum(expected, found) => write!(
                f,
                "Somme de contrôle invalide, attendue: {expected} trouvée: {found}"
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_chunk_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}
