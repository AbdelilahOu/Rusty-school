// use crate::handlers::attendance;
use actix_web::{web, Scope};

use crate::handlers::attendance;

pub fn load_attendance_routes() -> Scope {
    web::scope("/attendance").route("/", web::get().to(attendance::list_attendance))
}
