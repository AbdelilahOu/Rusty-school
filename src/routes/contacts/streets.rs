use actix_web::{web, Scope};

pub fn load_streets_routes() -> Scope {
    web::scope("/streets")
}
