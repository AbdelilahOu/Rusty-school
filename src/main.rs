use axum::{routing::get, Router};
use std::net::SocketAddr;
use tokio::signal;

// locale
mod handlers;
mod routes;
use handlers::health::health_check;

use crate::routes::students::load_students_routes;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(health_check()))
        .nest("/students", load_students_routes());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    println!("->> LISTENING on {addr}\n");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("signal received, starting graceful shutdown");
}
