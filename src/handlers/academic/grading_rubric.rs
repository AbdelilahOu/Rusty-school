use crate::types::shared::*;
use actix_web::{
    web::{Json, Path},
    HttpResponse,
};
use service::{models::Rubric, mutation::*, query::*, transaction::*, uuid::Uuid};
//
type Body = Json<Rubric>;
pub async fn list(queries: TQueries, body: TFiltersBody, state: State) -> HttpResponse {
    let gradees = QueriesService::list_rubrics(
        &state.db_conn,
        QueriesFilters {
            queries: queries.into_inner(),
            filters: body.clone().filters,
        },
    )
    .await;

    match gradees {
        Ok(i) => HttpResponse::Created().json(ResponseData {
            error: None,
            message: Some("grading rubrics selected successfully".to_string()),
            data: Some(i),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ResponseData::<Option<String>> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}

pub async fn create(body: Body, state: State) -> HttpResponse {
    let res = TransactionsService::create_rubric(&state.db_conn, body.into_inner()).await;
    match res {
        Ok(id) => HttpResponse::Ok().json(ResponseData {
            error: None,
            message: Some("grading rubric created successfully".to_string()),
            data: Some(id.to_string()),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ResponseData::<Option<String>> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}

pub async fn delete(id: Path<Uuid>, state: State) -> HttpResponse {
    let delete_res = MutationsService::delete_rubric(&state.db_conn, id.into_inner()).await;
    match delete_res {
        Ok(i) => HttpResponse::Created().json(ResponseData {
            error: None,
            message: Some("grading rubric deleted successfully".to_string()),
            data: Some(i.to_string()),
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
        MutationsService::update_rubric(&state.db_conn, id.into_inner(), body.into_inner()).await;
    match update_res {
        Ok(i) => HttpResponse::Created().json(ResponseData {
            error: None,
            message: Some("grading rubric updated successfully".to_string()),
            data: Some(i),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ResponseData::<Option<String>> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}
