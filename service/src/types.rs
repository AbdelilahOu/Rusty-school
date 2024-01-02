use chrono::NaiveDateTime;
use sea_orm::{prelude::Uuid, FromQueryResult};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ListQuery {
    pub page: u64,
    pub limit: u64,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Filters {
    pub feild: String,
    pub operation: String,
    pub value: String,
}

pub struct QueriesFilters {
    pub queries: ListQuery,
    pub filters: Vec<Filters>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CStudent {
    pub first_name: String,
    pub last_name: String,
    pub level_id: Option<Uuid>,
}

pub struct StudentWithAddress {
    pub student: CStudent,
    pub details: CPDetails,
}

pub struct ParentWithAddress {
    pub parent: CParent,
    pub details: CPDetails,
}

pub struct TeacherWithAddress {
    pub teacher: CTeacher,
    pub details: CPDetails,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CTeacher {
    pub first_name: String,
    pub last_name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CParent {
    pub first_name: String,
    pub last_name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CPDetails {
    pub email: Option<String>,
    pub phone: Option<String>,
    pub country_id: Option<Uuid>,
    pub state_id: Option<Uuid>,
    pub city_id: Option<Uuid>,
    pub district_id: Option<Uuid>,
    pub street_id: Option<Uuid>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CCountry {
    pub name: String,
    pub initials: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CState {
    pub name: String,
    pub initials: Option<String>,
    pub country_id: Option<Uuid>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CCity {
    pub name: String,
    pub state_id: Option<Uuid>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CDistrict {
    pub name: String,
    pub city_id: Option<Uuid>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CStreet {
    pub name: String,
    pub zip_code: Option<String>,
    pub street_type: Option<String>,
    pub district_id: Option<Uuid>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub picture: Option<String>,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct CScan {
    pub person_id: Uuid,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CLevel {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CSubject {
    pub name: String,
    pub description: Option<String>,
    pub level_id: Option<Uuid>,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct CGroup {
    pub name: String,
    pub description: Option<String>,
    pub level_id: Option<Uuid>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CRoom {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CClass {
    pub subject_id: Option<Uuid>,
    pub teacher_id: Option<Uuid>,
    pub group_id: Option<Uuid>,
    pub room_id: Option<Uuid>,
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
