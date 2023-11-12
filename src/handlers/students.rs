// use ::service::Mutation;
use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpRequest, HttpResponse,
};
use serde::Serialize;

#[derive(Serialize)]
struct CreateResponse {
    error: Option<String>,
    data: String,
}

pub async fn create_student(_req: HttpRequest) -> HttpResponse {
    // Mutation::create_student().await;

    HttpResponse::Ok()
        .status(StatusCode::CREATED)
        .content_type(ContentType::json())
        .json(CreateResponse {
            error: None,
            data: String::from("Student created successfully"),
        })
}
