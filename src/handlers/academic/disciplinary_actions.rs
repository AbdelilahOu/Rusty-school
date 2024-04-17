use crate::types::shared::{ResponseData, State};
use actix_web::{
    web::{Json, Path, Query},
    HttpResponse,
};
use service::{
    models::{Disciplinary, DisciplinaryQuery},
    mutation::MutationService,
    query::QueryService,
    uuid::Uuid,
};
//
pub async fn create(body: Json<Disciplinary>, state: State) -> HttpResponse {
    let res = MutationService::create_disciplinary(&state.db_conn, body.into_inner()).await;
    match res {
        Ok(id) => HttpResponse::Created().json(ResponseData {
            error: None,
            message: Some("Disciplinary created successfully".to_string()),
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
    let res = MutationService::delete_disciplinary(&state.db_conn, id.into_inner()).await;
    match res {
        Ok(delete_count) => HttpResponse::Ok().json(ResponseData {
            error: None,
            message: Some("Disciplinary deleted successfully".to_string()),
            data: Some(delete_count.to_string()),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ResponseData::<Option<String>> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}

pub async fn list(query: Query<DisciplinaryQuery>, state: State) -> HttpResponse {
    let res = QueryService::list_disciplinaries(&state.db_conn, query.into_inner()).await;
    match res {
        Ok(disciplinary_actions) => HttpResponse::Ok().json(ResponseData {
            error: None,
            message: Some("Disciplinarys selected successfully".to_string()),
            data: Some(disciplinary_actions),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ResponseData::<Option<String>> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}

pub async fn update(id: Path<Uuid>, body: Json<Disciplinary>, state: State) -> HttpResponse {
    let res =
        MutationService::update_disciplinary(&state.db_conn, id.into_inner(), body.into_inner())
            .await;
    match res {
        Ok(id) => HttpResponse::Ok().json(ResponseData {
            error: None,
            message: Some("Disciplinary updated successfully".to_string()),
            data: Some(id),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ResponseData::<Option<String>> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}
