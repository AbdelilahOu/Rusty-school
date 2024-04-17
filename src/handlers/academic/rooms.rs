use crate::types::shared::{ResponseData, State};
use actix_web::{
    web::{Json, Path, Query},
    HttpResponse,
};
use service::{
    models::{Room, RoomQuery},
    mutation::MutationService,
    query::QueryService,
    uuid::Uuid,
};
//
pub async fn create(body: Json<Room>, state: State) -> HttpResponse {
    let res = MutationService::create_room(&state.db_conn, body.into_inner()).await;
    match res {
        Ok(id) => HttpResponse::Created().json(ResponseData {
            error: None,
            message: Some("Room created successfully".to_string()),
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
    let res = MutationService::delete_room(&state.db_conn, id.into_inner()).await;
    match res {
        Ok(delete_count) => HttpResponse::Ok().json(ResponseData {
            error: None,
            message: Some("Room deleted successfully".to_string()),
            data: Some(delete_count.to_string()),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ResponseData::<Option<String>> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}

pub async fn list(query: Query<RoomQuery>, state: State) -> HttpResponse {
    let res = QueryService::list_rooms(&state.db_conn, query.into_inner()).await;
    match res {
        Ok(rooms) => HttpResponse::Ok().json(ResponseData {
            error: None,
            message: Some("Rooms selected successfully".to_string()),
            data: Some(rooms),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ResponseData::<Option<String>> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}

pub async fn update(id: Path<Uuid>, body: Json<Room>, state: State) -> HttpResponse {
    let res =
        MutationService::update_room(&state.db_conn, id.into_inner(), body.into_inner()).await;
    match res {
        Ok(id) => HttpResponse::Ok().json(ResponseData {
            error: None,
            message: Some("Room updated successfully".to_string()),
            data: Some(id),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ResponseData::<Option<String>> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}
