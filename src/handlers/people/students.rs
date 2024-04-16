use crate::{
    guards::auth_guard,
    types::shared::{ResponseData, State},
};
use actix_web::{
    web::{Json, Path, Query},
    HttpRequest, HttpResponse,
};
use service::{
    models::{Student, StudentQueries},
    mutation::MutationsService,
    query::QueriesService,
    uuid::Uuid,
};
//
type Body = Json<Student>;
pub async fn create(body: Body, state: State, req: HttpRequest) -> HttpResponse {
    // get headers
    let headers = req.headers();
    // check token for auth
    let authorized = auth_guard(headers, state.config.jwt_secret.clone());
    // unauth
    if let Err(message) = authorized {
        return HttpResponse::Unauthorized().json(ResponseData::<Option<String>> {
            error: Some(message),
            message: None,
            data: None,
        });
    }
    let res = MutationsService::create_student(&state.db_conn, body.into_inner()).await;
    match res {
        Ok(id) => HttpResponse::Ok().json(ResponseData {
            error: None,
            message: Some("Student created successfully".to_string()),
            data: Some(id.to_string()),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ResponseData::<Option<String>> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}

pub async fn delete(id: Path<Uuid>, state: State, req: HttpRequest) -> HttpResponse {
    // get headers
    let headers = req.headers();
    // check token for auth
    let authorized = auth_guard(headers, state.config.jwt_secret.clone());
    // unauth
    if let Err(message) = authorized {
        return HttpResponse::Unauthorized().json(ResponseData::<Option<String>> {
            error: Some(message),
            message: None,
            data: None,
        });
    }

    let delete_res = MutationsService::delete_student(&state.db_conn, id.into_inner()).await;
    match delete_res {
        Ok(i) => HttpResponse::Ok().json(ResponseData {
            error: None,
            message: Some("Student deleted successfully".to_string()),
            data: Some(i.to_string()),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ResponseData::<Option<String>> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}

pub async fn list(q: Query<StudentQueries>, state: State, req: HttpRequest) -> HttpResponse {
    // get headers
    let headers = req.headers();
    // check token for auth
    let authorized = auth_guard(headers, state.config.jwt_secret.clone());
    // unauth
    if let Err(message) = authorized {
        return HttpResponse::Unauthorized().json(ResponseData::<Option<String>> {
            error: Some(message),
            message: None,
            data: None,
        });
    }
    let students = QueriesService::list_students(&state.db_conn, q.into_inner()).await;
    match students {
        Ok(i) => HttpResponse::Ok().json(ResponseData {
            error: None,
            message: Some("Students selected successfully".to_string()),
            data: Some(i),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ResponseData::<Option<String>> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}

pub async fn update(id: Path<Uuid>, body: Body, state: State, req: HttpRequest) -> HttpResponse {
    // get headers
    let headers = req.headers();
    // check token for auth
    let authorized = auth_guard(headers, state.config.jwt_secret.clone());
    // unauth
    if let Err(message) = authorized {
        return HttpResponse::Unauthorized().json(ResponseData::<Option<String>> {
            error: Some(message),
            message: None,
            data: None,
        });
    }

    let update_res =
        MutationsService::update_student(&state.db_conn, id.into_inner(), body.into_inner()).await;
    match update_res {
        Ok(i) => HttpResponse::Ok().json(ResponseData {
            error: None,
            message: Some("Student updated successfully".to_string()),
            data: Some(i),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ResponseData::<Option<String>> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}
