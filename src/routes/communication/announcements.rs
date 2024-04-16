use crate::handlers::communication::announcements;
use actix_web::{web, Scope};
pub fn load_announcements_routes() -> Scope {
    web::scope("/announcements")
        .route("/", web::get().to(announcements::list))
        .route("/", web::post().to(announcements::create))
        .route("/{id}", web::put().to(announcements::update))
        .route("/{id}", web::delete().to(announcements::delete))
}
