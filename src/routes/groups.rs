use actix_web::{web, Scope};

use crate::handlers::groups;

pub fn load_groups_routes() -> Scope {
    web::scope("/groups")
        .route("/all", web::post().to(groups::list))
        .route("/{id}", web::put().to(groups::update))
        .route("/by-level-id/{id}", web::get().to(groups::list_by_level_id))
        .route("/", web::post().to(groups::create))
        .route("/{id}", web::delete().to(groups::delete))
}
