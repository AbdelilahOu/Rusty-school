use crate::handlers::academic::attendance;
use actix_web::{web, Scope};

pub fn load_attendance_routes() -> Scope {
    web::scope("/attendance").route("/", web::get().to(attendance::list))
}
