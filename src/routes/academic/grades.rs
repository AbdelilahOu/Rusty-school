use actix_web::{web, Scope};

use crate::handlers::academic::grades;

pub fn load_grades_routes() -> Scope {
    web::scope("/grades")
        .route("/all", web::post().to(grades::list))
        .route("/", web::post().to(grades::create))
        .route("/{id}", web::put().to(grades::update))
        .route("/{id}", web::delete().to(grades::delete))
}
