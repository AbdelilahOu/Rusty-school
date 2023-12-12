use actix_web::{web, Scope};

use crate::handlers::levels;

pub fn load_levels_routes() -> Scope {
    web::scope("/levels")
        .route("/", web::post().to(levels::create_level))
        .route("/", web::get().to(levels::list_levels))
}
