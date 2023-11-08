use axum::{
    routing::{delete, get, post, put},
    Router,
};

use crate::handlers::health::health_check;

pub fn load_classes_routes() -> Router {
    let router = Router::new()
        .route("/", get(health_check()))
        .route("/", post(health_check()))
        .route("/", delete(health_check()))
        .route("/", put(health_check()));
    router
}
