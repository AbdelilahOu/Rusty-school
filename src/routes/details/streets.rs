use actix_web::{web, Scope};

use crate::handlers::details;

pub fn load_streets_routes() -> Scope {
    web::scope("/streets")
        .route("/all", web::post().to(details::streets::list))
        .route("/", web::post().to(details::streets::create))
        .route("/{id}", web::put().to(details::streets::update))
        .route("/{id}", web::delete().to(details::streets::delete))
}
