use actix_web::{web, Scope};

use crate::handlers::scans;

pub fn load_scans_routes() -> Scope {
    web::scope("/scans")
        .route("/", web::post().to(scans::create))
        .route("/", web::get().to(scans::list))
}
