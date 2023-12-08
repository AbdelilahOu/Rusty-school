// use crate::handlers::scans;
use actix_web::{web, Scope};

use crate::handlers::scans;

pub fn load_scans_routes() -> Scope {
    web::scope("/scans")
        .route("/", web::post().to(scans::create_scan))
        .route("/", web::get().to(scans::list_scans))
}
