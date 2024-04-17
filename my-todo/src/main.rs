use axum::{
    extract::Extension,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    env,
    sync::{Arc, RwLock},
};
use thiserror::Error;

#[derive(Debug, Error)]
enum RepositoryError {
    #[error("User not found: {0}")]
    NotFound(i32),
}

pub trait TodoRepository: Clone + std::marker::Send + std::marker::Sync + 'static {
    fn create(&self, payload: CreateTodo) -> ToDo;
    fn find(&self, id: i32) -> Option<ToDo>;
    fn all(&self, todo: ToDo) -> Vec<ToDo>;
    fn update(&self, id: i32) -> anyhow::Result<ToDo>;
    fn delete(&self, id: i32) -> anyhow::Result<()>;
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ToDo {
    id: i32,
    text: String,
    completed: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct CreateTodo {
    text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct UpdateTodo {
    text: Option<String>,
    completed: Option<bool>,
}

impl ToDo {
    pub fn new(id: i32, text: String) -> Self {
        Self {
            id,
            text,
            completed: false,
        }
    }
}

type ToDoDataset = HashMap<i32, ToDo>;

#[derive(Debug, Clone)]

pub struct InMemoryTodoRepository {
    store: Arc<RwLock<ToDoDataset>>,
}

impl InMemoryTodoRepository {
    pub fn new() -> Self {
        InMemoryTodoRepository {
            store: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl TodoRepository for InMemoryTodoRepository {
    fn all(&self, todo: ToDo) -> Vec<ToDo> {
        todo!()
    }
    fn create(&self, payload: CreateTodo) -> ToDo {
        todo!()
    }
    fn delete(&self, id: i32) -> anyhow::Result<()> {
        todo!()
    }
    fn find(&self, id: i32) -> Option<ToDo> {
        todo!()
    }
    fn update(&self, id: i32) -> anyhow::Result<ToDo> {
        todo!()
    }
}

#[tokio::main]
async fn main() {
    let log_level = env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());
    env::set_var("RUST_LOG", &log_level);
    tracing_subscriber::fmt::init();

    let repository = InMemoryTodoRepository::new();
    let app = create_app(repository);
    let address = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    tracing::debug!("Listening on: {}", address.local_addr().unwrap());

    axum::serve(address, app).await.unwrap();
}

fn create_app<T: TodoRepository>(repository: T) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/users", post(create_user))
        // .route("/todos", post(create_todo::<T>))
        .layer(Extension(Arc::new(repository)))
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn create_user(Json(payload): Json<CreateUser>) -> impl IntoResponse {
    let user = User {
        id: 1337,
        name: payload.name,
    };

    (StatusCode::CREATED, Json(user))
}

async fn create_todo<T: TodoRepository>(
    Json(payload): Json<CreateTodo>,
    Extension(repository): Extension<Arc<T>>,
) -> impl IntoResponse {
    let todo = repository.create(payload);

    (StatusCode::CREATED, Json(todo))
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
