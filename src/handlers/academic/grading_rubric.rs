use crate::{
    guards::auth_guard,
    types::shared::{ResponseData, State},
};
use actix_web::{
    web::{Json, Path, Query},
    HttpRequest as Request, HttpResponse as Response,
};
use service::{
    models::{Rubric, RubricQuery},
    mutation::MutationService,
    query::QueryService,
    transaction::TransactionService,
    uuid::Uuid,
};
//
pub async fn create(req: Request, body: Json<Rubric>, state: State) -> Response {
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
    let res = TransactionService::create_rubric(&state.db_conn, body.into_inner()).await;
    match res {
        Ok(id) => Response::Created().json(ResponseData {
            error: None,
            message: Some("grading rubric created successfully".to_string()),
            data: Some(id.to_string()),
        }),
        Err(e) => Response::InternalServerError().json(ResponseData::<String> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}

pub async fn list(req: Request, query: Query<RubricQuery>, state: State) -> Response {
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
    let res = QueryService::list_rubrics(&state.db_conn, query.into_inner()).await;
    match res {
        Ok(rubrics) => Response::Ok().json(ResponseData {
            error: None,
            message: Some("grading rubrics selected successfully".to_string()),
            data: Some(rubrics),
        }),
        Err(e) => Response::InternalServerError().json(ResponseData::<String> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}

pub async fn delete(req: Request, id: Path<Uuid>, state: State) -> Response {
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
    let res = MutationService::delete_rubric(&state.db_conn, id.into_inner()).await;
    match res {
        Ok(delete_count) => Response::Ok().json(ResponseData {
            error: None,
            message: Some("grading rubric deleted successfully".to_string()),
            data: Some(delete_count),
        }),
        Err(e) => Response::InternalServerError().json(ResponseData::<String> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}

pub async fn update(req: Request, id: Path<Uuid>, body: Json<Rubric>, state: State) -> Response {
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
    let res = MutationService::update_rubric(&state.db_conn, id.into_inner(), body.into_inner()).await;
    match res {
        Ok(id) => Response::Ok().json(ResponseData {
            error: None,
            message: Some("grading rubric updated successfully".to_string()),
            data: Some(id),
        }),
        Err(e) => Response::InternalServerError().json(ResponseData::<String> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}
