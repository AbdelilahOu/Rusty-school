use actix_web::{
    web::{Json, Path, Query},
    HttpRequest as Request, HttpResponse as Response,
};

use service::{
    models::{Level, LevelQuery},
    mutation::MutationService,
    query::QueryService,
    uuid::Uuid,
};

use crate::{
    guards::{auth_guard, role_guard},
    types::shared::{ResponseData, State},
};

pub async fn create(req: Request, body: Json<Level>, state: State) -> Response {
    let headers = req.headers();
    let authorized = auth_guard(headers, state.config.jwt_secret.clone());
    if let Err(message) = authorized {
        return Response::Unauthorized().json(ResponseData::<String> {
            error: Some(message),
            message: None,
            data: None,
        });
    }
    if let Ok(claims) = authorized {
        if !role_guard(claims.role, vec!["admin", "assistant"]) {
            return Response::Forbidden().json(ResponseData::<String> {
                error: Some("unauthorized role".to_string()),
                message: None,
                data: None,
            });
        }
    }
    let res = MutationService::create_level(&state.db_conn, body.into_inner()).await;
    match res {
        Ok(id) => Response::Created().json(ResponseData {
            error: None,
            message: Some("Level created successfully".to_string()),
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
    let headers = req.headers();
    let authorized = auth_guard(headers, state.config.jwt_secret.clone());
    if let Err(message) = authorized {
        return Response::Unauthorized().json(ResponseData::<String> {
            error: Some(message),
            message: None,
            data: None,
        });
    }
    if let Ok(claims) = authorized {
        if !role_guard(claims.role, vec!["admin", "assistant"]) {
            return Response::Forbidden().json(ResponseData::<String> {
                error: Some("unauthorized role".to_string()),
                message: None,
                data: None,
            });
        }
    }
    let res = MutationService::delete_level(&state.db_conn, id.into_inner()).await;
    match res {
        Ok(delete_count) => Response::Ok().json(ResponseData {
            error: None,
            message: Some("Level deleted successfully".to_string()),
            data: Some(delete_count),
        }),
        Err(e) => Response::InternalServerError().json(ResponseData::<String> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}

pub async fn list(req: Request, query: Query<LevelQuery>, state: State) -> Response {
    let headers = req.headers();
    let authorized = auth_guard(headers, state.config.jwt_secret.clone());
    if let Err(message) = authorized {
        return Response::Unauthorized().json(ResponseData::<String> {
            error: Some(message),
            message: None,
            data: None,
        });
    }
    if let Ok(claims) = authorized {
        if !role_guard(claims.role, vec!["admin", "assistant"]) {
            return Response::Forbidden().json(ResponseData::<String> {
                error: Some("unauthorized role".to_string()),
                message: None,
                data: None,
            });
        }
    }
    let res = QueryService::list_levels(&state.db_conn, query.into_inner()).await;
    match res {
        Ok(levels) => Response::Ok().json(ResponseData {
            error: None,
            message: Some("Levels selected successfully".to_string()),
            data: Some(levels),
        }),
        Err(e) => Response::InternalServerError().json(ResponseData::<String> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}

pub async fn update(req: Request, id: Path<Uuid>, body: Json<Level>, state: State) -> Response {
    let headers = req.headers();
    let authorized = auth_guard(headers, state.config.jwt_secret.clone());
    if let Err(message) = authorized {
        return Response::Unauthorized().json(ResponseData::<String> {
            error: Some(message),
            message: None,
            data: None,
        });
    }
    if let Ok(claims) = authorized {
        if !role_guard(claims.role, vec!["admin", "assistant"]) {
            return Response::Forbidden().json(ResponseData::<String> {
                error: Some("unauthorized role".to_string()),
                message: None,
                data: None,
            });
        }
    }
    let res =
        MutationService::update_level(&state.db_conn, id.into_inner(), body.into_inner()).await;
    match res {
        Ok(id) => Response::Ok().json(ResponseData {
            error: None,
            message: Some("Level updated successfully".to_string()),
            data: Some(id),
        }),
        Err(e) => Response::InternalServerError().json(ResponseData::<String> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}
