mod config;
mod core;
mod dto;
mod entity;
mod repository;
mod service;

use axum::{
    Router,
    routing::{get, post},
};
use sea_orm::DatabaseConnection;

use config::db::{init_db};
use service::{auth::{login, register}, user::{get_user, get_user_list}};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let conn: DatabaseConnection = init_db().await;

    let user_router = Router::new()
        .route("/", get(get_user_list))
        .route("/{id}", get(get_user));

    let app = Router::new()
        .route("/", get(|| async move { "ok" }))
        .route("/auth/login", post(login))
        .route("/auth/register", post(register))
        .nest("/users", user_router)
        .with_state(conn);

    let listner = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();
    axum::serve(listner, app).await.unwrap();
}
