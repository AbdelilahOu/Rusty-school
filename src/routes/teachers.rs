use crate::handlers::teachers;
use actix_web::{web, Scope};

pub fn load_teachers_routes() -> Scope {
    web::scope("/teachers")
        .route("/", web::post().to(teachers::create_teacher))
        .route("/{id}", web::get().to(teachers::get_teacher))
        .route("/{id}", web::put().to(teachers::update_teacher))
        .route("/search", web::post().to(teachers::get_teachers))
        .route("/{id}", web::delete().to(teachers::delete_teacher))
}
