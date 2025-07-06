mod config;
mod core;
mod dto;
mod entity;
mod repository;
mod route;
mod service;

use axum::{Router, routing::get};
use sea_orm::DatabaseConnection;
use tracing::info;

use config::{
    db::{init_db},
    logging::{layer::get_trace_layer, registry::init_logging},
    utils::handler_404,
};
use route::{
    auth::get_router as get_auth_router,
    user::get_router as get_user_router,
};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    init_logging().await;

    let conn: DatabaseConnection = init_db().await;
    info!("Connect Database!");

    let app = Router::new()
        .route("/", get(|| async move { "ok" }))
        .nest("/auth", get_auth_router())
        .nest("/users", get_user_router())
        .layer(get_trace_layer())
        .fallback(handler_404)
        .with_state(conn);

    let listner = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();
    axum::serve(listner, app).await.unwrap();
}
