#![allow(dead_code)]
#![allow(unused)]

pub mod chunk;
pub mod chunk_type;
pub mod commands;
pub mod error;
pub mod png;

// Alias pour écrire plus rapidement
pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;
