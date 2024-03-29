extern crate serde;

use serde::{Serialize, Deserialize};

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub enum Duration {
    Infinite,       // Says until removed otherwise
    Steps(i32),     // Movement steps of the entity
    Updates(i32)    // Game updates
}