use std::collections::HashMap;
use std::path::{Path};

/// Result type
pub type Result<T> = std::result::Result<T, i32>;

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
#[derive(Default)]
pub struct KvStore {
    kv: HashMap<String, String>,
}

impl KvStore {
    /// Create `KvStore`
    pub fn new() -> KvStore {
        KvStore { kv: HashMap::new() }
    }

    /// Set a key-value pair
    pub fn set(&mut self, key: String, val: String) -> Result<()> {
        self.kv.insert(key, val);
        Ok(())
    }

    /// Get value by key
    ///
    /// Return `None` if key does not exists
    pub fn get(&self, key: String) -> Result<Option<String>> {
        Ok(self.kv.get(&key).cloned())
    }

    /// Remove a key-value pair
    pub fn remove(&mut self, key: String) -> Result<()> {
        self.kv.remove(&key);
        Ok(())
    }

    /// TODO
    pub fn open(_: &Path) -> Result<KvStore> {
        Ok(KvStore::new())
    }
}
