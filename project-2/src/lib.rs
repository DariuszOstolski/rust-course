#[allow(dead_code)]
#[macro_use] extern crate failure_derive;

pub use error::{KvsError, Result};
pub use kv::KvStore;

mod error;
mod kv;
