use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug)]
pub struct Grade {
    pub student_id: Uuid,
    pub assignment_id: Uuid,
    pub score: f32,
    pub feedback: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GradeQueries {
    pub page: u64,
    pub limit: u64,
    pub student_id: Option<Uuid>,
}
