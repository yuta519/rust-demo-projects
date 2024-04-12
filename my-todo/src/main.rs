use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::env;
use thiserror::Error;

#[derive(Debug, Error)]
enum RepositoryError {
    #[error("User not found: {0}")]
    NotFound(i32),
}

pub trait TodoRepository: Clone + std::marker::Send + std::marker::Sync + 'static {
    fn create(&self, todo: ToDo) -> Result<ToDo, RepositoryError>;
    fn find(&self, id: i32) -> Result<ToDo, RepositoryError>;
    fn all(&self, todo: ToDo) -> Result<ToDo, RepositoryError>;
    fn update(&self, id: i32) -> Result<ToDo, RepositoryError>;
    fn delete(&self, id: i32) -> Result<ToDo, RepositoryError>;
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ToDo {
    id: i32,
    text: String,
    completed: bool,
}

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

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
struct CreateUszer {
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
        http::{header, Method, Request},
    };
    use tower::ServiceExt;

    #[tokio::test]
    async fn should_return_hello_world() {
        let req = Request::builder()
            .method(Method::GET)
            .uri("/")
            .body(Body::empty())
            .unwrap();

        let res = create_app().oneshot(req).await.unwrap();
        let bytes = axum::body::to_bytes(res.into_body(), usize::MAX)
            .await
            .unwrap();
        let body = String::from_utf8(bytes.to_vec()).unwrap();
        assert_eq!(body, "Hello, World!");
    }

    #[tokio::test]
    async fn should_return_user_data() {
        let req = Request::builder()
            .method(Method::POST)
            .uri("/users")
            .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::from(r#"{"name":"Alice"}"#))
            .unwrap();

        let res = create_app().oneshot(req).await.unwrap();
        let bytes = axum::body::to_bytes(res.into_body(), usize::MAX)
            .await
            .unwrap();
        let body = String::from_utf8(bytes.to_vec()).unwrap();
        let user: User = serde_json::from_str(&body).expect("Cannot convert to User Instance");

        assert_eq!(
            user,
            User {
                id: 1337,
                name: "Alice".to_string(),
            }
        );
    }
}
