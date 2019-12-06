use std::path::{Path, PathBuf};
use crate::{KvsError, Result};


#[allow(dead_code)]
pub struct KvStore {

}

impl KvStore {
    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        Ok(KvStore{})
    }

    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        Ok(())
    }

    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        Ok(None)
    }

    pub fn remove(self: &mut Self, key: String) -> Result<()> {
        Ok(())
    }
}