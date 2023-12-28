use actix_web::{web, Scope};

use crate::handlers::rooms;

pub fn load_rooms_routes() -> Scope {
    web::scope("/rooms")
        .route("/{id}", web::put().to(rooms::update_room))
        .route("/", web::post().to(rooms::create_room))
        .route("/", web::get().to(rooms::create_room))
        .route("/{id}", web::delete().to(rooms::delete_room))
}
