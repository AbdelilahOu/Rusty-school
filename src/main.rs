use actix_web::{middleware::Logger, web, App, HttpServer};
use db::db::establish_connection;
use routes::students::load_students_routes;
use sea_orm::DatabaseConnection;

mod db;
mod handlers;
mod routes;

pub struct AppState {
    db_conn: DatabaseConnection,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let conn_res = establish_connection().await;
    match conn_res {
        Ok(conn) => {
            let _ = HttpServer::new(move || {
                App::new()
                    .app_data(web::Data::new(AppState {
                        db_conn: conn.clone(),
                    }))
                    .wrap(Logger::default())
                    .service(load_students_routes())
            })
            .bind(("127.0.0.1", 8080))?
            .run()
            .await;
        }
        Err(db_err) => panic!("{}", db_err.to_string()),
    };

    Ok(())
}
