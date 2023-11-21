use actix_web::{web, Scope};

pub fn load_cities_routes() -> Scope {
    web::scope("/cities")
}
