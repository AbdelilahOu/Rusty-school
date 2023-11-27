// use crate::handlers::auth;
use actix_web::{web, Scope};

use crate::handlers::{auth, health_check};

pub fn load_auth_routes() -> Scope {
    web::scope("/auth")
        .route("/login", web::get().to(auth::login))
        .route("/register", web::post().to(health_check::healthy))
        .route("/sessions/google", web::get().to(auth::google_auth_handler))
}
