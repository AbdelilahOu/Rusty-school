use actix_web::web::{Data as ActData, Json as ActJson, Path as ActPath, Query as ActQuery};
use serde::{Deserialize, Serialize};
use service::{Filters, ListQuery};
use uuid::Uuid;

use crate::AppState;

#[derive(Serialize)]
pub struct ResultResponse<T> {
    pub error: Option<String>,
    pub message: Option<String>,
    pub data: Option<T>,
}

#[derive(Deserialize, Clone)]
pub struct FiltersBody {
    pub filters: Vec<Filters>,
}

pub type TFiltersBody = ActJson<FiltersBody>;
pub type TQueries = ActQuery<ListQuery>;
pub type State = ActData<AppState>;
pub type IdParam = ActPath<Uuid>;
