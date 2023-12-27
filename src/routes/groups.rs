use actix_web::{web, Scope};

use crate::handlers::groups;

pub fn load_groups_routes() -> Scope {
    web::scope("/groups")
        .route("/filters", web::post().to(groups::list_groups))
        .route("/{id}", web::put().to(groups::update_group))
        .route("/by-level-id/{id}", web::get().to(groups::get_level_groups))
        .route("/", web::post().to(groups::create_group))
        .route("/{id}", web::delete().to(groups::delete_group))
}
