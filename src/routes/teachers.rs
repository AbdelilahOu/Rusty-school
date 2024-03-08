use crate::handlers::teachers;
use actix_web::{web, Scope};

pub fn load_teachers_routes() -> Scope {
    web::scope("/teachers")
        .route("/", web::post().to(teachers::create_teacher))
        .route("/{id}", web::get().to(teachers::get_teacher))
        .route("/{id}", web::put().to(teachers::update_teacher))
        .route("/all", web::post().to(teachers::list_teachers))
        .route("/{id}", web::delete().to(teachers::delete_teacher))
        .route(
            "/{id}/subjects/{sub_id}",
            web::post().to(teachers::add_subject),
        )
        .route("/subjects/{id}", web::delete().to(teachers::delete_subject))
}
