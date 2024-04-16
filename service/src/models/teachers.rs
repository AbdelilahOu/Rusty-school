use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Teacher {
    pub first_name: String,
    pub last_name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TeacherQueries {
    pub page: u64,
    pub limit: u64,
    pub full_name: Option<String>,
}
