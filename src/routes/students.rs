use crate::handlers::students;

use actix_web::{web, Scope};

pub fn load_students_routes() -> Scope {
    web::scope("/students")
        .route("/", web::post().to(students::create))
        .route("/{id}", web::put().to(students::update))
        .route("/all", web::post().to(students::list))
        .route("/{id}", web::delete().to(students::delete))
}
