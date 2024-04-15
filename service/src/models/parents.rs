use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Parent {
    pub first_name: String,
    pub last_name: String,
}
