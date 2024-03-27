use actix_web::{web, Scope};

use crate::handlers::grades;

pub fn load_grades_routes() -> Scope {
    web::scope("/grades")
        .route("/", web::post().to(grades::create_grade))
        .route("/{id}", web::put().to(grades::update_grade))
        .route("/{id}", web::delete().to(grades::delete_grade))
}
