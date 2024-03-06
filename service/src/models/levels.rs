use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct CLevel {
    pub name: String,
    pub description: Option<String>,
}
