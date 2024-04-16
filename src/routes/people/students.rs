use crate::handlers::people::students;
use actix_web::{web, Scope};
pub fn load_students_routes() -> Scope {
    web::scope("/students")
        .route("/", web::get().to(students::list))
        .route("/", web::post().to(students::create))
        .route("/{id}", web::put().to(students::update))
        .route("/{id}", web::delete().to(students::delete))
}
