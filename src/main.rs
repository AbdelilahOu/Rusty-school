use actix_web::{middleware::Logger, web, App, HttpServer};
use db::db::establish_connection;
use routes::students::load_students_routes;
use sea_orm::DatabaseConnection;

mod db;
mod handlers;
mod routes;

struct AppState {
    db_conn: DatabaseConnection,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let conn_res = establish_connection().await;
    match conn_res {
        Ok(conn) => {
            let _ = HttpServer::new(|| {
                App::new()
                    // .app_data(web::Data::new(AppState { db_conn: conn }))
                    .wrap(Logger::default())
                    .wrap(Logger::new("%a %{User-Agent}i"))
                    .service(load_students_routes())
            })
            .bind(("127.0.0.1", 8080))?
            .run()
            .await;
        }
        Err(db_err) => println!("{}", db_err.to_string()),
    }
    Ok(())
}
