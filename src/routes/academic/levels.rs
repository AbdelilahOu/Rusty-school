use crate::handlers::academic::levels;
use actix_web::{web, Scope};

pub fn load_levels_routes() -> Scope {
    web::scope("/levels")
        .route("/", web::get().to(levels::list))
        .route("/", web::post().to(levels::create))
        .route("/{id}", web::put().to(levels::update))
        .route("/{id}", web::delete().to(levels::delete))
}
