use crate::handlers::parents;
use actix_web::{web, Scope};

pub fn load_parents_routes() -> Scope {
    web::scope("/parents")
        .route("/", web::post().to(parents::create_parent))
        .route("/{id}", web::put().to(parents::update_parent))
        .route("/all", web::post().to(parents::list_parents))
        .route("/{id}", web::delete().to(parents::delete_parent))
}
