use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Level {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LevelQueries {
    pub page: u64,
    pub limit: u64,
    pub name: Option<String>,
}
