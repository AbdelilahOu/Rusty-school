use actix_web::{web, Scope};

use crate::handlers::contacts;

pub fn load_streets_routes() -> Scope {
    web::scope("/streets")
        // .route("/", web::get().to(contacts::get_street))
        .route("/", web::post().to(contacts::create_street))
        .route("/{id}", web::put().to(contacts::update_street))
        .route("/{id}", web::delete().to(contacts::delete_street))
}
