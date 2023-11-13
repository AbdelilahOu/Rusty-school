use crate::{models::res::ResultResponse, AppState};
use ::service::Mutation;
use actix_web::{
    http::{header::ContentType, StatusCode},
    web::{Data as ActData, Json as ActJson, Path as ActPath},
    HttpResponse,
};
use service::CStudent;
use uuid::Uuid;

pub async fn create_student(body: ActJson<CStudent>, state: ActData<AppState>) -> HttpResponse {
    let res = Mutation::create_student(
        &state.db_conn,
        CStudent {
            first_name: body.first_name.clone(),
            last_name: body.last_name.clone(),
            address: body.address.clone(),
            level: body.level.clone(),
        },
    )
    .await;
    match res {
        Ok(id) => HttpResponse::Ok()
            .status(StatusCode::CREATED)
            .content_type(ContentType::json())
            .json(ResultResponse {
                error: None,
                message: Some("Student created successfully".to_string()),
                data: Some(id.to_string()),
            }),
        Err(e) => HttpResponse::Ok()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .content_type(ContentType::json())
            .json(ResultResponse::<Option<String>> {
                error: Some(e.to_string()),
                message: None,
                data: None,
            }),
    }
}

pub async fn delete_student(params: ActPath<Uuid>, state: ActData<AppState>) -> HttpResponse {
    let id = params.into_inner();
    let delete_res = Mutation::delete_student(&state.db_conn, id).await;

    match delete_res {
        Ok(i) => HttpResponse::Created()
            .content_type(ContentType::json())
            .json(ResultResponse {
                error: None,
                message: Some("Student deleted successfully".to_string()),
                data: Some(i.to_string()),
            }),
        Err(e) => HttpResponse::InternalServerError()
            .content_type(ContentType::json())
            .json(ResultResponse::<Option<String>> {
                error: Some(e),
                message: None,
                data: None,
            }),
    }
}
