use crate::models::commen::*;
use actix_web::{http::header::ContentType, HttpResponse as HttpRes};
use service::*;
//
pub async fn list_attendance(queries: TQueries, body: TFiltersBody, state: State) -> HttpRes {
    let attendances = QueriesService::list_attendance(
        &state.db_conn,
        QueriesFilters {
            queries: queries.into_inner(),
            filters: body.clone().filters,
        },
    )
    .await;

    match attendances {
        Ok(i) => HttpRes::Created()
            .content_type(ContentType::json())
            .json(ResponseData {
                error: None,
                message: Some("Attendance selected successfully".to_string()),
                data: Some(i),
            }),
        Err(e) => HttpRes::InternalServerError()
            .content_type(ContentType::json())
            .json(ResponseData::<Option<String>> {
                error: Some(e.to_string()),
                message: None,
                data: None,
            }),
    }
}
