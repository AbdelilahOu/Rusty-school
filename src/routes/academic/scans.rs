use crate::handlers::academic::scans;
use actix_web::{web, Scope};

pub fn load_scans_routes() -> Scope {
    web::scope("/scans")
        .route("/", web::get().to(scans::list))
        .route("/", web::post().to(scans::create))
}
