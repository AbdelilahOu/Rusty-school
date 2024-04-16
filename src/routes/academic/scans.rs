use actix_web::{web, Scope};

use crate::handlers::academic::scans;

pub fn load_scans_routes() -> Scope {
    web::scope("/scans")
        .route("/", web::get().to(scans::list))
        .route("/", web::post().to(scans::create))
}
