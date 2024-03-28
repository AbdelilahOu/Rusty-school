use actix_web::{web, Scope};

use crate::handlers::disciplinary_actions;

pub fn load_disciplinary_actions_routes() -> Scope {
    web::scope("/disciplinary_actions")
        .route(
            "/",
            web::get().to(disciplinary_actions::list_disciplinary_actions),
        )
        .route(
            "/",
            web::post().to(disciplinary_actions::create_disciplinary_action),
        )
        .route(
            "/{id}",
            web::put().to(disciplinary_actions::update_disciplinary_action),
        )
        .route(
            "/{id}",
            web::delete().to(disciplinary_actions::delete_disciplinary_action),
        )
}
