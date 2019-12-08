
extern crate failure;

use std::io;
use failure::Error;


#[derive(Debug, Fail)]
pub enum KvsError {
    #[fail(display = "file not found: {}", name)]
    FileNotFound {
        name: String,
    },

    #[fail(display = "{}", _0)]
    Io(#[cause] io::Error),

    #[fail(display = "Serialize error{}", _0)]
    Serialization(#[cause] serde_json::error::Error),

    #[fail(display = "Key not found")]
    KeyNotFound,
    /// Unexpected command type error.
    /// It indicated a corrupted log or a program bug.
    #[fail(display = "Unexpected command type")]
    UnexpectedCommandType,
}

impl From<io::Error> for KvsError {
    fn from(err: io::Error) -> KvsError {
        KvsError::Io(err)
    }
}

impl From<serde_json::error::Error> for KvsError {
    fn from(err: serde_json::error::Error) -> KvsError {
        KvsError::Serialization(err)
    }
}

pub type Result<T> = std::result::Result<T, KvsError>;
