use crate::types::shared::{ResponseData, State};
use actix_web::{
    web::{Json, Query},
    HttpResponse,
};
use service::{
    models::{Scan, ScansQuery},
    mutation::MutationService,
    query::QueryService,
};

type Body = Json<Scan>;
pub async fn create(body: Body, state: State) -> HttpResponse {
    let res = MutationService::create_scan(&state.db_conn, body.into_inner()).await;
    match res {
        Ok(id) => HttpResponse::Created().json(ResponseData {
            error: None,
            message: Some("scan created successfully".to_string()),
            data: Some(id.to_string()),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ResponseData::<Option<String>> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}

pub async fn list(q: Query<ScansQuery>, state: State) -> HttpResponse {
    let scans = QueryService::list_scans(&state.db_conn, q.into_inner()).await;
    match scans {
        Ok(i) => HttpResponse::Ok().json(ResponseData {
            error: None,
            message: Some("Scans selected successfully".to_string()),
            data: Some(i),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ResponseData::<Option<String>> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}
