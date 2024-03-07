use chrono::{NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug)]
pub struct CTimeTable {}

#[derive(Deserialize, Serialize, Debug)]
pub struct CEvent {
    pub start_time: NaiveTime,
    pub full_date: NaiveDate,
    pub event_title: String,
    pub event_description: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CActivity {
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub day_of_week: u32,
    pub activity_title: String,
    pub activity_description: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CLecture {
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub day_of_week: u32,
    pub class_id: Uuid,
}
