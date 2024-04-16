use crate::types::shared::{ResponseData, State};
use actix_web::{web::Query, HttpResponse};
use service::{models::AttendanceQuery, query::QueryService};
//
pub async fn list(q: Query<AttendanceQuery>, state: State) -> HttpResponse {
    let attendances = QueryService::list_attendance(&state.db_conn, q.into_inner()).await;
    match attendances {
        Ok(i) => HttpResponse::Ok().json(ResponseData {
            error: None,
            message: Some("Attendance selected successfully".to_string()),
            data: Some(i),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ResponseData::<Option<String>> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}
