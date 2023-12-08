use crate::models::commen::*;
use actix_web::{
    http::{header::ContentType, StatusCode},
    web::Json as ActJson,
    HttpResponse,
};

use service::*;

// i like my functions to stay inline
type CtBody = ActJson<CDistrict>;

pub async fn create_district(body: CtBody, state: State) -> HttpResponse {
    let res = ServiceMutation::create_district(&state.db_conn, body.into_inner()).await;
    match res {
        Ok(id) => HttpResponse::Ok()
            .status(StatusCode::CREATED)
            .content_type(ContentType::json())
            .json(ResponseData {
                error: None,
                message: Some("District created successfully".to_string()),
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

pub async fn list_districts(queries: TQueries, body: TFiltersBody, state: State) -> HttpResponse {
    let res = ServiceQuery::list_districts(
        &state.db_conn,
        QueriesFilters {
            queries: queries.into_inner(),
            filters: body.clone().filters,
        },
    )
    .await;
    match res {
        Ok(i) => HttpResponse::Ok()
            .content_type(ContentType::json())
            .json(ResponseData {
                error: None,
                message: Some("Districts fetched successfully".to_string()),
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

pub async fn delete_district(id: IdParam, state: State) -> HttpResponse {
    let delete_res = ServiceMutation::delete_district(&state.db_conn, id.into_inner()).await;

    match delete_res {
        Ok(i) => HttpResponse::Created()
            .content_type(ContentType::json())
            .json(ResponseData {
                error: None,
                message: Some("District deleted successfully".to_string()),
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

pub async fn update_district(id: IdParam, body: CtBody, state: State) -> HttpResponse {
    let update_res =
        ServiceMutation::update_district(&state.db_conn, id.into_inner(), body.into_inner()).await;
    match update_res {
        Ok(i) => HttpResponse::Created()
            .content_type(ContentType::json())
            .json(ResponseData {
                error: None,
                message: Some("District updated successfully".to_string()),
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
