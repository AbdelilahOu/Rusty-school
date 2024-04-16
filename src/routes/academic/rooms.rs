use actix_web::{web, Scope};

use crate::handlers::academic::rooms;

pub fn load_rooms_routes() -> Scope {
    web::scope("/rooms")
        .route("/", web::get().to(rooms::list))
        .route("/", web::post().to(rooms::create))
        .route("/{id}", web::put().to(rooms::update))
        .route("/{id}", web::delete().to(rooms::delete))
}
