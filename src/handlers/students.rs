use crate::{models::res::ResultResponse, AppState};
use actix_web::{
    http::{header::ContentType, StatusCode},
    web::{Data as ActData, Json as ActJson, Path as ActPath, Query as ActQuery},
    HttpResponse,
};

use service::{CStudent, ListQuery, QueriesFilters, ServiceMutation, ServiceQuery};
use uuid::Uuid;

pub async fn create_student(body: ActJson<CStudent>, state: ActData<AppState>) -> HttpResponse {
    let res = ServiceMutation::create_student(&state.db_conn, body.into_inner()).await;
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
    let delete_res = ServiceMutation::delete_student(&state.db_conn, params.into_inner()).await;

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

pub async fn get_student(params: ActPath<Uuid>, state: ActData<AppState>) -> HttpResponse {
    let selected_student = ServiceQuery::get_student(params.into_inner(), &state.db_conn).await;

    match selected_student {
        Ok(i) => HttpResponse::Created()
            .content_type(ContentType::json())
            .json(ResultResponse {
                error: None,
                message: Some("Student selected successfully".to_string()),
                data: Some(i),
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

pub async fn get_students(queries: ActQuery<ListQuery>, state: ActData<AppState>) -> HttpResponse {
    let students = ServiceQuery::list_students(
        QueriesFilters {
            queries: queries.into_inner(),
            filters: Vec::<String>::new(),
        },
        &state.db_conn,
    )
    .await;

    match students {
        Ok(i) => HttpResponse::Created()
            .content_type(ContentType::json())
            .json(ResultResponse {
                error: None,
                message: Some("Students selected successfully".to_string()),
                data: Some(i),
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

pub async fn update_student(
    id: ActPath<Uuid>,
    body: ActJson<CStudent>,
    state: ActData<AppState>,
) -> HttpResponse {
    let update_res =
        ServiceMutation::update_student(&state.db_conn, id.into_inner(), body.into_inner()).await;

    match update_res {
        Ok(i) => HttpResponse::Created()
            .content_type(ContentType::json())
            .json(ResultResponse {
                error: None,
                message: Some("Student updated successfully".to_string()),
                data: Some(i),
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
