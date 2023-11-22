use crate::{models::commen::*, AppState};
use actix_web::{
    http::{header::ContentType, StatusCode},
    web::{Data as ActData, Json as ActJson, Path as ActPath, Query as ActQuery},
    HttpResponse,
};

use service::*;
use uuid::Uuid;

type State = ActData<AppState>;

pub async fn create_city(state: State) -> HttpResponse {
    HttpResponse::Ok().status(StatusCode::OK).body("great")
}

pub async fn get_city(state: State) -> HttpResponse {
    HttpResponse::Ok().status(StatusCode::OK).body("great")
}

pub async fn update_city(state: State) -> HttpResponse {
    HttpResponse::Ok().status(StatusCode::OK).body("great")
}

pub async fn delete_city(state: State) -> HttpResponse {
    HttpResponse::Ok().status(StatusCode::OK).body("great")
}
