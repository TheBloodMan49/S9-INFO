use crate::Result;
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::error::Error::{
    CorruptedPng, DeletionFailure, FileUnreadable, FileUnwritable, IncorrectKey, NoMessageFound,
};
use crate::png::Png;
use std::fs;
use std::str::FromStr;

pub fn encode(path: &String, key: &str, message: &str) -> Result<()> {
    if let Ok(data) = fs::read(path) {
        if let Ok(mut png) = Png::try_from(data.as_slice()) {
            if let Ok(chunk_type) = ChunkType::from_str(key) {
                let chunk = Chunk::new(chunk_type, message.as_bytes().to_vec());
                png.append_chunk(chunk);
                let encoded_data = png.as_bytes();
                if fs::write(path, encoded_data).is_err() {
                    return Err(Box::new(FileUnwritable));
                }
                Ok(())
            } else {
                Err(Box::new(IncorrectKey(key.to_string())))
            }
        } else {
            Err(Box::new(CorruptedPng))
        }
    } else {
        Err(Box::new(FileUnreadable))
    }
}

pub fn decode(path: &String, key: &str) -> Result<()> {
    if let Ok(data) = fs::read(path) {
        if let Ok(png) = Png::try_from(data.as_slice()) {
            if let Some(chunk) = png.chunk_by_type(&key) {
                let message = chunk.data_as_string()?;
                println!("{}", message);
                Ok(())
            } else {
                Err(Box::new(NoMessageFound))
            }
        } else {
            Err(Box::new(CorruptedPng))
        }
    } else {
        Err(Box::new(FileUnreadable))
    }
}

pub fn remove(path: &String, key: &str) -> Result<()> {
    if let Ok(data) = fs::read(path) {
        if let Ok(mut png) = Png::try_from(data.as_slice()) {
            if png.remove_chunk(&key).is_ok() {
                let encoded_data = png.as_bytes();
                if fs::write(path, encoded_data).is_err() {
                    return Err(Box::new(FileUnwritable));
                }
                Ok(())
            } else {
                Err(Box::new(DeletionFailure))
            }
        } else {
            Err(Box::new(CorruptedPng))
        }
    } else {
        Err(Box::new(FileUnreadable))
    }
}
