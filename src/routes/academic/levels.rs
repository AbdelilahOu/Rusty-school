use actix_web::{web, Scope};

use crate::handlers::academic::levels;

pub fn load_levels_routes() -> Scope {
    web::scope("/levels")
        .route("/all", web::post().to(levels::list))
        .route("/", web::put().to(levels::update))
        .route("/", web::post().to(levels::create))
        .route("/", web::delete().to(levels::delete))
}
