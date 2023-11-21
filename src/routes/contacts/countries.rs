use actix_web::{web, Scope};

pub fn load_countries_routes() -> Scope {
    web::scope("/countries")
}
