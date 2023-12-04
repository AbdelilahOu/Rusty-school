use crate::models::commen::*;
use actix_web::{
    http::{header::ContentType, StatusCode},
    web::Json as ActJson,
    HttpResponse,
};
use service::*;
// i like my functions to stay inline
type TeBody = ActJson<CTeacher>;

pub async fn create_teacher(body: TeBody, state: State) -> HttpResponse {
    let res = ServiceMutation::create_teacher(&state.db_conn, body.into_inner()).await;
    match res {
        Ok(id) => HttpResponse::Ok()
            .status(StatusCode::CREATED)
            .content_type(ContentType::json())
            .json(ResponseData {
                error: None,
                message: Some("Teacher created successfully".to_string()),
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

pub async fn delete_teacher(id: IdParam, state: State) -> HttpResponse {
    let delete_res = ServiceMutation::delete_teacher(&state.db_conn, id.into_inner()).await;

    match delete_res {
        Ok(i) => HttpResponse::Created()
            .content_type(ContentType::json())
            .json(ResponseData {
                error: None,
                message: Some("Teacher deleted successfully".to_string()),
                data: Some(i.to_string()),
            }),
        Err(e) => HttpResponse::InternalServerError()
            .content_type(ContentType::json())
            .json(ResponseData::<Option<String>> {
                error: Some(e),
                message: None,
                data: None,
            }),
    }
}

pub async fn get_teacher(id: IdParam, state: State) -> HttpResponse {
    let selected_teacher = ServiceQuery::get_teacher(&state.db_conn, id.into_inner()).await;

    match selected_teacher {
        Ok(i) => HttpResponse::Created()
            .content_type(ContentType::json())
            .json(ResponseData {
                error: None,
                message: Some("Teacher selected successfully".to_string()),
                data: Some(i),
            }),
        Err(e) => HttpResponse::InternalServerError()
            .content_type(ContentType::json())
            .json(ResponseData::<Option<String>> {
                error: Some(e),
                message: None,
                data: None,
            }),
    }
}

pub async fn get_teachers(queries: TQueries, body: TFiltersBody, state: State) -> HttpResponse {
    let teachers = ServiceQuery::list_teachers(
        &state.db_conn,
        QueriesFilters {
            queries: queries.into_inner(),
            filters: body.clone().filters,
        },
    )
    .await;

    match teachers {
        Ok(i) => HttpResponse::Created()
            .content_type(ContentType::json())
            .json(ResponseData {
                error: None,
                message: Some("Teachers selected successfully".to_string()),
                data: Some(i),
            }),
        Err(e) => HttpResponse::InternalServerError()
            .content_type(ContentType::json())
            .json(ResponseData::<Option<String>> {
                error: Some(e),
                message: None,
                data: None,
            }),
    }
}

pub async fn update_teacher(id: IdParam, body: TeBody, state: State) -> HttpResponse {
    let update_res =
        ServiceMutation::update_teacher(&state.db_conn, id.into_inner(), body.into_inner()).await;
    match update_res {
        Ok(i) => HttpResponse::Created()
            .content_type(ContentType::json())
            .json(ResponseData {
                error: None,
                message: Some("Teacher updated successfully".to_string()),
                data: Some(i),
            }),
        Err(e) => HttpResponse::InternalServerError()
            .content_type(ContentType::json())
            .json(ResponseData::<Option<String>> {
                error: Some(e),
                message: None,
                data: None,
            }),
    }
}
