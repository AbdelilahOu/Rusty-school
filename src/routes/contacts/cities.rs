use actix_web::{web, Scope};

use crate::handlers::contacts;

pub fn load_cities_routes() -> Scope {
    web::scope("/cities")
        .route("/filters", web::post().to(contacts::get_cities))
        .route("/", web::post().to(contacts::create_city))
        .route("/{id}", web::put().to(contacts::update_city))
        .route("/{id}", web::delete().to(contacts::delete_city))
}
