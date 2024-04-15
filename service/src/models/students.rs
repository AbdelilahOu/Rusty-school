use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug)]
pub struct Student {
    pub first_name: String,
    pub last_name: String,
    pub group_id: Option<Uuid>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct StudentQueries {
    pub page: u64,
    pub limit: u64,
    pub fullname: Option<String>,
}
