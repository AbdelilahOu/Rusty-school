use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug)]
pub struct Teacher {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub person_id: Option<Uuid>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TeacherQuery {
    pub page: u64,
    pub limit: u64,
    pub full_name: Option<String>,
}
