use actix_web::{web, Scope};

pub fn load_levels_routes() -> Scope {
    web::scope("/levels")
}
