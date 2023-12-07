use actix_web::{web, Scope};

use crate::handlers::details;

pub fn load_districts_routes() -> Scope {
    web::scope("/districts")
        .route("/filters", web::post().to(details::get_districts))
        .route("/", web::post().to(details::create_district))
        .route("/{id}", web::put().to(details::update_district))
        .route("/{id}", web::delete().to(details::delete_district))
}
