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

#[derive(Serialize, Deserialize)]
pub struct ListQuery {
    pub page: u64,
    pub limit: u64,
}
