use crate::models::commen::*;
use actix_web::{web::Json as ActJson, HttpResponse};
use service::{models::CGroup, mutation::*, query::*};
//
type Body = ActJson<CGroup>;

pub async fn create(body: Body, state: State) -> HttpResponse {
    let res = MutationsService::create_group(&state.db_conn, body.into_inner()).await;
    match res {
        Ok(id) => HttpResponse::Ok().json(ResponseData {
            error: None,
            message: Some("Group created successfully".to_string()),
            data: Some(id.to_string()),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ResponseData::<Option<String>> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}

pub async fn delete(id: IdParam, state: State) -> HttpResponse {
    let delete_res = MutationsService::delete_group(&state.db_conn, id.into_inner()).await;

    match delete_res {
        Ok(i) => HttpResponse::Created().json(ResponseData {
            error: None,
            message: Some("Group deleted successfully".to_string()),
            data: Some(i.to_string()),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ResponseData::<Option<String>> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}

pub async fn list_by_level_id(id: IdParam, state: State) -> HttpResponse {
    let selected_group = QueriesService::list_level_groups(&state.db_conn, id.into_inner()).await;

    match selected_group {
        Ok(i) => HttpResponse::Created().json(ResponseData {
            error: None,
            message: Some("Group selected by level id successfully".to_string()),
            data: Some(i),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ResponseData::<Option<String>> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}

pub async fn list(queries: TQueries, body: TFiltersBody, state: State) -> HttpResponse {
    let groups = QueriesService::list_groups(
        &state.db_conn,
        QueriesFilters {
            queries: queries.into_inner(),
            filters: body.clone().filters,
        },
    )
    .await;

    match groups {
        Ok(i) => HttpResponse::Created().json(ResponseData {
            error: None,
            message: Some("Groups selected successfully".to_string()),
            data: Some(i),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ResponseData::<Option<String>> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}

pub async fn update(id: IdParam, body: Body, state: State) -> HttpResponse {
    let update_res =
        MutationsService::update_group(&state.db_conn, id.into_inner(), body.into_inner()).await;
    match update_res {
        Ok(i) => HttpResponse::Created().json(ResponseData {
            error: None,
            message: Some("Group updated successfully".to_string()),
            data: Some(i),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ResponseData::<Option<String>> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}
