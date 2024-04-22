mod handlers;
mod repositories;

use axum::{
    extract::Extension,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::{env, sync::Arc};

use handlers::create_todo;
use repositories::{InMemoryTodoRepository, TodoRepository};

#[tokio::main]
async fn main() {
    let log_level = env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());
    env::set_var("RUST_LOG", &log_level);
    tracing_subscriber::fmt::init();

    let repository = InMemoryTodoRepository::new();
    let app = create_app(repository);
    let address = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();

    tracing::debug!("Listening on: {}", address.local_addr().unwrap());

    axum::serve(address, app).await.unwrap();
}

fn create_app<T: TodoRepository>(repository: T) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/todos", post(create_todo::<T>))
        .layer(Extension(Arc::new(repository)))
}

async fn root() -> &'static str {
    "Hello, World!"
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
struct CreateUser {
    name: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
struct User {
    id: u64,
    name: String,
}

#[cfg(test)]
mod test {
    use super::*;
    use axum::{
        body::Body,
        http::{Method, Request},
    };
    use tower::ServiceExt;

    #[tokio::test]
    async fn should_return_hello_world() {
        let req = Request::builder()
            .method(Method::GET)
            .uri("/")
            .body(Body::empty())
            .unwrap();

        let repository = InMemoryTodoRepository::new();
        let res = create_app(repository).oneshot(req).await.unwrap();
        let bytes = axum::body::to_bytes(res.into_body(), usize::MAX)
            .await
            .unwrap();
        let body = String::from_utf8(bytes.to_vec()).unwrap();
        assert_eq!(body, "Hello, World!");
    }
}
