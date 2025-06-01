//! .

use super::{MAGIC, VERSION};

pub type DecodeResult<T> = std::result::Result<T, DecodeError>;

#[derive(thiserror::Error, Debug)]
pub enum DecodeError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Wrong magic used: {0}, expected {MAGIC}")]
    WrongMagic(u32),
    #[error("Wrong version used: {0}, expected {VERSION}")]
    WrongVersion(u32),
}
