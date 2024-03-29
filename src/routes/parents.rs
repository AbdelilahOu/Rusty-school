use crate::handlers::parents;

use actix_web::{web, Scope};

pub fn load_parents_routes() -> Scope {
    web::scope("/parents")
        .route("/", web::post().to(parents::create))
        .route("/{id}", web::put().to(parents::update))
        .route("/all", web::post().to(parents::list))
        .route("/{id}", web::delete().to(parents::delete))
}
