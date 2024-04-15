use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Level {
    pub name: String,
    pub description: Option<String>,
}
