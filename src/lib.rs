#![deny(missing_docs)]
//! A simple key-value store

pub use error::{Result, KvsError};
pub use kv::KvStore;

mod error;
mod kv;
