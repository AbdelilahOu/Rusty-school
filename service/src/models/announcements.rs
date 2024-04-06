use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct CAnnouncement {
    pub title: String,
    pub description: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub start_date: Option<NaiveDateTime>,
    pub end_date: Option<NaiveDateTime>,
    pub category: String,
    pub targets: Option<String>,
    pub attachements: Option<NaiveDateTime>,
    pub important: Option<bool>,
    pub audience: Option<String>,
    pub alert: Option<NaiveDateTime>,
}
