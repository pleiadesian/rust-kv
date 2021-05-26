use failure::Fail;
use std::io;

/// Error type for KV store
#[derive(Fail, Debug)]
pub enum KvsError {
    /// IO error
    #[fail(display = "IoError occurred")]
    Io(#[fail(cause)] io::Error),
    /// Serde error
    #[fail(display = "serde_json::Error occurred")]
    Serde(#[fail(cause)] serde_json::Error),
    /// Key not found
    #[fail(display = "Key not found")]
    KeyNotFound,
}

impl From<io::Error> for KvsError {
    fn from(err: io::Error) -> KvsError {
        KvsError::Io(err)
    }
}

impl From<serde_json::Error> for KvsError {
    fn from(err: serde_json::Error) -> KvsError {
        KvsError::Serde(err)
    }
}

/// KV store Error type
pub type Result<T> = std::result::Result<T, KvsError>;
