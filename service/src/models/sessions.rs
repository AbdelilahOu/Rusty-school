use chrono::NaiveDateTime;
use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug)]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
    pub user_agent: String,
    pub client_ip: String,
    pub is_blocked: bool,
    pub refresh_token: String,
    pub expires_at: NaiveDateTime,
}

#[derive(Deserialize, Serialize, Debug, FromQueryResult)]
pub struct GetSessionWithRole {
    pub id: Uuid,
    pub user_id: Uuid,
    pub is_blocked: bool,
    pub refresh_token: String,
    pub expires_at: NaiveDateTime,
    pub role: String,
}
