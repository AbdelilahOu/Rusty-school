use actix_web::{web, Scope};

use crate::handlers::assignments;

pub fn load_assignments_routes() -> Scope {
    web::scope("/assignments")
        .route("/", web::get().to(assignments::list_assignments))
        .route("/", web::post().to(assignments::create_assignment))
        .route("/{id}", web::put().to(assignments::update_assignment))
        .route("/{id}", web::delete().to(assignments::delete_assignment))
}
