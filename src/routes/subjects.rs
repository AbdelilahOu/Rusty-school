use actix_web::{web, Scope};

use crate::handlers::subjects;

pub fn load_subjects_routes() -> Scope {
    web::scope("/subjects")
        .route("/all", web::post().to(subjects::list))
        .route("/{id}", web::put().to(subjects::update))
        .route(
            "/by-level-id/{id}",
            web::get().to(subjects::list_by_level_id),
        )
        .route("/", web::post().to(subjects::create))
        .route("/{id}", web::delete().to(subjects::delete))
}
