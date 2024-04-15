use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Room {
    pub name: String,
    pub description: Option<String>,
}
