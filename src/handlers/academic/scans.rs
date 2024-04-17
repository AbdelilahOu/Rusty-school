use crate::types::shared::{ResponseData, State};
use actix_web::{
    web::{Json, Query},
    HttpResponse as Response,
};
use service::{
    models::{Scan, ScansQuery},
    mutation::MutationService,
    query::QueryService,
};
//
pub async fn create(body: Json<Scan>, state: State) -> Response {
    let res = MutationService::create_scan(&state.db_conn, body.into_inner()).await;
    match res {
        Ok(id) => Response::Created().json(ResponseData {
            error: None,
            message: Some("scan created successfully".to_string()),
            data: Some(id.to_string()),
        }),
        Err(e) => Response::InternalServerError().json(ResponseData::<Option<String>> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}

pub async fn list(query: Query<ScansQuery>, state: State) -> Response {
    let res = QueryService::list_scans(&state.db_conn, query.into_inner()).await;
    match res {
        Ok(scans) => Response::Ok().json(ResponseData {
            error: None,
            message: Some("Scans selected successfully".to_string()),
            data: Some(scans),
        }),
        Err(e) => Response::InternalServerError().json(ResponseData::<Option<String>> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}
