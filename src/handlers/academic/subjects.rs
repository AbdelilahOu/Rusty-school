use crate::{
    guards::auth_guard,
    types::shared::{ResponseData, State},
};
use actix_web::{
    web::{Json, Path, Query},
    HttpRequest as Request, HttpResponse as Response,
};
use service::{
    models::{Subject, SubjectQuery},
    mutation::MutationService,
    query::QueryService,
    uuid::Uuid,
};

//
pub async fn create(req: Request, body: Json<Subject>, state: State) -> Response {
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
    let res = MutationService::create_subject(&state.db_conn, body.into_inner()).await;
    match res {
        Ok(id) => Response::Created().json(ResponseData {
            error: None,
            message: Some("Subject created successfully".to_string()),
            data: Some(id.to_string()),
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
    let res = MutationService::delete_subject(&state.db_conn, id.into_inner()).await;
    match res {
        Ok(delete_count) => Response::Ok().json(ResponseData {
            error: None,
            message: Some("Subject deleted successfully".to_string()),
            data: Some(delete_count.to_string()),
        }),
        Err(e) => Response::InternalServerError().json(ResponseData::<String> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}

pub async fn list_by_level_id(req: Request, id: Path<Uuid>, state: State) -> Response {
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
    let res = QueryService::list_level_subjects(&state.db_conn, id.into_inner()).await;
    match res {
        Ok(subjects) => Response::Ok().json(ResponseData {
            error: None,
            message: Some("Subject selected by level id successfully".to_string()),
            data: Some(subjects),
        }),
        Err(e) => Response::InternalServerError().json(ResponseData::<String> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}

pub async fn list(req: Request, query: Query<SubjectQuery>, state: State) -> Response {
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
    let res = QueryService::list_subjects(&state.db_conn, query.into_inner()).await;
    match res {
        Ok(subjects) => Response::Ok().json(ResponseData {
            error: None,
            message: Some("Subjects selected successfully".to_string()),
            data: Some(subjects),
        }),
        Err(e) => Response::InternalServerError().json(ResponseData::<String> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}

pub async fn update(req: Request, id: Path<Uuid>, body: Json<Subject>, state: State) -> Response {
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
    let res =
        MutationService::update_subject(&state.db_conn, id.into_inner(), body.into_inner()).await;
    match res {
        Ok(id) => Response::Ok().json(ResponseData {
            error: None,
            message: Some("Subject updated successfully".to_string()),
            data: Some(id),
        }),
        Err(e) => Response::InternalServerError().json(ResponseData::<String> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}
