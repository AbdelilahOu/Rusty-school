use crate::handlers::academic::groups;
use actix_web::{web, Scope};

pub fn load_groups_routes() -> Scope {
    web::scope("/groups")
        .route("/", web::get().to(groups::list))
        .route("/", web::post().to(groups::create))
        .route("/{id}", web::put().to(groups::update))
        .route("/{id}", web::delete().to(groups::delete))
        .route("/by-level-id/{id}", web::get().to(groups::list_by_level_id))
}
