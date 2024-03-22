use actix_web::{web, Scope};

use crate::handlers::assignment;

pub fn load_assignment_routes() -> Scope {
    web::scope("/assignment")
        .route("/", web::get().to(assignment::list_assignment))
        .route("/", web::post().to(assignment::create_assignment))
        .route("/{id}", web::put())
        .route("/{id}", web::delet())
}
