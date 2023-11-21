use sea_orm::prelude::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ListQuery {
    pub page: u64,
    pub limit: u64,
}

#[derive(Deserialize, Clone)]
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
    pub level: String,
}

#[derive(Serialize, Debug)]
pub struct GStudent {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub level: String,
}

pub struct StudentWithAddress {
    pub student: CStudent,
    pub contact: CContact,
}

pub struct ParentWithAddress {
    pub parent: CParent,
    pub contact: CContact,
}

pub struct TeacherWithAddress {
    pub teacher: CTeacher,
    pub contact: CContact,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CTeacher {
    pub first_name: String,
    pub last_name: String,
}

#[derive(Serialize, Debug)]
pub struct GTeacher {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CParent {
    pub first_name: String,
    pub last_name: String,
}

#[derive(Serialize, Debug)]
pub struct GParent {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CContact {
    pub email: Option<String>,
    pub phone: Option<String>,
    pub country_id: Option<Uuid>,
    pub state_id: Option<Uuid>,
    pub city_id: Option<Uuid>,
    pub district_id: Option<Uuid>,
    pub street_id: Option<Uuid>,
}
