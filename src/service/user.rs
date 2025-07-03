use axum::{extract::{Path, State}};
use sea_orm::DatabaseConnection;

use crate::core::{error::ApiError, http::{Http2xx, Http4xx}, response::ApiResponse};
use crate::entity::user::Model;
use crate::repository::user::{find_all, find_by_id};

pub async fn get_user_list(
    State(conn): State<DatabaseConnection>,
) -> ApiResponse<Vec<Model>> {
    let users = find_all(&conn).await;
    ApiResponse::new(Http2xx::Ok, users)
}

pub async fn get_user(
    State(conn): State<DatabaseConnection>,
    Path(id): Path<i32>,
) -> Result<ApiResponse<Model>, ApiError<Http4xx>> {
    match find_by_id(&conn, id).await
    {
        Some(user) => Ok(ApiResponse::new(Http2xx::Ok, user)),
        None => Err(ApiError::new(Http4xx::UserNotFound)),
    }
}
