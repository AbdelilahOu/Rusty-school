use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Parent {
    pub first_name: String,
    pub last_name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ParentQueries {
    pub page: u64,
    pub limit: u64,
    pub full_name: Option<String>,
}
