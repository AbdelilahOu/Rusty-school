use crate::types::shared::{ResponseData, State};
use actix_web::{
    web::{Json, Path, Query},
    HttpResponse,
};
use service::{
    models::{Teacher, TeacherQuery},
    mutation::MutationService,
    query::QueryService,
    uuid::Uuid,
};

type Body = Json<Teacher>;
pub async fn create(body: Body, state: State) -> HttpResponse {
    let res = MutationService::create_teacher(&state.db_conn, body.into_inner()).await;
    match res {
        Ok(id) => HttpResponse::Ok().json(ResponseData {
            error: None,
            message: Some("Teacher created successfully".to_string()),
            data: Some(id.to_string()),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ResponseData::<Option<String>> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}

pub async fn add_subject(params: Path<(Uuid, Uuid)>, state: State) -> HttpResponse {
    let res = MutationService::create_teacher_subject(&state.db_conn, params.into_inner()).await;
    match res {
        Ok(i) => HttpResponse::Ok().json(ResponseData {
            error: None,
            message: Some("Subject added successfully".to_string()),
            data: Some(i.to_string()),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ResponseData::<Option<String>> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}

pub async fn delete_subject(id: Path<Uuid>, state: State) -> HttpResponse {
    let res = MutationService::delete_teacher_subject(&state.db_conn, id.into_inner()).await;
    match res {
        Ok(i) => HttpResponse::Ok().json(ResponseData {
            error: None,
            message: Some("Subject deleted successfully".to_string()),
            data: Some(i.to_string()),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ResponseData::<Option<String>> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}

pub async fn delete(id: Path<Uuid>, state: State) -> HttpResponse {
    let delete_res = MutationService::delete_teacher(&state.db_conn, id.into_inner()).await;
    match delete_res {
        Ok(i) => HttpResponse::Ok().json(ResponseData {
            error: None,
            message: Some("Teacher deleted successfully".to_string()),
            data: Some(i.to_string()),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ResponseData::<Option<String>> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}

pub async fn list(q: Query<TeacherQuery>, state: State) -> HttpResponse {
    let teachers = QueryService::list_teachers(&state.db_conn, q.into_inner()).await;
    match teachers {
        Ok(i) => HttpResponse::Ok().json(ResponseData {
            error: None,
            message: Some("Teachers selected successfully".to_string()),
            data: Some(i),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ResponseData::<Option<String>> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}

pub async fn update(id: Path<Uuid>, body: Body, state: State) -> HttpResponse {
    let update_res =
        MutationService::update_teacher(&state.db_conn, id.into_inner(), body.into_inner()).await;
    match update_res {
        Ok(i) => HttpResponse::Ok().json(ResponseData {
            error: None,
            message: Some("Teacher updated successfully".to_string()),
            data: Some(i),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ResponseData::<Option<String>> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}
