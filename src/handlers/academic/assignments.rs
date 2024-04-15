use crate::types::shared::*;
use actix_web::{
    web::{Json, Path},
    HttpResponse,
};
use service::{models::Assignment, mutation::*, query::*, uuid::Uuid};
//
type Body = Json<Assignment>;
pub async fn create(body: Body, state: State) -> HttpResponse {
    let res = MutationsService::create_assignment(&state.db_conn, body.into_inner()).await;
    match res {
        Ok(id) => HttpResponse::Ok().json(ResponseData {
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
    let delete_res = MutationsService::delete_assignment(&state.db_conn, id.into_inner()).await;

    match delete_res {
        Ok(i) => HttpResponse::Created().json(ResponseData {
            error: None,
            message: Some("Assignment deleted successfully".to_string()),
            data: Some(i.to_string()),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ResponseData::<Option<String>> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}

pub async fn list(q: TQueries, body: TFiltersBody, state: State) -> HttpResponse {
    let assignmentes = QueriesService::list_assignments(
        &state.db_conn,
        QueriesFilters {
            queries: q.into_inner(),
            filters: body.clone().filters,
        },
    )
    .await;

    match assignmentes {
        Ok(i) => HttpResponse::Created().json(ResponseData {
            error: None,
            message: Some("Assignments selected successfully".to_string()),
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
        MutationsService::update_assignment(&state.db_conn, id.into_inner(), body.into_inner())
            .await;
    match update_res {
        Ok(i) => HttpResponse::Created().json(ResponseData {
            error: None,
            message: Some("Assignment updated successfully".to_string()),
            data: Some(i),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ResponseData::<Option<String>> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}
