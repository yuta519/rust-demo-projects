use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(root));
    let address = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(address, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}
