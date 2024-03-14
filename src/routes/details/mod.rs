mod cities;
mod countries;
mod districts;
mod states;
mod streets;

use crate::handlers::details;
use actix_web::{web, Scope};
use {cities::*, countries::*, districts::*, states::*, streets::*};

pub fn load_details_routes() -> Scope {
    web::scope("/details")
        .route("/", web::post().to(details::create_details))
        .route("/{id}", web::put().to(details::update_details))
        .route("/{id}", web::delete().to(details::delete_details))
        .service(load_countries_routes())
        .service(load_districts_routes())
        .service(load_streets_routes())
        .service(load_states_routes())
        .service(load_cities_routes())
}
