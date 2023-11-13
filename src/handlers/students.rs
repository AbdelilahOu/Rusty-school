use ::service::Mutation;
use actix_web::{
    http::{header::ContentType, StatusCode},
    web::{self, Json},
    HttpRequest, HttpResponse,
};
use serde::{Deserialize, Serialize};

use crate::AppState;

#[derive(Serialize)]
struct CreateResponse {
    error: Option<String>,
    data: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateBody {
    first_name: String,
    last_name: String,
}

pub async fn create_student(body: Json<CreateBody>, state: web::Data<AppState>) -> HttpResponse {
    HttpResponse::Ok()
        .status(StatusCode::CREATED)
        .content_type(ContentType::json())
        .body("HI")
}

pub async fn delete_student(_req: HttpRequest, data: web::Data<AppState>) -> HttpResponse {
    let delete_res = Mutation::delete_student(&data.db_conn, uuid::Uuid::new_v4()).await;

    match delete_res {
        Ok(i) => HttpResponse::Ok()
            .status(StatusCode::CREATED)
            .content_type(ContentType::json())
            .json(CreateResponse {
                error: None,
                data: Some(i.to_string()),
            }),
        Err(e) => HttpResponse::Ok()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .content_type(ContentType::json())
            .json(CreateResponse {
                error: Some(e),
                data: None,
            }),
    }
}
