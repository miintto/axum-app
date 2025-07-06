use axum::{Router, routing::post};
use sea_orm::DatabaseConnection;

use crate::service::auth::{login, register};

pub fn get_router() -> Router<DatabaseConnection> {
    Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
}
