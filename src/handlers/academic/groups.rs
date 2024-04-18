use crate::types::shared::{ResponseData, State};
use actix_web::{
    web::{Json, Path, Query},
    HttpResponse as Response,
};
use service::{
    models::{Group, GroupQuery},
    mutation::MutationService,
    query::QueryService,
    uuid::Uuid,
};

//
pub async fn create(body: Json<Group>, state: State) -> Response {
    let res = MutationService::create_group(&state.db_conn, body.into_inner()).await;
    match res {
        Ok(id) => Response::Created().json(ResponseData {
            error: None,
            message: Some("Group created successfully".to_string()),
            data: Some(id.to_string()),
        }),
        Err(e) => Response::InternalServerError().json(ResponseData::<String> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}

pub async fn delete(id: Path<Uuid>, state: State) -> Response {
    let res = MutationService::delete_group(&state.db_conn, id.into_inner()).await;
    match res {
        Ok(delete_count) => Response::Ok().json(ResponseData {
            error: None,
            message: Some("Group deleted successfully".to_string()),
            data: Some(delete_count.to_string()),
        }),
        Err(e) => Response::InternalServerError().json(ResponseData::<String> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}

pub async fn list_by_level_id(id: Path<Uuid>, state: State) -> Response {
    let res = QueryService::list_level_groups(&state.db_conn, id.into_inner()).await;
    match res {
        Ok(groups) => Response::Ok().json(ResponseData {
            error: None,
            message: Some("Groups selected by level id successfully".to_string()),
            data: Some(groups),
        }),
        Err(e) => Response::InternalServerError().json(ResponseData::<String> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}

pub async fn list(query: Query<GroupQuery>, state: State) -> Response {
    let res = QueryService::list_groups(&state.db_conn, query.into_inner()).await;
    match res {
        Ok(groups) => Response::Ok().json(ResponseData {
            error: None,
            message: Some("Groups selected successfully".to_string()),
            data: Some(groups),
        }),
        Err(e) => Response::InternalServerError().json(ResponseData::<String> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}

pub async fn update(id: Path<Uuid>, body: Json<Group>, state: State) -> Response {
    let res =
        MutationService::update_group(&state.db_conn, id.into_inner(), body.into_inner()).await;
    match res {
        Ok(id) => Response::Ok().json(ResponseData {
            error: None,
            message: Some("Group updated successfully".to_string()),
            data: Some(id),
        }),
        Err(e) => Response::InternalServerError().json(ResponseData::<String> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}
