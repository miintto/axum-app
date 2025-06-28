use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use sea_orm::DatabaseConnection;
use serde_json::{Value, json, to_value};

use crate::repository::user::find_by_id;

pub async fn get_user(
    State(conn): State<DatabaseConnection>,
    Path(id): Path<i32>,
) -> (StatusCode, Json<Value>) {
    match find_by_id(&conn, id).await
    {
        Some(user) => (StatusCode::OK, Json(to_value(user).unwrap())),
        None => (StatusCode::NOT_FOUND, Json(json!({"error": "User not Found"}))),
    }
}
