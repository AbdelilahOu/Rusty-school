use actix_web::{web, Scope};

use crate::handlers::details;

pub fn load_cities_routes() -> Scope {
    web::scope("/cities")
        .route("/all", web::post().to(details::list_cities))
        .route("/", web::post().to(details::create_city))
        .route("/{id}", web::put().to(details::update_city))
        .route("/{id}", web::delete().to(details::delete_city))
}
