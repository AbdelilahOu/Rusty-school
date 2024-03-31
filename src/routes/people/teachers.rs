use crate::handlers::people::teachers;

use actix_web::{web, Scope};

pub fn load_teachers_routes() -> Scope {
    web::scope("/teachers")
        .route("/", web::post().to(teachers::create))
        .route("/{id}", web::put().to(teachers::update))
        .route("/all", web::post().to(teachers::list))
        .route("/{id}", web::delete().to(teachers::delete))
        .route(
            "/{id}/subject/{sub_id}",
            web::post().to(teachers::add_subject),
        )
        .route("/subject/{id}", web::delete().to(teachers::delete_subject))
}
