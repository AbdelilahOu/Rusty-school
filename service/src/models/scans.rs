use chrono::NaiveDateTime;
use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug)]
pub struct CScan {
    pub person_id: Uuid,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, FromQueryResult)]
pub struct SelectScans {
    pub id: Uuid,
    pub scan_date: Option<NaiveDateTime>,
    pub person_id: Uuid,
    pub person_type: String,
    pub full_name: String,
    pub _id: Option<Uuid>,
}
