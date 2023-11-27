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

#[derive(Deserialize, Clone, Debug)]
pub struct TokenResponse {
    pub id_token: Option<String>,
    pub access_token: Option<String>,
    pub token_type: Option<String>,
    pub expires_in: Option<i32>,
    pub refresh_token: Option<String>,
    pub scope: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct AuthQueryParams {
    pub code: String,
}

#[derive(Deserialize, Debug)]
pub struct GoogleUser {
    pub id: String,
    pub email: String,
    pub verified_email: bool,
    pub name: String,
    pub given_name: String,
    pub family_name: String,
    pub picture: String,
    pub locale: String,
}

pub type TFiltersBody = ActJson<FiltersBody>;
pub type TQueries = ActQuery<ListQuery>;
pub type State = ActData<AppState>;
pub type IdParam = ActPath<Uuid>;
pub type AuthQuery = ActQuery<AuthQueryParams>;
