use crate::handlers::people::parents;
use actix_web::{web, Scope};

pub fn load_parents_routes() -> Scope {
    web::scope("/parents")
        .route("/", web::get().to(parents::list))
        .route("/", web::post().to(parents::create))
        .route("/{id}", web::put().to(parents::update))
        .route("/{id}", web::delete().to(parents::delete))
}
