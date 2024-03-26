use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug)]
pub struct CGrade {
    pub student_id: Option<Uuid>,
    pub assignment_id: Option<Uuid>,
    pub score: f32,
    pub feedback: String,
}
