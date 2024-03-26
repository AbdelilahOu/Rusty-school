use crate::models::commen::*;
use actix_web::{
    http::{header::ContentType, StatusCode},
    web::Json as ActJson,
    HttpResponse as HttpRes,
};
use service::{models::CGrade, mutation::*, query::*};
//
type StBody = ActJson<CGrade>;

pub async fn create_grade(body: StBody, state: State) -> HttpRes {
    let res = MutationsService::create_grade(&state.db_conn, body.into_inner()).await;
    match res {
        Ok(id) => HttpRes::Ok()
            .status(StatusCode::CREATED)
            .content_type(ContentType::json())
            .json(ResponseData {
                error: None,
                message: Some("Grade created successfully".to_string()),
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

pub async fn delete_grade(id: IdParam, state: State) -> HttpRes {
    let delete_res = MutationsService::delete_grade(&state.db_conn, id.into_inner()).await;

    match delete_res {
        Ok(i) => HttpRes::Created()
            .content_type(ContentType::json())
            .json(ResponseData {
                error: None,
                message: Some("Grade deleted successfully".to_string()),
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

pub async fn update_grade(id: IdParam, body: StBody, state: State) -> HttpRes {
    let update_res =
        MutationsService::update_grade(&state.db_conn, id.into_inner(), body.into_inner())
            .await;
    match update_res {
        Ok(i) => HttpRes::Created()
            .content_type(ContentType::json())
            .json(ResponseData {
                error: None,
                message: Some("Grade updated successfully".to_string()),
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
