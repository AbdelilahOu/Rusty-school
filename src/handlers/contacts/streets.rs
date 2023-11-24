use crate::{models::commen::*, AppState};
use actix_web::{
    http::{header::ContentType, StatusCode},
    web::{Data as ActData, Json as ActJson, Path as ActPath},
    HttpResponse,
};

use service::*;
use uuid::Uuid;

// i like my functions to stay inline
// type TFiltersBody = ActJson<FiltersBody>;
type State = ActData<AppState>;
type IdParam = ActPath<Uuid>;
type CtBody = ActJson<CStreet>;

pub async fn create_street(body: CtBody, state: State) -> HttpResponse {
    let res = ServiceMutation::create_street(&state.db_conn, body.into_inner()).await;
    match res {
        Ok(id) => HttpResponse::Ok()
            .status(StatusCode::CREATED)
            .content_type(ContentType::json())
            .json(ResultResponse {
                error: None,
                message: Some("Street created successfully".to_string()),
                data: Some(id.to_string()),
            }),
        Err(e) => HttpResponse::Ok()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .content_type(ContentType::json())
            .json(ResultResponse::<Option<String>> {
                error: Some(e.to_string()),
                message: None,
                data: None,
            }),
    }
}

pub async fn delete_street(id: IdParam, state: State) -> HttpResponse {
    let delete_res = ServiceMutation::delete_street(&state.db_conn, id.into_inner()).await;

    match delete_res {
        Ok(i) => HttpResponse::Created()
            .content_type(ContentType::json())
            .json(ResultResponse {
                error: None,
                message: Some("Street deleted successfully".to_string()),
                data: Some(i.to_string()),
            }),
        Err(e) => HttpResponse::InternalServerError()
            .content_type(ContentType::json())
            .json(ResultResponse::<Option<String>> {
                error: Some(e),
                message: None,
                data: None,
            }),
    }
}

pub async fn update_street(id: IdParam, body: CtBody, state: State) -> HttpResponse {
    let update_res =
        ServiceMutation::update_street(&state.db_conn, id.into_inner(), body.into_inner()).await;
    match update_res {
        Ok(i) => HttpResponse::Created()
            .content_type(ContentType::json())
            .json(ResultResponse {
                error: None,
                message: Some("Street updated successfully".to_string()),
                data: Some(i),
            }),
        Err(e) => HttpResponse::InternalServerError()
            .content_type(ContentType::json())
            .json(ResultResponse::<Option<String>> {
                error: Some(e),
                message: None,
                data: None,
            }),
    }
}
