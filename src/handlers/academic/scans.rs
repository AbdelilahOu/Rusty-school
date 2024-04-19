use crate::{
    guards::auth_guard,
    types::shared::{ResponseData, State},
};
use actix_web::{
    web::{Json, Query},
    HttpRequest as Request, HttpResponse as Response,
};
use service::{
    models::{Scan, ScansQuery},
    mutation::MutationService,
    query::QueryService,
};
//
pub async fn create(req: Request, body: Json<Scan>, state: State) -> Response {
    // get headers
    let headers = req.headers();
    // check token for auth
    let authorized = auth_guard(headers, state.config.jwt_secret.clone());
    // unauth
    if let Err(message) = authorized {
        return Response::Unauthorized().json(ResponseData::<String> {
            error: Some(message),
            message: None,
            data: None,
        });
    }
    let res = MutationService::create_scan(&state.db_conn, body.into_inner()).await;
    match res {
        Ok(id) => Response::Created().json(ResponseData {
            error: None,
            message: Some("scan created successfully".to_string()),
            data: Some(id.to_string()),
        }),
        Err(e) => Response::InternalServerError().json(ResponseData::<String> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}

pub async fn list(req: Request, query: Query<ScansQuery>, state: State) -> Response {
    // get headers
    let headers = req.headers();
    // check token for auth
    let authorized = auth_guard(headers, state.config.jwt_secret.clone());
    // unauth
    if let Err(message) = authorized {
        return Response::Unauthorized().json(ResponseData::<String> {
            error: Some(message),
            message: None,
            data: None,
        });
    }
    let res = QueryService::list_scans(&state.db_conn, query.into_inner()).await;
    match res {
        Ok(scans) => Response::Ok().json(ResponseData {
            error: None,
            message: Some("Scans selected successfully".to_string()),
            data: Some(scans),
        }),
        Err(e) => Response::InternalServerError().json(ResponseData::<String> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}
