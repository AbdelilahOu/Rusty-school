use actix_web::{web, Scope};

use crate::handlers::contacts;

use super::{cities::*, countries::*, districts::*, states::*, streets::*};

pub fn load_contacts_routes() -> Scope {
    web::scope("/contacts")
        .route("/", web::post().to(contacts::create_contact))
        .route("/{id}", web::get().to(contacts::get_contact))
        .route("/{id}", web::put().to(contacts::update_contact))
        .route("/filter", web::post().to(contacts::get_contacts))
        .route("/{id}", web::delete().to(contacts::delete_contact))
        .service(load_countries_routes())
        .service(load_districts_routes())
        .service(load_streets_routes())
        .service(load_states_routes())
        .service(load_cities_routes())
}
