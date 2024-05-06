use crate::{
    guards::auth_guard,
    types::shared::{ResponseData, State},
};
use actix_web::{web::Query, HttpRequest as Request, HttpResponse as Response};
use service::{models::AttendanceQuery, query::QueryService};
//
pub async fn list(req: Request, query: Query<AttendanceQuery>, state: State) -> Response {
    let headers = req.headers();
    let authorized = auth_guard(headers, state.config.jwt_secret.clone());
    if let Err(message) = authorized {
        return Response::Unauthorized().json(ResponseData::<String> {
            error: Some(message),
            message: None,
            data: None,
        });
    }
    let res = QueryService::list_attendance(&state.db_conn, query.into_inner()).await;
    match res {
        Ok(attendances) => Response::Ok().json(ResponseData {
            error: None,
            message: Some("Attendance selected successfully".to_string()),
            data: Some(attendances),
        }),
        Err(e) => Response::InternalServerError().json(ResponseData::<String> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}
