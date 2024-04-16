use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug)]
pub struct Class {
    pub subject_id: Option<Uuid>,
    pub teacher_id: Option<Uuid>,
    pub group_id: Option<Uuid>,
    pub room_id: Option<Uuid>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ClassQuery {
    pub page: u64,
    pub limit: u64,
    pub subject_id: Option<Uuid>,
    pub teacher_id: Option<Uuid>,
    pub group_id: Option<Uuid>,
}
