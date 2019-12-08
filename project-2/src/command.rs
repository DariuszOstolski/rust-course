use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) enum Command {
    Set{key: String, value: String},
    Get{key: String},
    Remove{key: String}
}

