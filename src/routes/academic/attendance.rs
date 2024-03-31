use actix_web::{web, Scope};

use crate::handlers::academic::attendance;

pub fn load_attendance_routes() -> Scope {
    web::scope("/attendance").route("/", web::get().to(attendance::list))
}
