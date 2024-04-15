use actix_web::web::{Data as ActData, Json as ActJson, Query as ActQuery};
use serde::{Deserialize, Serialize};
use service::query::{Filters, ListQuery};

use crate::AppState;

#[derive(Serialize)]
pub struct ResponseData<T> {
    pub error: Option<String>,
    pub message: Option<String>,
    pub data: Option<T>,
}

#[derive(Deserialize, Clone)]
pub struct FiltersBody {
    pub filters: Vec<Filters>,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub jwt_secret: String,
    pub db_url: String,
}

pub type TFiltersBody = ActJson<FiltersBody>;
pub type TQueries = ActQuery<ListQuery>;
pub type State = ActData<AppState>;
