use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug)]
pub struct CTimeTable {}

#[derive(Deserialize, Serialize, Debug)]
pub struct CEvent {
    pub start_time: NaiveDateTime,
    pub full_date: NaiveDateTime,
    pub event_title: String,
    pub event_description: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CActivity {
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub day_of_week: String,
    pub activity_title: String,
    pub activity_description: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CLecture {
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub day_of_week: String,
    pub class_id: Uuid,
}
