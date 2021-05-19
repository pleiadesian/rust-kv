use crate::{Result, KvsError};
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
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
        self.buf_writer.write(&[b'\n']);
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
        let lines = self.buf_reader.by_ref().lines();
        let mut command_latest = Command::Set {
            key: "".to_owned(),
            val: "".to_owned(),
        };
        let mut found = false;
        for line in lines {
            let command_line: Command = serde_json::from_str(line?.as_str())?;
            match command_line {
                Command::Set { key, val } => {
                    if key == key {
                        command_latest = Command::Set { key, val };
                        found = true;
                    }
                }
                Command::Remove { key } => {
                    if key == key {
                        command_latest = Command::Remove { key };
                        found = false;
                    }
                }
            }
        }
        if found {
            serde_json::to_writer(&mut self.buf_writer, &command)?;
            self.buf_writer.write(&[b'\n']);
            self.buf_writer.flush();
            Ok(())
        } else {
            Err(KvsError::KeyNotFound)
        }
    }

    /// Open a KV store from disk
    pub fn open(path: &Path) -> Result<KvStore> {
        let logpath = path.join("kv.log");
        fs::create_dir_all(path)?;
        let file = OpenOptions::new()
            .write(true)
            .append(true)
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
