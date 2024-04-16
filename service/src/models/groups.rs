use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug)]
pub struct Group {
    pub name: String,
    pub description: Option<String>,
    pub level_id: Option<Uuid>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GroupQuery {
    pub page: u64,
    pub limit: u64,
    pub name: Option<String>,
}
