use actix_web::{web, Scope};

use crate::handlers::academic::disciplinary_actions;

pub fn load_disciplinary_actions_routes() -> Scope {
    web::scope("/disciplinary_actions")
        .route("/", web::get().to(disciplinary_actions::list))
        .route("/", web::post().to(disciplinary_actions::create))
        .route("/{id}", web::put().to(disciplinary_actions::update))
        .route("/{id}", web::delete().to(disciplinary_actions::delete))
}
