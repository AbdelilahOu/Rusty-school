use actix_web::{web, App, HttpServer};
use sea_orm::DatabaseConnection;

mod database;
mod handlers;
mod middlewares;
mod models;
mod routes;
mod utils;

use database::db::establish_connection;
use routes::{
    auth::load_auth_routes, contacts::load_contacts_routes, parents::load_parents_routes,
    scans::load_scans_routes, students::load_students_routes, teachers::load_teachers_routes,
};

pub struct AppState {
    db_conn: DatabaseConnection,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let conn_res = establish_connection().await;
    match conn_res {
        Ok(conn) => {
            // start server
            let _ = HttpServer::new(move || {
                App::new()
                    .app_data(web::Data::new(AppState {
                        db_conn: conn.clone(),
                    }))
                    .service(load_students_routes())
                    .service(load_teachers_routes())
                    .service(load_contacts_routes())
                    .service(load_parents_routes())
                    .service(load_scans_routes())
                    .service(load_auth_routes())
            })
            .bind(("127.0.0.1", 8080))?
            .run()
            .await;
        }
        Err(db_err) => {
            panic!("{}", db_err.to_string())
        }
    };

    Ok(())
}
