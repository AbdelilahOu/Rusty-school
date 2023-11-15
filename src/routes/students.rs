use crate::handlers::students;
use actix_web::{web, Scope};

pub fn load_students_routes() -> Scope {
    web::scope("/students")
        .route("/", web::post().to(students::create_student))
        .route("/{id}", web::get().to(students::get_student))
        .route("/{id}", web::put().to(students::update_student))
        .route("/search", web::post().to(students::get_students))
        .route("/{id}", web::delete().to(students::delete_student))
}
