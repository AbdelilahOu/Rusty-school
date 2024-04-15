use crate::models::commen::*;
use actix_web::{http::header::ContentType, HttpResponse};
use service::query::*;
//
pub async fn list(queries: TQueries, body: TFiltersBody, state: State) -> HttpResponse {
    let attendances = QueriesService::list_attendance(
        &state.db_conn,
        QueriesFilters {
            queries: queries.into_inner(),
            filters: body.clone().filters,
        },
    )
    .await;

    match attendances {
        Ok(i) => HttpResponse::Ok()
            .content_type(ContentType::json())
            .json(ResponseData {
                error: None,
                message: Some("Attendance selected successfully".to_string()),
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
