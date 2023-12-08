use actix_web::{web, Scope};

use crate::handlers::details;

pub fn load_streets_routes() -> Scope {
    web::scope("/streets")
        .route("/filters", web::post().to(details::list_streets))
        .route("/", web::post().to(details::create_street))
        .route("/{id}", web::put().to(details::update_street))
        .route("/{id}", web::delete().to(details::delete_street))
}
