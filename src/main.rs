mod config;
mod core;
mod dto;
mod entity;
mod repository;
mod service;

use axum::{Router, routing::{get, post}};
use sea_orm::DatabaseConnection;
use tracing::info;

use config::{
    db::{init_db},
    logging::{layer::get_trace_layer, registry::init_logging},
};
use service::{
    auth::{login, register},
    user::{get_my_info, get_user, get_user_list},
};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    init_logging().await;

    let conn: DatabaseConnection = init_db().await;
    info!("Connect Database!");

    let user_router = Router::new()
        .route("/", get(get_user_list))
        .route("/{id}", get(get_user))
        .route("/me", get(get_my_info));

    let app = Router::new()
        .route("/", get(|| async move { "ok" }))
        .route("/auth/login", post(login))
        .route("/auth/register", post(register))
        .nest("/users", user_router)
        .layer(get_trace_layer())
        .with_state(conn);

    let listner = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();
    axum::serve(listner, app).await.unwrap();
}
