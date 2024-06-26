use crate::handlers::auth;
use actix_web::{web, Scope};

pub fn load_auth_routes() -> Scope {
    web::scope("/auth")
        .route("/login", web::get().to(auth::login))
        .route("/sessions/google", web::get().to(auth::google_auth))
        .route("/renew-access", web::post().to(auth::renew_access_token))
}
