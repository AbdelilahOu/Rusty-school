use crate::types::shared::{ResponseData, State};
use actix_web::{web::Query, HttpResponse};
use service::{models::AttendanceQuery, query::QueryService};
//
pub async fn list(query: Query<AttendanceQuery>, state: State) -> HttpResponse {
    let res = QueryService::list_attendance(&state.db_conn, query.into_inner()).await;
    match res {
        Ok(attendances) => HttpResponse::Ok().json(ResponseData {
            error: None,
            message: Some("Attendance selected successfully".to_string()),
            data: Some(attendances),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ResponseData::<Option<String>> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}
