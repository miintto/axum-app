use axum::{extract::{Path, State}};
use sea_orm::DatabaseConnection;

use crate::core::{
    http::{Http2xx, Http4xx},
    permission::{AdminOnly, Authenticated},
    response::ApiResponse,
};
use crate::dto::user::UserResponse;
use crate::repository::user::{find_all, find_by_id};

pub async fn get_user_list(
    AdminOnly(_): AdminOnly,
    State(conn): State<DatabaseConnection>,
) -> ApiResponse<Vec<UserResponse>> {
    let users = find_all(&conn).await;
    ApiResponse::new(Http2xx::Ok, users.into_iter().map(UserResponse::from).collect())
}

pub async fn get_user(
    AdminOnly(_): AdminOnly,
    State(conn): State<DatabaseConnection>,
    Path(id): Path<i32>,
) -> Result<ApiResponse<UserResponse>, Http4xx> {
    match find_by_id(&conn, id).await
    {
        Some(user) => Ok(ApiResponse::new(Http2xx::Ok, UserResponse::from(user))),
        None => Err(Http4xx::UserNotFound),
    }
}

pub async fn get_my_info(
    Authenticated(claims): Authenticated,
    State(conn): State<DatabaseConnection>,
) -> Result<ApiResponse<UserResponse>, Http4xx> {
    match find_by_id(&conn, claims.user_id).await
    {
        Some(user) => Ok(ApiResponse::new(Http2xx::Ok, UserResponse::from(user))),
        None => Err(Http4xx::UserNotFound),
    }
}
