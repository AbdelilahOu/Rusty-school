use actix_web::{web, Scope};

use crate::handlers::subjects;

pub fn load_subjects_routes() -> Scope {
    web::scope("/subjects")
        .route("/all", web::post().to(subjects::list_subjects))
        .route("/{id}", web::put().to(subjects::update_subject))
        .route(
            "/by-level-id/{id}",
            web::get().to(subjects::get_level_subjects),
        )
        .route("/", web::post().to(subjects::create_subject))
        .route("/{id}", web::delete().to(subjects::delete_subject))
}
