// use crate::handlers::auth;
use actix_web::{web, Scope};

use crate::handlers::health_check;

pub fn load_auth_routes() -> Scope {
    web::scope("/")
        .route("/login", web::post().to(health_check::healthy))
        .route("/register", web::post().to(health_check::healthy))
        .route("/callback", web::delete().to(health_check::healthy))
}
