use actix_web::{web, Scope};

use crate::handlers::contacts;

pub fn load_districts_routes() -> Scope {
    web::scope("/districts")
        // .route("/", web::get().to(contacts::get_district))
        .route("/", web::post().to(contacts::create_district))
        .route("/{id}", web::put().to(contacts::update_district))
        .route("/{id}", web::delete().to(contacts::delete_district))
}
