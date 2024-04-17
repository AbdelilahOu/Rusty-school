use crate::types::shared::{ResponseData, State};
use actix_web::{
    web::{Json, Path, Query},
    HttpResponse,
};
use service::{
    models::{Announcement, AnnouncementQuery},
    mutation::MutationService,
    query::QueryService,
    uuid::Uuid,
};

pub async fn create(body: Json<Announcement>, state: State) -> HttpResponse {
    let res = MutationService::create_announcement(&state.db_conn, body.into_inner()).await;
    match res {
        Ok(id) => HttpResponse::Created().json(ResponseData {
            error: None,
            message: Some("Announcements created successfully".to_string()),
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
    let res = MutationService::delete_announcement(&state.db_conn, id.into_inner()).await;
    match res {
        Ok(delete_count) => HttpResponse::Ok().json(ResponseData {
            error: None,
            message: Some("Announcements deleted successfully".to_string()),
            data: Some(delete_count.to_string()),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ResponseData::<Option<String>> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}

pub async fn list(query: Query<AnnouncementQuery>, state: State) -> HttpResponse {
    let res = QueryService::list_announcements(&state.db_conn, query.into_inner()).await;
    match res {
        Ok(announcements) => HttpResponse::Ok().json(ResponseData {
            error: None,
            message: Some("Announcementss selected successfully".to_string()),
            data: Some(announcements),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ResponseData::<Option<String>> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}

pub async fn update(id: Path<Uuid>, body: Json<Announcement>, state: State) -> HttpResponse {
    let res =
        MutationService::update_announcement(&state.db_conn, id.into_inner(), body.into_inner())
            .await;
    match res {
        Ok(id) => HttpResponse::Ok().json(ResponseData {
            error: None,
            message: Some("Announcements updated successfully".to_string()),
            data: Some(id),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ResponseData::<Option<String>> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}
