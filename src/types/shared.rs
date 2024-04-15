use actix_web::web::{Data, Json, Query};
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

pub type TFiltersBody = Json<FiltersBody>;
pub type TQueries = Query<ListQuery>;
pub type State = Data<AppState>;
