#[allow(dead_code)]
#[macro_use] extern crate failure_derive;

pub use error::{KvsError, Result};
pub use kv::KvStore;
use command::Command;

mod error;
mod kv;
mod command;
