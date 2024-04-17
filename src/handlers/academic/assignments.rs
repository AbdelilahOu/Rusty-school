use crate::types::shared::{ResponseData, State};
use actix_web::{
    web::{Json, Path, Query},
    HttpResponse,
};
use service::{
    models::{Assignment, AssignmentQuery},
    mutation::MutationService,
    query::QueryService,
    uuid::Uuid,
};
//
// type Body = ;
pub async fn create(body: Json<Assignment>, state: State) -> HttpResponse {
    let res = MutationService::create_assignment(&state.db_conn, body.into_inner()).await;
    match res {
        Ok(id) => HttpResponse::Created().json(ResponseData {
            error: None,
            message: Some("Assignment created successfully".to_string()),
            data: Some(id.to_string()),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ResponseData::<Option<String>> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}

pub async fn delete(id: Path<Uuid>, state: State) -> HttpResponse {
    let res = MutationService::delete_assignment(&state.db_conn, id.into_inner()).await;
    match res {
        Ok(delete_count) => HttpResponse::Ok().json(ResponseData {
            error: None,
            message: Some("Assignment deleted successfully".to_string()),
            data: Some(delete_count.to_string()),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ResponseData::<Option<String>> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}

pub async fn list(query: Query<AssignmentQuery>, state: State) -> HttpResponse {
    let res = QueryService::list_assignments(&state.db_conn, query.into_inner()).await;
    match res {
        Ok(assignmentes) => HttpResponse::Ok().json(ResponseData {
            error: None,
            message: Some("Assignments selected successfully".to_string()),
            data: Some(assignmentes),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ResponseData::<Option<String>> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}

pub async fn update(id: Path<Uuid>, body: Json<Assignment>, state: State) -> HttpResponse {
    let res =
        MutationService::update_assignment(&state.db_conn, id.into_inner(), body.into_inner())
            .await;
    match res {
        Ok(id) => HttpResponse::Ok().json(ResponseData {
            error: None,
            message: Some("Assignment updated successfully".to_string()),
            data: Some(id),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ResponseData::<Option<String>> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}
