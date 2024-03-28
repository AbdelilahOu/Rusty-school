use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug)]
pub struct CDisciAction {
    pub student_id: Uuid,
    pub issued_at: Option<NaiveDateTime>,
    pub description: String,
    pub consequences: String,
}
