use crate::types::shared::{ResponseData, State};
use actix_web::{
    web::{Json, Path, Query},
    HttpResponse,
};
use service::{
    models::{Level, LevelQuery},
    mutation::MutationService,
    query::QueryService,
    uuid::Uuid,
};

pub async fn create(body: Json<Level>, state: State) -> HttpResponse {
    let res = MutationService::create_level(&state.db_conn, body.into_inner()).await;
    match res {
        Ok(id) => HttpResponse::Created().json(ResponseData {
            error: None,
            message: Some("Level created successfully".to_string()),
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
    let res = MutationService::delete_level(&state.db_conn, id.into_inner()).await;
    match res {
        Ok(delete_count) => HttpResponse::Ok().json(ResponseData {
            error: None,
            message: Some("Level deleted successfully".to_string()),
            data: Some(delete_count.to_string()),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ResponseData::<Option<String>> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}

pub async fn list(query: Query<LevelQuery>, state: State) -> HttpResponse {
    let res = QueryService::list_levels(&state.db_conn, query.into_inner()).await;
    match res {
        Ok(levels) => HttpResponse::Ok().json(ResponseData {
            error: None,
            message: Some("Levels selected successfully".to_string()),
            data: Some(levels),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ResponseData::<Option<String>> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}

pub async fn update(id: Path<Uuid>, body: Json<Level>, state: State) -> HttpResponse {
    let res =
        MutationService::update_level(&state.db_conn, id.into_inner(), body.into_inner()).await;
    match res {
        Ok(id) => HttpResponse::Ok().json(ResponseData {
            error: None,
            message: Some("Level updated successfully".to_string()),
            data: Some(id),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ResponseData::<Option<String>> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}
