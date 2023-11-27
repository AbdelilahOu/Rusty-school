use actix_web::{http::StatusCode, HttpResponse};
use url::Url;

use crate::models::commen::State;

pub async fn login(state: State) -> HttpResponse {
    let url = Url::parse("XXXXXXXXXXXXXXXXXXXXX").unwrap();
    HttpResponse::Ok().status(StatusCode::OK).body("hi")
}
