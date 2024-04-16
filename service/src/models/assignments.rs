use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug)]
pub struct Assignment {
    pub title: String,
    pub description: String,
    pub due_date: NaiveDateTime,
    pub submission_type: String,
    pub gradin_rubric_id: Option<Uuid>,
    pub file: Option<String>,
    pub teacher_id: Option<Uuid>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AssignmentQueries {
    pub page: u64,
    pub limit: u64,
    pub title: Option<String>,
    pub due_date: Option<NaiveDateTime>,
    pub teacher_id: Option<Uuid>,
}
