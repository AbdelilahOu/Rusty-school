use actix_web::{web, Scope};

use crate::handlers::details;

pub fn load_countries_routes() -> Scope {
    web::scope("/countries")
        .route("/all", web::post().to(details::coutries::list))
        .route("/", web::post().to(details::coutries::create))
        .route("/{id}", web::put().to(details::coutries::update))
        .route("/{id}", web::delete().to(details::coutries::delete))
}
