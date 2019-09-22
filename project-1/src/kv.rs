use std::collections::HashMap;

pub struct KvStore {
    store: std::collections::HashMap<String, String>,
}

impl KvStore {
    pub fn new() -> KvStore {
        KvStore {
            store: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: String) {
        self.store.insert(key, value);
    }

    pub fn get(&mut self, key: String) -> Option<String> {
        let result = self.store.get(&key);
        match result {
            Some(result) => Some(result.clone()),
            None => None,
        }
    }

    pub fn remove(&mut self, key: String) -> Option<String> {
        self.store.remove(&key)
    }
}
