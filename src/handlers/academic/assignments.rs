use crate::models::commen::*;
use ::service::{models::CAssignment, mutation::*, query::*};
use actix_web::{
    http::{header::ContentType, StatusCode},
    web::Json as ActJson,
    HttpResponse as HttpRes,
};
//
type Body = ActJson<CAssignment>;

pub async fn create(body: Body, state: State) -> HttpRes {
    let res = MutationsService::create_assignment(&state.db_conn, body.into_inner()).await;
    match res {
        Ok(id) => HttpRes::Ok()
            .status(StatusCode::CREATED)
            .content_type(ContentType::json())
            .json(ResponseData {
                error: None,
                message: Some("Assignment created successfully".to_string()),
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

pub async fn delete(id: IdParam, state: State) -> HttpRes {
    let delete_res = MutationsService::delete_assignment(&state.db_conn, id.into_inner()).await;

    match delete_res {
        Ok(i) => HttpRes::Created()
            .content_type(ContentType::json())
            .json(ResponseData {
                error: None,
                message: Some("Assignment deleted successfully".to_string()),
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

pub async fn list(queries: TQueries, body: TFiltersBody, state: State) -> HttpRes {
    let assignmentes = QueriesService::list_assignments(
        &state.db_conn,
        QueriesFilters {
            queries: queries.into_inner(),
            filters: body.clone().filters,
        },
    )
    .await;

    match assignmentes {
        Ok(i) => HttpRes::Created()
            .content_type(ContentType::json())
            .json(ResponseData {
                error: None,
                message: Some("Assignments selected successfully".to_string()),
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

pub async fn update(id: IdParam, body: Body, state: State) -> HttpRes {
    let update_res =
        MutationsService::update_assignment(&state.db_conn, id.into_inner(), body.into_inner())
            .await;
    match update_res {
        Ok(i) => HttpRes::Created()
            .content_type(ContentType::json())
            .json(ResponseData {
                error: None,
                message: Some("Assignment updated successfully".to_string()),
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
