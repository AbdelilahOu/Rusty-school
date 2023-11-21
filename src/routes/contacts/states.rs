use actix_web::{web, Scope};

pub fn load_states_routes() -> Scope {
    web::scope("/states")
}
