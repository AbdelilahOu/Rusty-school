use crate::types::shared::{ResponseData, State};
use actix_web::{
    web::{Json, Path, Query},
    HttpResponse,
};
use service::{
    models::{Parent, ParentQuery},
    mutation::MutationService,
    query::QueryService,
    uuid::Uuid,
};

pub async fn create(body: Json<Parent>, state: State) -> HttpResponse {
    let res = MutationService::create_parent(&state.db_conn, body.into_inner()).await;
    match res {
        Ok(id) => HttpResponse::Created().json(ResponseData {
            error: None,
            message: Some("Parent created successfully".to_string()),
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
    let res = MutationService::delete_parent(&state.db_conn, id.into_inner()).await;
    match res {
        Ok(delete_count) => HttpResponse::Ok().json(ResponseData {
            error: None,
            message: Some("Parent deleted successfully".to_string()),
            data: Some(delete_count.to_string()),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ResponseData::<Option<String>> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}

pub async fn list(query: Query<ParentQuery>, state: State) -> HttpResponse {
    let res = QueryService::list_parents(&state.db_conn, query.into_inner()).await;
    match res {
        Ok(parents) => HttpResponse::Ok().json(ResponseData {
            error: None,
            message: Some("Parents selected successfully".to_string()),
            data: Some(parents),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ResponseData::<Option<String>> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}

pub async fn update(id: Path<Uuid>, body: Json<Parent>, state: State) -> HttpResponse {
    let res =
        MutationService::update_parent(&state.db_conn, id.into_inner(), body.into_inner()).await;
    match res {
        Ok(id) => HttpResponse::Ok().json(ResponseData {
            error: None,
            message: Some("Parent updated successfully".to_string()),
            data: Some(id),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ResponseData::<Option<String>> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}
