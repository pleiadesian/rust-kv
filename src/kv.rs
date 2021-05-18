use crate::Result;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{BufReader, BufWriter};
use std::path::Path;
use serde::{Serialize, Deserialize};
use serde_json;

/// `KvStore` stores key-value pairs
///
/// Example:
///
/// ```rust
/// # use kvs::KvStore;
/// let mut store = KvStore::new();
/// store.set("key1".to_owned(), "value1".to_owned());
/// let value = store.get("key1".to_owned());
/// assert_eq!(value, Some("value1".to_owned()))
/// ```
pub struct KvStore {
    // kv: HashMap<String, String>,
    buf_writer: BufWriter<File>,
}

impl KvStore {
    /// Set a key-value pair
    pub fn set(&mut self, key: String, val: String) -> Result<()> {
        // self.kv.insert(key, val);
        let command = Command::Set { key: key, val: val };
        self.buf_writer.write(command.serialize());
        Ok(())
    }

    /// Get value by key
    ///
    /// Return `None` if key does not exists
    pub fn get(&self, key: String) -> Result<Option<String>> {
        // Ok(self.kv.get(&key).cloned())
    }

    /// Remove a key-value pair
    pub fn remove(&mut self, key: String) -> Result<()> {
        // self.kv.remove(&key);
        // Ok(())
    }

    /// Open a KV store from disk
    pub fn open(path: &Path) -> Result<KvStore> {
        fs::create_dir_all(path)?;
        let file = File::open(path.join("kv.log"))?;
        serde_json::to_writer(self.buf_writer);
        let mut buf_writer = BufWriter::new(file);
        Ok(KvStore {
            buf_writer: buf_writer,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
enum Command {
    Set { key: String, val: String },
    Remove { key: String },
}
