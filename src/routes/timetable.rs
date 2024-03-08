use actix_web::{web, Scope};

use crate::handlers::timetable;

pub fn load_timetable_routes() -> Scope {
    web::scope("/timetable")
        .route("/all", web::post().to(timetable::list_timetable))
        .route("/activity", web::post().to(timetable::create_activity))
        .route("/event", web::post().to(timetable::create_event))
        .route("/lecture", web::post().to(timetable::create_lecture))
        .route("/{id}", web::delete().to(timetable::delete_timetable_item))
}
