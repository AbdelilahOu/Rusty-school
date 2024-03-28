use actix_web::{web, Scope};

use crate::handlers::details;

pub fn load_states_routes() -> Scope {
    web::scope("/states")
        .route("/all", web::post().to(details::states::list))
        .route("/", web::post().to(details::states::create))
        .route("/{id}", web::put().to(details::states::update))
        .route("/{id}", web::delete().to(details::states::delete))
}
