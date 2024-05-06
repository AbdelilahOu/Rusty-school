use crate::{
    guards::{auth_guard, role_guard},
    types::shared::{ResponseData, State},
};
use actix_web::{
    web::{Json, Path},
    HttpRequest as Request, HttpResponse as Response,
};
use service::{
    models::{Activity, Event, Lecture},
    mutation::MutationService,
    query::QueryService,
    transaction::TransactionService,
    uuid::Uuid,
};
//
pub async fn create_event(req: Request, body: Json<Event>, state: State) -> Response {
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
            return Response::Unauthorized().json(ResponseData::<String> {
                error: Some("unauthorized role".to_string()),
                message: None,
                data: None,
            });
        }
    }
    let res = TransactionService::create_event(&state.db_conn, body.into_inner()).await;
    match res {
        Ok(_) => Response::Created().json(ResponseData {
            error: None,
            message: Some("time table event created successfully".to_string()),
            data: Some(()),
        }),
        Err(e) => Response::InternalServerError().json(ResponseData::<String> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}

pub async fn create_activity(req: Request, body: Json<Activity>, state: State) -> Response {
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
            return Response::Unauthorized().json(ResponseData::<String> {
                error: Some("unauthorized role".to_string()),
                message: None,
                data: None,
            });
        }
    }
    let res = TransactionService::create_activity(&state.db_conn, body.into_inner()).await;
    match res {
        Ok(_) => Response::Created().json(ResponseData {
            error: None,
            message: Some("time table activity created successfully".to_string()),
            data: Some(()),
        }),
        Err(e) => Response::InternalServerError().json(ResponseData::<String> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}

pub async fn create_lecture(req: Request, body: Json<Lecture>, state: State) -> Response {
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
            return Response::Unauthorized().json(ResponseData::<String> {
                error: Some("unauthorized role".to_string()),
                message: None,
                data: None,
            });
        }
    }
    let res = TransactionService::create_lecture(&state.db_conn, body.into_inner()).await;
    match res {
        Ok(_) => Response::Created().json(ResponseData {
            error: None,
            message: Some("time table lecture created successfully".to_string()),
            data: Some(()),
        }),
        Err(e) => Response::InternalServerError().json(ResponseData::<String> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}

pub async fn list(req: Request, state: State) -> Response {
    let headers = req.headers();
    let authorized = auth_guard(headers, state.config.jwt_secret.clone());
    if let Err(message) = authorized {
        return Response::Unauthorized().json(ResponseData::<String> {
            error: Some(message),
            message: None,
            data: None,
        });
    }
    let res = QueryService::list_time_table(&state.db_conn).await;
    match res {
        Ok(timetable) => Response::Ok().json(ResponseData {
            error: None,
            message: Some("TimeTable selected successfully".to_string()),
            data: Some(timetable),
        }),
        Err(e) => Response::InternalServerError().json(ResponseData::<String> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}

pub async fn delete_timetable_item(req: Request, id: Path<Uuid>, state: State) -> Response {
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
            return Response::Unauthorized().json(ResponseData::<String> {
                error: Some("unauthorized role".to_string()),
                message: None,
                data: None,
            });
        }
    }
    let res = MutationService::delete_time_table(&state.db_conn, id.into_inner()).await;
    match res {
        Ok(i) => Response::Ok().json(ResponseData {
            error: None,
            message: Some("TimeTable item deleted successfully".to_string()),
            data: Some(i),
        }),
        Err(e) => Response::InternalServerError().json(ResponseData::<String> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}
