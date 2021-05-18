use crate::Result;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::{self, File, OpenOptions};
use std::io::{BufReader, BufWriter, Write};
use std::path::Path;

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
    buf_writer: BufWriter<File>,
    buf_reader: BufReader<File>,
}

impl KvStore {
    /// Set a key-value pair
    pub fn set(&mut self, key: String, val: String) -> Result<()> {
        let command = Command::Set { key, val };
        serde_json::to_writer(&mut self.buf_writer, &command)?;
        self.buf_writer.flush();
        Ok(())
    }

    /// Get value by key
    ///
    /// Return `None` if key does not exists
    pub fn get(&self, key: String) -> Result<Option<String>> {
        Ok(Some("None".to_owned()))
    }

    /// Remove a key-value pair
    pub fn remove(&mut self, key: String) -> Result<()> {
        let command = Command::Remove { key };
        let commands = serde_json::from_reader(&mut self.buf_reader)?;  // TODO: specify a type
        Ok(())
    }

    /// Open a KV store from disk
    pub fn open(path: &Path) -> Result<KvStore> {
        let logpath = path.join("kv.log");
        fs::create_dir_all(path)?;
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&logpath)?;
        let mut buf_writer = BufWriter::new(file);
        let file = OpenOptions::new().read(true).open(&logpath)?;
        let mut buf_reader = BufReader::new(file);
        Ok(KvStore {
            buf_writer,
            buf_reader,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
enum Command {
    Set { key: String, val: String },
    Remove { key: String },
}
