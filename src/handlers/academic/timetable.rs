use crate::types::shared::{ResponseData, State};
use actix_web::{
    web::{Json, Path},
    HttpResponse as Response,
};
use service::{
    models::{Activity, Event, Lecture},
    mutation::MutationService,
    query::QueryService,
    transaction::TransactionService,
    uuid::Uuid,
};
//
pub async fn create_event(body: Json<Event>, state: State) -> Response {
    let res = TransactionService::create_event(&state.db_conn, body.into_inner()).await;
    match res {
        Ok(_) => Response::Created().json(ResponseData {
            error: None,
            message: Some("time table event created successfully".to_string()),
            data: Some(()),
        }),
        Err(e) => Response::InternalServerError().json(ResponseData::<String> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}

pub async fn create_activity(body: Json<Activity>, state: State) -> Response {
    let res = TransactionService::create_activity(&state.db_conn, body.into_inner()).await;
    match res {
        Ok(_) => Response::Created().json(ResponseData {
            error: None,
            message: Some("time table activity created successfully".to_string()),
            data: Some(()),
        }),
        Err(e) => Response::InternalServerError().json(ResponseData::<String> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}

pub async fn create_lecture(body: Json<Lecture>, state: State) -> Response {
    let res = TransactionService::create_lecture(&state.db_conn, body.into_inner()).await;
    match res {
        Ok(_) => Response::Created().json(ResponseData {
            error: None,
            message: Some("time table lecture created successfully".to_string()),
            data: Some(()),
        }),
        Err(e) => Response::InternalServerError().json(ResponseData::<String> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}

pub async fn list(state: State) -> Response {
    let res = QueryService::list_time_table(&state.db_conn).await;
    match res {
        Ok(timetable) => Response::Ok().json(ResponseData {
            error: None,
            message: Some("TimeTable selected successfully".to_string()),
            data: Some(timetable),
        }),
        Err(e) => Response::InternalServerError().json(ResponseData::<String> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}

pub async fn delete_timetable_item(id: Path<Uuid>, state: State) -> Response {
    let res = MutationService::delete_time_table(&state.db_conn, id.into_inner()).await;
    match res {
        Ok(i) => Response::Ok().json(ResponseData {
            error: None,
            message: Some("TimeTable item deleted successfully".to_string()),
            data: Some(i),
        }),
        Err(e) => Response::InternalServerError().json(ResponseData::<String> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}
