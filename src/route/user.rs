use axum::{Router, routing::get};
use sea_orm::DatabaseConnection;

use crate::service::user::{get_my_info, get_user, get_user_list};

pub fn get_router() -> Router<DatabaseConnection> {
    Router::new()
        .route("/", get(get_user_list))
        .route("/{id}", get(get_user))
        .route("/me", get(get_my_info))
}
