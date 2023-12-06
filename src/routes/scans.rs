// use crate::handlers::scans;
use actix_web::{web, Scope};

use crate::handlers::{health_check, scans};

pub fn load_scans_routes() -> Scope {
    web::scope("/scans")
        .route("/", web::post().to(scans::create_scan))
        .route("/", web::get().to(scans::get_scans))
        .route("/{id}", web::delete().to(health_check::healthy))
}
