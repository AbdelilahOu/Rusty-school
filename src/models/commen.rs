use actix_web::web::{Data as ActData, Json as ActJson, Path as ActPath, Query as ActQuery};
use serde::{Deserialize, Serialize};
use service::query::{Filters, ListQuery};
use service::uuid::Uuid;

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
pub type IdParam = ActPath<Uuid>;
pub type Params<T> = ActPath<T>;
