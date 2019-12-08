use std::path::{Path, PathBuf};
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use crate::{KvsError, Result};
use crate::{Command};

/// The `KvStore` stores string key/value pairs.
///
/// Key/value pairs are persisted to disk in log files. Log files are named after
/// monotonically increasing generation numbers with a `log` extension name.
/// A `BTreeMap` in memory stores the keys and the value locations for fast query.
///
/// ```rust
/// # use kvs::{KvStore, Result};
/// # fn try_main() -> Result<()> {
/// use std::env::current_dir;
/// let mut store = KvStore::open(current_dir()?)?;
/// store.set("key".to_owned(), "value".to_owned())?;
/// let val = store.get("key".to_owned())?;
/// assert_eq!(val, Some("value".to_owned()));
/// # Ok(())
/// # }
/// ```
pub struct KvStore {
   file: std::fs::File
}

impl KvStore {
    /// Opens a `KvStore` with the given path.
    ///
    /// This will create a new directory if the given one does not exist.
    ///
    /// # Errors
    ///
    /// It propagates I/O or deserialization errors during the log replay.
    pub fn open(pathBuf: impl Into<PathBuf>) -> Result<KvStore> {
        let path = pathBuf.into();
        
        println!("Creating path: {:?}", &path);
        fs::create_dir_all(&path)?;        
        
        let filePath = std::path::Path::new(&path).join("kvstore.log");                

        println!("Opening file: {:?}", &filePath);
        let file = match std::fs::File::create(&filePath.as_os_str()) {
            Ok(file) => file,
            Err(e) => return Err(KvsError::Io(e)),
        };
        Ok(KvStore{file: file})
    }

    /// Sets the value of a string key to a string.
    ///
    /// If the key already exists, the previous value will be overwritten.
    ///
    /// # Errors
    ///
    /// It propagates I/O or serialization errors during writing the log.
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let command = Command::Set{key: key, value: value};
        let serialized = serde_json::to_string(&command)?;
        self.file.write(serialized.as_bytes())?;
        Ok(())
    }

    
    /// Gets the string value of a given string key.
    ///
    /// Returns `None` if the given key does not exist.
    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        Ok(None)
    }

    /// Removes a given key.
    ///
    /// # Errors
    ///
    /// It returns `KvsError::KeyNotFound` if the given key is not found.
    ///
    /// It propagates I/O or serialization errors during writing the log.
    pub fn remove(self: &mut Self, key: String) -> Result<()> {
        Ok(())
    }

    /// Clears stale entries in the log.
    pub fn compact(&mut self) -> Result<()> {
        Ok(())
    }
}
