pub trait KvStore {
    fn set(&mut self, key: String, value: String) {

    }

    fn get(&mut self, key: String) -> Option<String> {
        None
    }

    fn remove(&mut self, key: String) {

    }
}