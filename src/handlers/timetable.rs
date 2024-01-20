use crate::models::commen::*;
use actix_web::{
    http::{header::ContentType, StatusCode},
    web::Json as ActJson,
    HttpResponse,
};
use service::*;

type ScBody = ActJson<CTimeTable>;

pub async fn create_time_table(body: ScBody, state: State) -> HttpResponse {
    let res = MutationsService::create_time_table(&state.db_conn, body.into_inner()).await;
    match res {
        Ok(id) => HttpResponse::Created()
            .status(StatusCode::CREATED)
            .content_type(ContentType::json())
            .json(ResponseData {
                error: None,
                message: Some("scan created successfully".to_string()),
                data: Some(id.to_string()),
            }),
        Err(e) => HttpResponse::InternalServerError()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .content_type(ContentType::json())
            .json(ResponseData::<Option<String>> {
                error: Some(e.to_string()),
                message: None,
                data: None,
            }),
    }
}

pub async fn list_time_table(queries: TQueries, body: TFiltersBody, state: State) -> HttpResponse {
    let scans = QueriesService::list_time_table(&state.db_conn).await;

    match scans {
        Ok(i) => HttpResponse::Ok()
            .content_type(ContentType::json())
            .json(ResponseData {
                error: None,
                message: Some("TimeTable selected successfully".to_string()),
                data: Some(i),
            }),
        Err(e) => HttpResponse::InternalServerError()
            .content_type(ContentType::json())
            .json(ResponseData::<Option<String>> {
                error: Some(e.to_string()),
                message: None,
                data: None,
            }),
    }
}
