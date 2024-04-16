use crate::handlers::academic::subjects;
use actix_web::{web, Scope};

pub fn load_subjects_routes() -> Scope {
    web::scope("/subjects")
        .route("/", web::get().to(subjects::list))
        .route("/", web::post().to(subjects::create))
        .route("/{id}", web::put().to(subjects::update))
        .route("/{id}", web::delete().to(subjects::delete))
        .route(
            "/by-level-id/{id}",
            web::get().to(subjects::list_by_level_id),
        )
}
