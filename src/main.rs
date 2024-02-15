use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer};
use models::commen::ConfigObj;
use service::sea_orm::DatabaseConnection;

mod config;
mod database;
mod guards;
mod handlers;
mod models;
mod routes;
mod utils;

use crate::routes::{
    attendance::load_attendance_routes, auth::load_auth_routes, classes::load_classes_routes,
    details::load_details_routes, groups::load_groups_routes, levels::load_levels_routes,
    parents::load_parents_routes, rooms::load_rooms_routes, scans::load_scans_routes,
    students::load_students_routes, subjects::load_subjects_routes, teachers::load_teachers_routes,
    timetable::load_timetable_routes,
};

use database::{establish_connection, run_migrations};

pub struct AppState {
    db_conn: DatabaseConnection,
    env: ConfigObj,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // load config
    let loaded_config = config::load_config();
    //
    let conn = establish_connection(loaded_config.db_url.clone())
        .await
        .unwrap();
    // run migrations
    let _ = run_migrations(&conn).await.unwrap();
    //
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    // start server
    println!("Server started on http://0.0.0.0:8080");
    let _ = HttpServer::new(move || {
        App::new()
            .wrap(Logger::new("> %r status: [%s] took: %T s"))
            .app_data(web::Data::new(AppState {
                db_conn: conn.clone(),
                env: loaded_config.clone(),
            }))
            .route("/", web::get().to(handlers::health_check::healthy))
            .service(load_students_routes())
            .service(load_teachers_routes())
            .service(load_details_routes())
            .service(load_parents_routes())
            .service(load_levels_routes())
            .service(load_scans_routes())
            .service(load_auth_routes())
            .service(load_subjects_routes())
            .service(load_groups_routes())
            .service(load_rooms_routes())
            .service(load_classes_routes())
            .service(load_attendance_routes())
            .service(load_timetable_routes())
            .default_service(web::to(|| HttpResponse::NotFound()))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await;
    Ok(())
}
