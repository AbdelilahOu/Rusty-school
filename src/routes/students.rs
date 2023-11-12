use crate::handlers::health;
use actix_web::{web, Scope};

pub fn load_students_routes() -> Scope {
    web::scope("/students").route("/", web::get().to(health::health_check))
}
