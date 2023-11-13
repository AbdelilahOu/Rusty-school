use ::service::Mutation;
use actix_web::{
    http::{header::ContentType, StatusCode},
    web::{Data as ActData, Json as ActJson, Path as ActPath},
    HttpResponse,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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

pub async fn create_student(body: ActJson<CreateBody>, state: ActData<AppState>) -> HttpResponse {
    HttpResponse::Ok()
        .status(StatusCode::CREATED)
        .content_type(ContentType::json())
        .body("HI")
}

pub async fn delete_student(params: ActPath<Uuid>, state: ActData<AppState>) -> HttpResponse {
    let id = params.into_inner();
    let delete_res = Mutation::delete_student(&state.db_conn, id).await;

    match delete_res {
        Ok(i) => HttpResponse::Created()
            .content_type(ContentType::json())
            .json(CreateResponse {
                error: None,
                data: Some(i.to_string()),
            }),
        Err(e) => HttpResponse::InternalServerError()
            .content_type(ContentType::json())
            .json(CreateResponse {
                error: Some(e),
                data: None,
            }),
    }
}
