use actix_web::web::Data;
use serde::Serialize;

use crate::AppState;

#[derive(Serialize)]
pub struct ResponseData<T> {
    pub error: Option<String>,
    pub message: Option<String>,
    pub data: Option<T>,
}

pub type State = Data<AppState>;
