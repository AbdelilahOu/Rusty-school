// use crate::handlers::auth;
use actix_web::{web, Scope};

use crate::handlers::auth;

pub fn load_auth_routes() -> Scope {
    web::scope("/auth")
        .route("/login", web::get().to(auth::login))
        .route("/sessions/google", web::get().to(auth::google_auth_handler))
}
