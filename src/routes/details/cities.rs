use actix_web::{web, Scope};

use crate::handlers::details;

pub fn load_cities_routes() -> Scope {
    web::scope("/cities")
        .route("/all", web::post().to(details::cities::list))
        .route("/", web::post().to(details::cities::create))
        .route("/{id}", web::put().to(details::cities::update))
        .route("/{id}", web::delete().to(details::cities::delete))
}
