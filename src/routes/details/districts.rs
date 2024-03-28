use actix_web::{web, Scope};

use crate::handlers::details;

pub fn load_districts_routes() -> Scope {
    web::scope("/districts")
        .route("/all", web::post().to(details::districts::list))
        .route("/", web::post().to(details::districts::create))
        .route("/{id}", web::put().to(details::districts::update))
        .route("/{id}", web::delete().to(details::districts::delete))
}
