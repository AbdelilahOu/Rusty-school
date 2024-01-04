// use crate::handlers::attendance;
use actix_web::{web, Scope};

use crate::handlers::health_check;

pub fn load_attendance_routes() -> Scope {
    web::scope("/attendance").route("/", web::get().to(health_check::healthy))
}
