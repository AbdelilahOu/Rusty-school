pub mod models;
mod mutation;
mod query;
mod transaction;
mod utils;

use serde::Deserialize;

pub use mutation::*;
pub use query::*;
pub use transaction::*;
pub use utils::filters::*;

pub use chrono;
pub use sea_orm;
pub use uuid;

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
