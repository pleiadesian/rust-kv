use failure::Fail;

#[derive(Fail, Debug)]
pub enum KvsError {
    #[fail(display = "An error occurred")]
    _Error,
}

/// KV store Error type
pub type Result<T> = std::result::Result<T, KvsError>;
