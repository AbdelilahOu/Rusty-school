use actix_web::{web, Scope};

pub fn load_districts_routes() -> Scope {
    web::scope("/districts")
}
