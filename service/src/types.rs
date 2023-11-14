use sea_orm::prelude::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct CStudent {
    pub first_name: String,
    pub last_name: String,
    pub address: String,
    pub level: String,
}

#[derive(Serialize, Debug)]
pub struct GStudent {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub address: String,
    pub level: String,
}

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
