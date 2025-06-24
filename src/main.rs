use axum::{
    routing::{get},
    Router,
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async move { "ok" }));

    let listner = tokio::net::TcpListener::bind("127.0.0.1:8000").await.unwrap();
    axum::serve(listner, app).await.unwrap();
}
