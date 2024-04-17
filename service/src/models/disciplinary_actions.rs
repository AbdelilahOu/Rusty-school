use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug)]
pub struct Disciplinary {
    pub student_id: Uuid,
    pub issued_at: Option<NaiveDateTime>,
    pub description: String,
    pub consequences: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DisciplinaryQuery {
    pub page: u64,
    pub limit: u64,
    pub student_id: Option<Uuid>,
    pub issued_at: Option<NaiveDateTime>,
}
