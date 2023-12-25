use crate::{guards::auth::check_token, models::commen::*};
use actix_web::{
    http::{header::ContentType, StatusCode},
    web::Json as ActJson,
    HttpRequest as HttpReq, HttpResponse as HttpRes,
};
use service::*;
//
type StBody = ActJson<CSubject>;

pub async fn create_subject(body: StBody, state: State, req: HttpReq) -> HttpRes {
    // get headers
    let headers = req.headers();
    // check token for auth
    let authorized = check_token(headers, state.env.jwt_secret.clone());
    // unauth
    if authorized.is_none() {
        return HttpRes::Unauthorized()
            .status(StatusCode::UNAUTHORIZED)
            .content_type(ContentType::json())
            .json(ResponseData::<Option<String>> {
                error: Some("Unauthorized".to_string()),
                message: None,
                data: None,
            });
    }
    println!("{:?}", authorized);
    let res = ServiceMutation::create_subject(&state.db_conn, body.into_inner()).await;
    match res {
        Ok(id) => HttpRes::Ok()
            .status(StatusCode::CREATED)
            .content_type(ContentType::json())
            .json(ResponseData {
                error: None,
                message: Some("Subject created successfully".to_string()),
                data: Some(id.to_string()),
            }),
        Err(e) => HttpRes::InternalServerError()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .content_type(ContentType::json())
            .json(ResponseData::<Option<String>> {
                error: Some(e.to_string()),
                message: None,
                data: None,
            }),
    }
}

pub async fn delete_subject(id: IdParam, state: State) -> HttpRes {
    let delete_res = ServiceMutation::delete_subject(&state.db_conn, id.into_inner()).await;

    match delete_res {
        Ok(i) => HttpRes::Created()
            .content_type(ContentType::json())
            .json(ResponseData {
                error: None,
                message: Some("Subject deleted successfully".to_string()),
                data: Some(i.to_string()),
            }),
        Err(e) => HttpRes::InternalServerError()
            .content_type(ContentType::json())
            .json(ResponseData::<Option<String>> {
                error: Some(e.to_string()),
                message: None,
                data: None,
            }),
    }
}

pub async fn get_subject(id: IdParam, state: State) -> HttpRes {
    let selected_subject = ServiceQuery::get_subject(&state.db_conn, id.into_inner()).await;

    match selected_subject {
        Ok(i) => HttpRes::Created()
            .content_type(ContentType::json())
            .json(ResponseData {
                error: None,
                message: Some("Subject selected successfully".to_string()),
                data: Some(i),
            }),
        Err(e) => HttpRes::InternalServerError()
            .content_type(ContentType::json())
            .json(ResponseData::<Option<String>> {
                error: Some(e.to_string()),
                message: None,
                data: None,
            }),
    }
}

pub async fn list_subjects(queries: TQueries, body: TFiltersBody, state: State) -> HttpRes {
    let subjects = ServiceQuery::list_subjects(
        &state.db_conn,
        QueriesFilters {
            queries: queries.into_inner(),
            filters: body.clone().filters,
        },
    )
    .await;

    match subjects {
        Ok(i) => HttpRes::Created()
            .content_type(ContentType::json())
            .json(ResponseData {
                error: None,
                message: Some("Subjects selected successfully".to_string()),
                data: Some(i),
            }),
        Err(e) => HttpRes::InternalServerError()
            .content_type(ContentType::json())
            .json(ResponseData::<Option<String>> {
                error: Some(e.to_string()),
                message: None,
                data: None,
            }),
    }
}

pub async fn update_subject(id: IdParam, body: StBody, state: State) -> HttpRes {
    let update_res =
        ServiceMutation::update_subject(&state.db_conn, id.into_inner(), body.into_inner()).await;
    match update_res {
        Ok(i) => HttpRes::Created()
            .content_type(ContentType::json())
            .json(ResponseData {
                error: None,
                message: Some("Subject updated successfully".to_string()),
                data: Some(i),
            }),
        Err(e) => HttpRes::InternalServerError()
            .content_type(ContentType::json())
            .json(ResponseData::<Option<String>> {
                error: Some(e.to_string()),
                message: None,
                data: None,
            }),
    }
}
