use crate::handlers::details;
use actix_web::{web, Scope};
use {cities::*, countries::*, districts::*, states::*, streets::*};

mod cities;
mod countries;
mod districts;
mod states;
mod streets;

pub fn load_details_routes() -> Scope {
    web::scope("/details")
        .route("/", web::post().to(details::create))
        .route("/{id}", web::put().to(details::update))
        .route("/{id}", web::delete().to(details::delete))
        .service(load_countries_routes())
        .service(load_districts_routes())
        .service(load_streets_routes())
        .service(load_states_routes())
        .service(load_cities_routes())
}
