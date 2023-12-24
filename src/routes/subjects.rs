use actix_web::{web, Scope};

use crate::handlers::subjects;

pub fn load_subjects_routes() -> Scope {
    web::scope("/subjects")
        .route("/filters", web::post().to(subjects::list_subjects))
        .route("/", web::put().to(subjects::update_subject))
        .route("/", web::post().to(subjects::create_subject))
        .route("/", web::delete().to(subjects::delete_subject))
}
