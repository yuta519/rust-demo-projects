use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::env;

#[tokio::main]
async fn main() {
    let log_level = env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());
    env::set_var("RUST_LOG", &log_level);
    tracing_subscriber::fmt::init();

    let app = create_app();
    let address = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    tracing::debug!("Listening on: {}", address.local_addr().unwrap());

    axum::serve(address, app).await.unwrap();
}

fn create_app() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/users", post(create_user))
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn create_user(Json(payload): Json<CreateUszer>) -> impl IntoResponse {
    let user = User {
        id: 1337,
        name: payload.name,
    };

    (StatusCode::CREATED, Json(user))
}

#[derive(Deserialize)]
struct CreateUszer {
    name: String,
}

#[derive(Serialize)]
struct User {
    id: u64,
    name: String,
}
