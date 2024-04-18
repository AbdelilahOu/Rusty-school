use crate::{
    guards::auth_guard,
    types::shared::{ResponseData, State},
};
use actix_web::{
    web::{Json, Path, Query},
    HttpRequest as Request, HttpResponse as Response,
};
use service::{
    models::{Student, StudentQuery},
    mutation::MutationService,
    query::QueryService,
    uuid::Uuid,
};
//
pub async fn create(body: Json<Student>, state: State, req: Request) -> Response {
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
    let res = MutationService::create_student(&state.db_conn, body.into_inner()).await;
    match res {
        Ok(id) => Response::Ok().json(ResponseData {
            error: None,
            message: Some("Student created successfully".to_string()),
            data: Some(id.to_string()),
        }),
        Err(e) => Response::InternalServerError().json(ResponseData::<String> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}

pub async fn delete(id: Path<Uuid>, state: State, req: Request) -> Response {
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

    let res = MutationService::delete_student(&state.db_conn, id.into_inner()).await;
    match res {
        Ok(delete_count) => Response::Ok().json(ResponseData {
            error: None,
            message: Some("Student deleted successfully".to_string()),
            data: Some(delete_count.to_string()),
        }),
        Err(e) => Response::InternalServerError().json(ResponseData::<String> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}

pub async fn list(query: Query<StudentQuery>, state: State, req: Request) -> Response {
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
    let res = QueryService::list_students(&state.db_conn, query.into_inner()).await;
    match res {
        Ok(students) => Response::Ok().json(ResponseData {
            error: None,
            message: Some("Students selected successfully".to_string()),
            data: Some(students),
        }),
        Err(e) => Response::InternalServerError().json(ResponseData::<String> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}

pub async fn update(id: Path<Uuid>, body: Json<Student>, state: State, req: Request) -> Response {
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
        MutationService::update_student(&state.db_conn, id.into_inner(), body.into_inner()).await;
    match res {
        Ok(id) => Response::Ok().json(ResponseData {
            error: None,
            message: Some("Student updated successfully".to_string()),
            data: Some(id),
        }),
        Err(e) => Response::InternalServerError().json(ResponseData::<String> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}
