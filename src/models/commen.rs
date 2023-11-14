use serde::{Serialize, Deserialize};
use service::Filters;

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
