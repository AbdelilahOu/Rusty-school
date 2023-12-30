use actix_web::{web, Scope};

use crate::handlers::classes;

pub fn load_classes_routes() -> Scope {
    web::scope("/classes")
        .route("/{id}", web::put().to(classes::update_classe))
        .route("/", web::post().to(classes::create_classe))
        .route("/", web::get().to(classes::list_classes))
        .route("/{id}", web::delete().to(classes::delete_classe))
}
