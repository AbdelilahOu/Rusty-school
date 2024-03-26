use actix_web::{web, Scope};

use crate::handlers::grades;

pub fn load_grades_routes() -> Scope {
    web::scope("/grades")
        .route("/", web::post().to(gradess::create_grade))
        .route("/{id}", web::put().to(gradess::update_grade))
        .route("/{id}", web::delete().to(gradess::delete_grade))
}
