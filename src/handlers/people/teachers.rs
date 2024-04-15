use crate::types::shared::*;
use actix_web::{
    web::{Json, Path},
    HttpResponse,
};
use service::{models::CTeacher, mutation::*, query::*, uuid::Uuid};

type Body = Json<CTeacher>;
pub async fn create(body: Body, state: State) -> HttpResponse {
    let res = MutationsService::create_teacher(&state.db_conn, body.into_inner()).await;
    match res {
        Ok(id) => HttpResponse::Ok().json(ResponseData {
            error: None,
            message: Some("Teacher created successfully".to_string()),
            data: Some(id.to_string()),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ResponseData::<Option<String>> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}

pub async fn add_subject(params: Path<(Uuid, Uuid)>, state: State) -> HttpResponse {
    let res = MutationsService::create_teacher_subject(&state.db_conn, params.into_inner()).await;
    match res {
        Ok(i) => HttpResponse::Created().json(ResponseData {
            error: None,
            message: Some("Subject added successfully".to_string()),
            data: Some(i.to_string()),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ResponseData::<Option<String>> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}

pub async fn delete_subject(id: Path<Uuid>, state: State) -> HttpResponse {
    let res = MutationsService::delete_teacher_subject(&state.db_conn, id.into_inner()).await;
    match res {
        Ok(i) => HttpResponse::Created().json(ResponseData {
            error: None,
            message: Some("Subject deleted successfully".to_string()),
            data: Some(i.to_string()),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ResponseData::<Option<String>> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}

pub async fn delete(id: Path<Uuid>, state: State) -> HttpResponse {
    let delete_res = MutationsService::delete_teacher(&state.db_conn, id.into_inner()).await;

    match delete_res {
        Ok(i) => HttpResponse::Created().json(ResponseData {
            error: None,
            message: Some("Teacher deleted successfully".to_string()),
            data: Some(i.to_string()),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ResponseData::<Option<String>> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}

pub async fn list(queries: TQueries, body: TFiltersBody, state: State) -> HttpResponse {
    let teachers = QueriesService::list_teachers(
        &state.db_conn,
        QueriesFilters {
            queries: queries.into_inner(),
            filters: body.clone().filters,
        },
    )
    .await;

    match teachers {
        Ok(i) => HttpResponse::Created().json(ResponseData {
            error: None,
            message: Some("Teachers selected successfully".to_string()),
            data: Some(i),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ResponseData::<Option<String>> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}

pub async fn update(id: Path<Uuid>, body: Body, state: State) -> HttpResponse {
    let update_res =
        MutationsService::update_teacher(&state.db_conn, id.into_inner(), body.into_inner()).await;
    match update_res {
        Ok(i) => HttpResponse::Created().json(ResponseData {
            error: None,
            message: Some("Teacher updated successfully".to_string()),
            data: Some(i),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ResponseData::<Option<String>> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}
