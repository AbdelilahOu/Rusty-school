use actix_web::{middleware::Logger, App, HttpServer};
use routes::students::load_students_routes;

mod handlers;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(load_students_routes())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
