use actix_web::{web, Scope};

use crate::handlers::rooms;

pub fn load_rooms_routes() -> Scope {
    web::scope("/rooms")
        .route("/{id}", web::put().to(rooms::update))
        .route("/", web::post().to(rooms::create))
        .route("/", web::get().to(rooms::list))
        .route("/{id}", web::delete().to(rooms::delete))
}
