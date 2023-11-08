use axum::{
    routing::{delete, get, post, put},
    Router,
};

use crate::handlers::students::*;

pub fn load_students_routes() -> Router {
    let router = Router::new()
        .route("/", get(list_students()))
        .route("/:id", get(get_student()))
        .route("/", post(create_student()))
        .route("/:id", delete(delete_student()))
        .route("/:id", put(update_student()));
    router
}
