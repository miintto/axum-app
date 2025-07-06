use axum::{Extension, Router, extract::{Path, State}, routing::get};
use sea_orm::DatabaseConnection;

use crate::core::{
    http::{Http2xx, Http4xx},
    permission::{AdminOnly, Authenticated},
    response::ApiResponse,
};
use crate::dto::user::UserResponse;
use crate::repository::user::UserRepository;
use crate::service::user::UserService;

pub fn get_router() -> Router<DatabaseConnection> {
    let service: UserService = UserService::new(UserRepository::new());

    Router::new()
        .route("/", get(get_user_list))
        .route("/{id}", get(get_user))
        .route("/me", get(get_my_info))
        .layer(Extension(service))
}

async fn get_user_list(
    AdminOnly(_): AdminOnly,
    Extension(service): Extension<UserService>,
    State(conn): State<DatabaseConnection>,
) -> Result<ApiResponse<Vec<UserResponse>>, Http4xx> {
    let users = service.get_user_list(conn).await;
    Ok(ApiResponse::new(Http2xx::Ok, users))
}

async fn get_user(
    AdminOnly(_): AdminOnly,
    Extension(service): Extension<UserService>,
    State(conn): State<DatabaseConnection>,
    Path(id): Path<i32>,
) -> Result<ApiResponse<UserResponse>, Http4xx> {
    let user = service.get_user(conn, id).await?;
    Ok(ApiResponse::new(Http2xx::Ok, user))
}

async fn get_my_info(
    Authenticated(claims): Authenticated,
    Extension(service): Extension<UserService>,
    State(conn): State<DatabaseConnection>,
) -> Result<ApiResponse<UserResponse>, Http4xx> {
    let user = service.get_user(conn, claims.user_id).await?;
    Ok(ApiResponse::new(Http2xx::Ok, user))
}
