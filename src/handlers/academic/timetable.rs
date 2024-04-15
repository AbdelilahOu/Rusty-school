use crate::models::commen::*;
use actix_web::{
    web::{Json, Path},
    HttpResponse,
};
use service::{
    models::{CActivity, CEvent, CLecture},
    mutation::*,
    query::*,
    transaction::*,
    uuid::Uuid,
};

type EventBody = Json<CEvent>;
pub async fn create_event(body: EventBody, state: State) -> HttpResponse {
    let res = TransactionsService::create_event(&state.db_conn, body.into_inner()).await;
    match res {
        Ok(_) => HttpResponse::Created().json(ResponseData {
            error: None,
            message: Some("time table event created successfully".to_string()),
            data: Some(()),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ResponseData::<Option<String>> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}

type ActivityBody = Json<CActivity>;
pub async fn create_activity(body: ActivityBody, state: State) -> HttpResponse {
    let res = TransactionsService::create_activity(&state.db_conn, body.into_inner()).await;
    match res {
        Ok(_) => HttpResponse::Created().json(ResponseData {
            error: None,
            message: Some("time table activity created successfully".to_string()),
            data: Some(()),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ResponseData::<Option<String>> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}

type LectureBody = Json<CLecture>;
pub async fn create_lecture(body: LectureBody, state: State) -> HttpResponse {
    let res = TransactionsService::create_lecture(&state.db_conn, body.into_inner()).await;
    match res {
        Ok(_) => HttpResponse::Created().json(ResponseData {
            error: None,
            message: Some("time table lecture created successfully".to_string()),
            data: Some(()),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ResponseData::<Option<String>> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}

pub async fn list(state: State) -> HttpResponse {
    let timetable = QueriesService::list_time_table(&state.db_conn).await;

    match timetable {
        Ok(i) => HttpResponse::Ok().json(ResponseData {
            error: None,
            message: Some("TimeTable selected successfully".to_string()),
            data: Some(i),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ResponseData::<Option<String>> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}

pub async fn delete_timetable_item(id: Path<Uuid>, state: State) -> HttpResponse {
    let res = MutationsService::delete_time_table(&state.db_conn, id.into_inner()).await;

    match res {
        Ok(i) => HttpResponse::Ok().json(ResponseData {
            error: None,
            message: Some("TimeTable item deleted successfully".to_string()),
            data: Some(i),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ResponseData::<Option<String>> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}
