use actix_web::{web, Scope};

use crate::handlers::academic::grading_rubric;

pub fn load_grading_rubric_routes() -> Scope {
    web::scope("/grading_rubric")
        .route("/all", web::post().to(grading_rubric::list))
        .route("/", web::post().to(grading_rubric::create))
        .route("/{id}", web::put().to(grading_rubric::update))
        .route("/{id}", web::delete().to(grading_rubric::delete))
}
