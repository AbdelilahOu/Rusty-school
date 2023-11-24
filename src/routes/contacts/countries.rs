use actix_web::{web, Scope};

use crate::handlers::contacts;

pub fn load_countries_routes() -> Scope {
    web::scope("/countries")
        .route("/", web::get().to(contacts::get_countries))
        .route("/", web::post().to(contacts::create_country))
        .route("/{id}", web::put().to(contacts::update_country))
        .route("/{id}", web::delete().to(contacts::delete_country))
}
