use crate::types::shared::{ResponseData, State};
use actix_web::{
    web::{Json, Path, Query},
    HttpResponse as Response,
};
use service::{
    models::{Subject, SubjectQuery},
    mutation::MutationService,
    query::QueryService,
    uuid::Uuid,
};

//
pub async fn create(body: Json<Subject>, state: State) -> Response {
    let res = MutationService::create_subject(&state.db_conn, body.into_inner()).await;
    match res {
        Ok(id) => Response::Created().json(ResponseData {
            error: None,
            message: Some("Subject created successfully".to_string()),
            data: Some(id.to_string()),
        }),
        Err(e) => Response::InternalServerError().json(ResponseData::<Option<String>> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}

pub async fn delete(id: Path<Uuid>, state: State) -> Response {
    let res = MutationService::delete_subject(&state.db_conn, id.into_inner()).await;
    match res {
        Ok(delete_count) => Response::Ok().json(ResponseData {
            error: None,
            message: Some("Subject deleted successfully".to_string()),
            data: Some(delete_count.to_string()),
        }),
        Err(e) => Response::InternalServerError().json(ResponseData::<Option<String>> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}

pub async fn list_by_level_id(id: Path<Uuid>, state: State) -> Response {
    let res = QueryService::list_level_subjects(&state.db_conn, id.into_inner()).await;
    match res {
        Ok(subjects) => Response::Ok().json(ResponseData {
            error: None,
            message: Some("Subject selected by level id successfully".to_string()),
            data: Some(subjects),
        }),
        Err(e) => Response::InternalServerError().json(ResponseData::<Option<String>> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}

pub async fn list(query: Query<SubjectQuery>, state: State) -> Response {
    let res = QueryService::list_subjects(&state.db_conn, query.into_inner()).await;
    match res {
        Ok(subjects) => Response::Ok().json(ResponseData {
            error: None,
            message: Some("Subjects selected successfully".to_string()),
            data: Some(subjects),
        }),
        Err(e) => Response::InternalServerError().json(ResponseData::<Option<String>> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}

pub async fn update(id: Path<Uuid>, body: Json<Subject>, state: State) -> Response {
    let res =
        MutationService::update_subject(&state.db_conn, id.into_inner(), body.into_inner()).await;
    match res {
        Ok(id) => Response::Ok().json(ResponseData {
            error: None,
            message: Some("Subject updated successfully".to_string()),
            data: Some(id),
        }),
        Err(e) => Response::InternalServerError().json(ResponseData::<Option<String>> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}
