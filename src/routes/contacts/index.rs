use actix_web::{web, Scope};

use crate::handlers::contacts::index;

use super::{
    cities::load_cities_routes, countries::load_countries_routes, districts::load_districts_routes,
    states::load_states_routes, streets::load_streets_routes,
};

pub fn load_contacts_routes() -> Scope {
    web::scope("/contacts")
        .service(load_countries_routes())
        .service(load_states_routes())
        .service(load_cities_routes())
        .service(load_districts_routes())
        .service(load_streets_routes())
}
