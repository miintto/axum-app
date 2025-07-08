pub mod config;
mod core;
mod dto;
mod entity;
mod repository;
mod route;
mod service;

use axum::{Router, routing::get};
use sea_orm::DatabaseConnection;
use tracing::info;

use config::db::init_db;
use route::{
    auth::get_router as get_auth_router,
    user::get_router as get_user_router,
};

pub async fn app() -> Router {
    let db: DatabaseConnection = init_db().await;
    info!("Connect Database!");

    Router::new()
        .route("/", get(|| async move { "ok" }))
        .nest("/auth", get_auth_router(&db))
        .nest("/users", get_user_router(&db))
}
