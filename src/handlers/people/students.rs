use crate::{guards::auth::check_token, models::commen::*};
use actix_web::{
    http::{header::ContentType, StatusCode},
    web::Json as ActJson,
    HttpRequest, HttpResponse,
};
use service::{models::CStudent, mutation::*, query::*};
//
type Body = ActJson<CStudent>;

pub async fn create(body: Body, state: State, req: HttpRequest) -> HttpResponse {
    // get headers
    let headers = req.headers();
    // check token for auth
    let authorized = check_token(headers, state.config.jwt_secret.clone());
    // unauth
    if authorized.is_none() {
        return HttpResponse::Unauthorized()
            .status(StatusCode::UNAUTHORIZED)
            .content_type(ContentType::json())
            .json(ResponseData::<Option<String>> {
                error: Some("Unauthorized".to_string()),
                message: None,
                data: None,
            });
    }
    let res = MutationsService::create_student(&state.db_conn, body.into_inner()).await;
    match res {
        Ok(id) => HttpResponse::Ok()
            .status(StatusCode::CREATED)
            .content_type(ContentType::json())
            .json(ResponseData {
                error: None,
                message: Some("Student created successfully".to_string()),
                data: Some(id.to_string()),
            }),
        Err(e) => HttpResponse::InternalServerError()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .content_type(ContentType::json())
            .json(ResponseData::<Option<String>> {
                error: Some(e.to_string()),
                message: None,
                data: None,
            }),
    }
}

pub async fn delete(id: IdParam, state: State) -> HttpResponse {
    let delete_res = MutationsService::delete_student(&state.db_conn, id.into_inner()).await;
    match delete_res {
        Ok(i) => HttpResponse::Created()
            .content_type(ContentType::json())
            .json(ResponseData {
                error: None,
                message: Some("Student deleted successfully".to_string()),
                data: Some(i.to_string()),
            }),
        Err(e) => HttpResponse::InternalServerError()
            .content_type(ContentType::json())
            .json(ResponseData::<Option<String>> {
                error: Some(e.to_string()),
                message: None,
                data: None,
            }),
    }
}

pub async fn list(queries: TQueries, body: TFiltersBody, state: State) -> HttpResponse {
    let students = QueriesService::list_students(
        &state.db_conn,
        QueriesFilters {
            queries: queries.into_inner(),
            filters: body.clone().filters,
        },
    )
    .await;

    match students {
        Ok(i) => HttpResponse::Created()
            .content_type(ContentType::json())
            .json(ResponseData {
                error: None,
                message: Some("Students selected successfully".to_string()),
                data: Some(i),
            }),
        Err(e) => HttpResponse::InternalServerError()
            .content_type(ContentType::json())
            .json(ResponseData::<Option<String>> {
                error: Some(e.to_string()),
                message: None,
                data: None,
            }),
    }
}

pub async fn update(id: IdParam, body: Body, state: State) -> HttpResponse {
    let update_res =
        MutationsService::update_student(&state.db_conn, id.into_inner(), body.into_inner()).await;
    match update_res {
        Ok(i) => HttpResponse::Created()
            .content_type(ContentType::json())
            .json(ResponseData {
                error: None,
                message: Some("Student updated successfully".to_string()),
                data: Some(i),
            }),
        Err(e) => HttpResponse::InternalServerError()
            .content_type(ContentType::json())
            .json(ResponseData::<Option<String>> {
                error: Some(e.to_string()),
                message: None,
                data: None,
            }),
    }
}
