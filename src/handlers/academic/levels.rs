use crate::types::shared::*;
use actix_web::{
    web::{Json, Path, Query},
    HttpResponse,
};
use service::{
    models::{Level, LevelQueries},
    mutation::*,
    query::*,
    uuid::Uuid,
};

// i like my functions to stay inline
type Body = Json<Level>;
pub async fn create(body: Body, state: State) -> HttpResponse {
    let res = MutationsService::create_level(&state.db_conn, body.into_inner()).await;
    match res {
        Ok(id) => HttpResponse::Ok().json(ResponseData {
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
    let delete_res = MutationsService::delete_level(&state.db_conn, id.into_inner()).await;
    match delete_res {
        Ok(i) => HttpResponse::Created().json(ResponseData {
            error: None,
            message: Some("Level deleted successfully".to_string()),
            data: Some(i.to_string()),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ResponseData::<Option<String>> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}

pub async fn list(q: Query<LevelQueries>, state: State) -> HttpResponse {
    let levels = QueriesService::list_levels(&state.db_conn, q.into_inner()).await;
    match levels {
        Ok(i) => HttpResponse::Created().json(ResponseData {
            error: None,
            message: Some("Levels selected successfully".to_string()),
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
        MutationsService::update_level(&state.db_conn, id.into_inner(), body.into_inner()).await;
    match update_res {
        Ok(i) => HttpResponse::Created().json(ResponseData {
            error: None,
            message: Some("Level updated successfully".to_string()),
            data: Some(i),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ResponseData::<Option<String>> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}
