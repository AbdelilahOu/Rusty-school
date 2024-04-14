use actix_web::web::Query as ActQuery;
use serde::{Deserialize, Serialize};
use service::chrono::NaiveDateTime;
use service::uuid::Uuid;

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

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub session_id: Uuid,
    pub user_id: Uuid,
    pub exp: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogInResponse {
    pub session_id: Uuid,
    pub email: String,
    pub fullname: String,
    pub access_token: String,
    pub refresh_token: String,
    pub access_token_expires_at: NaiveDateTime,
    pub refresh_token_expires_at: NaiveDateTime,
}

pub type AuthQuery = ActQuery<AuthQueryParams>;
