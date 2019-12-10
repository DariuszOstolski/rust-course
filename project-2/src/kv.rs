use std::path::{PathBuf};
use std::fs;

use std::io::prelude::*;
use std::collections::BTreeMap;
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
   file: std::fs::File,
   index: BTreeMap<String, String>
}

impl KvStore {
    /// Opens a `KvStore` with the given path.
    ///
    /// This will create a new directory if the given one does not exist.
    ///
    /// # Errors
    ///
    /// It propagates I/O or deserialization errors during the log replay.
    pub fn open(path_buf: impl Into<PathBuf>) -> Result<KvStore> {
        let path = path_buf.into();
        
        println!("Creating path: {:?}", &path);
        fs::create_dir_all(&path)?;        
        
        let file_path = std::path::Path::new(&path).join("kvstore.log");                
        let mut map = BTreeMap::new();
        replay_log(&file_path, &mut map)?;
        println!("Opening file: {:?}", &file_path);
        let file = match std::fs::OpenOptions::new().create(true).append(true).open(&file_path.as_os_str()) {
            Ok(file) => file,
            Err(e) => return Err(KvsError::Io(e)),
        };
        Ok(KvStore{file: file, index: map})
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
        replay_command(&command, &mut self.index);
        Ok(())
    }

    
    /// Gets the string value of a given string key.
    ///
    /// Returns `None` if the given key does not exist.
    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        let command = Command::Get{key: key};
        Ok(replay_command(&command, &mut self.index))
    }

    /// Removes a given key.
    ///
    /// # Errors
    ///
    /// It returns `KvsError::KeyNotFound` if the given key is not found.
    ///
    /// It propagates I/O or serialization errors during writing the log.
    pub fn remove(self: &mut Self, key: String) -> Result<()> {
        if self.index.contains_key(&key) {
            let command = Command::Remove{key: key};
            let serialized = serde_json::to_string(&command)?;
            self.file.write(serialized.as_bytes())?;
            replay_command(&command, &mut self.index);
            Ok(())
        } else {
            Err(KvsError::KeyNotFound)
        }        
    }

    /// Clears stale entries in the log.
    pub fn compact(&mut self) -> Result<()> {
        Ok(())
    }
}


fn replay_command(command: &Command, map: &mut BTreeMap<String, String>) -> Option<String> {
    println!("Executing command {:?}", command);
    match command {
        Command::Set{key, value} => {
            map.insert(key.to_string(), value.to_string());
            None
        },
        Command::Get{key} => { 
            let result = map.get(key);
            match result {
                None => None,
                Some(x) => Some(x.to_string())
            }
        },
        Command::Remove{key} => map.remove(key)
    }
}

fn replay_log(file_path: &PathBuf, map: &mut BTreeMap<String, String>) -> Result<()> {        
    println!("Opening log: {:?}", file_path);
    let file = match std::fs::OpenOptions::new().read(true).open(&file_path.as_os_str()) {
        Ok(file) => file,
        Err(e) => { 
            println!("Replay log does not exists, exiting: {:?}", e);
            return Ok(()) }
    };
    println!("Replaying log");
    let deserializer = serde_json::Deserializer::from_reader(file);
    let iterator = deserializer.into_iter::<Command>();
    for command in iterator {        
        match command {
            Ok(item) => { replay_command(&item, map); },
            Err(e) => return Err(KvsError::Serialization(e))
        }            
    }
    println!("Log replayed succesfully");
    Ok(())
}
