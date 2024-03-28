use actix_web::{web, Scope};

use crate::handlers::assignments;

pub fn load_assignments_routes() -> Scope {
    web::scope("/assignments")
        .route("/", web::get().to(assignments::list))
        .route("/", web::post().to(assignments::create))
        .route("/{id}", web::put().to(assignments::update))
        .route("/{id}", web::delete().to(assignments::delete))
}
