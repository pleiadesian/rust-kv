use failure::Fail;
use serde_json;
use std::io;

#[derive(Fail, Debug)]
pub enum KvsError {
    #[fail(display = "An error occurred")]
    _Error,
    #[fail(display = "IoError occurred")]
    Io(#[fail(cause)] io::Error),
    #[fail(display = "serde_json::Error occurred")]
    Serde(#[fail(cause)] serde_json::Error),
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
