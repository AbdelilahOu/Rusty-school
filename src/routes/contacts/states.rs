use actix_web::{web, Scope};

use crate::handlers::contacts;

pub fn load_states_routes() -> Scope {
    web::scope("/states")
        .route("/filter", web::post().to(contacts::get_states))
        .route("/", web::post().to(contacts::create_state))
        .route("/{id}", web::put().to(contacts::update_state))
        .route("/{id}", web::delete().to(contacts::delete_state))
}
