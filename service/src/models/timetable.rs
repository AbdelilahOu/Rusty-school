use chrono::{NaiveDate, NaiveTime};
use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, PartialEq, FromQueryResult)]
pub struct SelectTimeTable {
    pub id: Uuid,
    pub item_type: String,
    pub day_of_week: Option<String>,
    pub full_date: Option<NaiveDate>,
    pub start_time: Option<NaiveTime>,
    pub end_time: Option<NaiveTime>,
    pub title: Option<String>,
}

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
