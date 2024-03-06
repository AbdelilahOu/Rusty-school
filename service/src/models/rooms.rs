use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct CRoom {
    pub name: String,
    pub description: Option<String>,
}
