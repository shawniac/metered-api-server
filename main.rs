mod config;

use axum::{routing::get, Router, Server};
use std::net::SocketAddr;
use config::Config;

#[tokio::main]
async fn main() {
    let config = Config::from_env();
    let addr: SocketAddr = config.server_addr.parse().expect("Invalid server address");

    let app = Router::new()
        .route("/x", get(handler_x))
        .route("/y", get(handler_y))
        .route("/z", get(handler_z));

    println!("Listening on {}", addr);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler_x() -> &'static str {
    "This is the /x route."
}

async fn handler_y() -> &'static str {
    "This is the /y route."
}

async fn handler_z() -> &'static str {
    "This is the /z route."
}
