use crate::{models::commen::*, AppState};
use actix_web::{
    http::{header::ContentType, StatusCode},
    web::{Data as ActData, Json as ActJson, Path as ActPath, Query as ActQuery},
    HttpResponse,
};

use service::*;
use uuid::Uuid;

type State = ActData<AppState>;

pub async fn create_state(state: State) -> HttpResponse {
    HttpResponse::Ok().status(StatusCode::OK).body("great")
}

pub async fn get_state(state: State) -> HttpResponse {
    HttpResponse::Ok().status(StatusCode::OK).body("great")
}

pub async fn update_state(state: State) -> HttpResponse {
    HttpResponse::Ok().status(StatusCode::OK).body("great")
}

pub async fn delete_state(state: State) -> HttpResponse {
    HttpResponse::Ok().status(StatusCode::OK).body("great")
}
