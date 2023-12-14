use actix_web::{web, Scope};

use crate::handlers::levels;

pub fn load_levels_routes() -> Scope {
    web::scope("/levels")
        .route("/filters", web::post().to(levels::list_levels))
        .route("/", web::put().to(levels::update_level))
        .route("/", web::post().to(levels::create_level))
        .route("/", web::delete().to(levels::delete_level))
}
