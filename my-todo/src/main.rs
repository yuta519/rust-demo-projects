use axum::{routing::get, Router};
use std::env;

#[tokio::main]
async fn main() {
    let log_level = env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());
    env::set_var("RUST_LOG", &log_level);
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/", get(root));
    let address = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::debug!("Listening on: {}", address.local_addr().unwrap());

    axum::serve(address, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}
