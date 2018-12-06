use crate::hostcalls;

/// A key-value store that persists between requests.
pub struct KVStore {
    _private: (),
}

impl KVStore {
    pub(crate) fn global() -> KVStore {
        KVStore { _private: () }
    }

    /// Insert a value into the store at the given key.
    ///
    /// Returns `true` if key was not present before this call.
    pub fn insert(&mut self, key: &str, value: &[u8]) -> bool {
        hostcalls::kvstore_insert(key, value)
    }

    /// Insert a value into the store at the given key if that key is
    /// not already present.
    ///
    /// Returns `true` if key was not present before this call.
    pub fn upsert(&mut self, key: &str, value: &[u8]) -> bool {
        hostcalls::kvstore_upsert(key, value)
    }

    /// Append to the value at the given key if that key is present in
    /// the store. If not, insert the value.
    ///
    /// Returns `true` if key was not present before this call.
    pub fn append(&mut self, key: &str, value: &[u8]) -> bool {
        hostcalls::kvstore_append(key, value)
    }

    /// Get a value from the store at the given key.
    ///
    /// If the key is not present, returns `None`.
    pub fn get(&self, key: &str) -> Option<Vec<u8>> {
        hostcalls::kvstore_get(key)
    }

    /// Remove a value from the store at the given key.
    ///
    /// Returns `true` if the key was removed, or `false` if the key
    /// was not present before this call.
    pub fn remove(&mut self, key: &str) -> bool {
        hostcalls::kvstore_remove(key)
    }
}
