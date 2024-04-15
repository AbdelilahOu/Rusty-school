use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer};
use service::sea_orm::DatabaseConnection;
use types::config::Config;

mod config;
mod database;
mod guards;
mod handlers;
mod routes;
mod types;
mod utils;

use crate::routes::{academic, auth, communication, people};

pub struct AppState {
    db_conn: DatabaseConnection,
    config: Config,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // load config
    let loaded_config = config::load_config();
    //
    let conn = database::establish_connection(loaded_config.db_url.clone())
        .await
        .unwrap();
    //
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    // start server
    println!("Server started on http://0.0.0.0:8080");
    let _ = HttpServer::new(move || {
        App::new()
            .wrap(Logger::new("> %r status: [%s] took: %T s"))
            .app_data(web::Data::new(AppState {
                db_conn: conn.clone(),
                config: loaded_config.clone(),
            }))
            .service(auth::load_auth_routes())
            .service(people::load_students_routes())
            .service(people::load_teachers_routes())
            .service(people::load_parents_routes())
            .service(academic::load_levels_routes())
            .service(academic::load_scans_routes())
            .service(academic::load_subjects_routes())
            .service(academic::load_groups_routes())
            .service(academic::load_rooms_routes())
            .service(academic::load_classes_routes())
            .service(academic::load_attendance_routes())
            .service(academic::load_timetable_routes())
            .service(academic::load_assignments_routes())
            .service(academic::load_grades_routes())
            .service(academic::load_grading_rubric_routes())
            .service(academic::load_disciplinary_actions_routes())
            .service(communication::load_announcements_routes())
            .default_service(web::to(|| HttpResponse::NotFound()))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await;
    Ok(())
}
