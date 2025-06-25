mod api;
mod entity;

use axum::{
    routing::{get},
    Router,
};

use api::user::{get_user};

#[tokio::main]
async fn main() {
    let user_router = Router::new()
        .route("/{id}", get(get_user));

    let app = Router::new()
        .route("/", get(|| async move { "ok" }))
        .nest("/users", user_router);

    let listner = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();
    axum::serve(listner, app).await.unwrap();
}
