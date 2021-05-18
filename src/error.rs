use failure::Fail;
use std::io;

#[derive(Fail, Debug)]
pub enum KvsError {
    #[fail(display = "An error occurred")]
    _Error,
    #[fail(display = "IoError occurred")]
    Io(#[fail(cause)] io::Error)
}

impl From<io::Error> for KvsError {
    fn from(err: io::Error) -> KvsError {
        KvsError::Io(err)
    }
}

/// KV store Error type
pub type Result<T> = std::result::Result<T, KvsError>;
