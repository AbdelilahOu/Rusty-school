use actix_web::{web, App, HttpResponse, HttpServer};
use models::commen::ConfigObj;
use sea_orm::DatabaseConnection;

mod config;
mod database;
mod guards;
mod handlers;
mod middlewares;
mod models;
mod routes;
mod utils;

use database::{establish_connection, run_migrations};
use routes::{
    auth::load_auth_routes, details::load_details_routes, levels::load_levels_routes,
    parents::load_parents_routes, scans::load_scans_routes, students::load_students_routes,
    teachers::load_teachers_routes,
};

pub struct AppState {
    db_conn: DatabaseConnection,
    env: ConfigObj,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let conn = establish_connection().await.unwrap();
    // run migrations
    let _ = run_migrations(&conn).await.unwrap();
    // start server
    let _ = HttpServer::new(move || {
        // load config
        let loaded_config = config::load_config();
        // start app
        App::new()
            .app_data(web::Data::new(AppState {
                db_conn: conn.clone(),
                env: loaded_config,
            }))
            .route("/", web::get().to(handlers::health_check::healthy))
            .service(load_students_routes())
            .service(load_teachers_routes())
            .service(load_details_routes())
            .service(load_parents_routes())
            .service(load_scans_routes())
            .service(load_auth_routes())
            .service(load_levels_routes())
            .default_service(web::to(|| HttpResponse::Unauthorized()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await;

    Ok(())
}
