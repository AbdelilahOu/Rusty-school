use actix_web::{web, Scope};

use crate::handlers::details;

pub fn load_countries_routes() -> Scope {
    web::scope("/countries")
        .route("/filter", web::post().to(details::get_countries))
        .route("/", web::post().to(details::create_country))
        .route("/{id}", web::put().to(details::update_country))
        .route("/{id}", web::delete().to(details::delete_country))
}
