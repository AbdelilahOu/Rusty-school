use actix_web::web::{Data as ActData, Json as ActJson, Path as ActPath, Query as ActQuery};
use serde::{Deserialize, Serialize};
use service::{Filters, ListQuery};
use uuid::Uuid;

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

#[derive(Deserialize, Clone, Debug)]
pub struct TokenResponse {
    pub id_token: String,
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i32,
    pub refresh_token: String,
    pub scope: String,
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

#[derive(Debug, Clone)]
pub struct ConfigObj {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub jwt_secret: String,
    pub jwt_max_age: i64,
    pub db_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user_uuid: Uuid,
    pub exp: usize,
    pub sub: String,
}

pub type TFiltersBody = ActJson<FiltersBody>;
pub type TQueries = ActQuery<ListQuery>;
pub type State = ActData<AppState>;
pub type IdParam = ActPath<Uuid>;
pub type AuthQuery = ActQuery<AuthQueryParams>;
