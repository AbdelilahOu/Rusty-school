use crate::handlers::academic::classes;
use actix_web::{web, Scope};

pub fn load_classes_routes() -> Scope {
    web::scope("/classes")
        .route("/{id}", web::put().to(classes::update))
        .route("/", web::post().to(classes::create))
        .route("/", web::get().to(classes::list))
        .route("/{id}", web::delete().to(classes::delete))
}
