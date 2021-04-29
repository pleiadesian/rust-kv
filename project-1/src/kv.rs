use std::collections::HashMap;

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
    kv: HashMap<String, String>,
}

impl KvStore {
    /// Create `KvStore`
    pub fn new() -> KvStore {
        KvStore {
            kv: HashMap::new(),
        }
    }

    /// Set a key-value pair
    pub fn set(&mut self, key: String, val: String) {
        self.kv.insert(key, val);
    }

    /// Get value by key
    /// 
    /// Return `None` if key does not exists
    pub fn get(&self, key: String) -> Option<String> {
        self.kv.get(&key).cloned()
    }

    /// Remove a key-value pair
    pub fn remove(&mut self, key: String) {
        self.kv.remove(&key);
    }
}