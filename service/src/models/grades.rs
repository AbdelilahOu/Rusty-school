use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug)]
pub struct CGrade {
    pub student_id: Uuid,
    pub assignment_id: Uuid,
    pub score: f32,
    pub feedback: Option<String>,
}
