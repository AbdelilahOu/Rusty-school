use actix_web::{web, Scope};

use crate::handlers::academic::timetable;

pub fn load_timetable_routes() -> Scope {
    web::scope("/timetable")
        .route("/", web::get().to(timetable::list))
        .route("/activity", web::post().to(timetable::create_activity))
        .route("/event", web::post().to(timetable::create_event))
        .route("/lecture", web::post().to(timetable::create_lecture))
        .route("/{id}", web::delete().to(timetable::delete_timetable_item))
}
