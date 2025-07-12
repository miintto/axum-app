use axum::{Extension, Router, extract::Path, routing::get};
use sea_orm::DatabaseConnection;

use crate::core::{
    error::ApiError,
    http::Http2xx,
    permission::{AdminOnly, Authenticated},
    response::ApiResponse,
    validate::ValidJson,
};
use crate::dto::user::{UpdateUser, UserResponse};
use crate::repository::user::UserRepository;
use crate::service::user::UserService;

pub fn get_router(db: &DatabaseConnection) -> Router {
    let service = UserService::new(UserRepository::new(&db));

    Router::new()
        .route("/", get(get_user_list))
        .route(
            "/{id}",
            get(get_user)
                .patch(update_user_info),
        )
        .route(
            "/me",
            get(get_my_info)
                .patch(update_my_info),
        )
        .layer(Extension(service))
}

async fn get_user_list(
    _: AdminOnly,
    Extension(service): Extension<UserService<UserRepository>>,
) -> Result<ApiResponse<Vec<UserResponse>>, ApiError> {
    let users = service.get_user_list().await?;
    Ok(ApiResponse::new(Http2xx::Ok, users))
}

async fn get_user(
    _: AdminOnly,
    Extension(service): Extension<UserService<UserRepository>>,
    Path(id): Path<i32>,
) -> Result<ApiResponse<UserResponse>, ApiError> {
    let user = service.get_user(id).await?;
    Ok(ApiResponse::new(Http2xx::Ok, user))
}

async fn update_user_info(
    _: AdminOnly,
    Extension(service): Extension<UserService<UserRepository>>,
    Path(id): Path<i32>,
    ValidJson(body): ValidJson<UpdateUser>,
) -> Result<ApiResponse<UserResponse>, ApiError> {
    let user = service.update_user(id, body).await?;
    Ok(ApiResponse::new(Http2xx::Ok, user))
}

async fn get_my_info(
    permission: Authenticated,
    Extension(service): Extension<UserService<UserRepository>>,
) -> Result<ApiResponse<UserResponse>, ApiError> {
    let user = service.get_user(permission.claims.user_id).await?;
    Ok(ApiResponse::new(Http2xx::Ok, user))
}

async fn update_my_info(
    permission: Authenticated,
    Extension(service): Extension<UserService<UserRepository>>,
    ValidJson(body): ValidJson<UpdateUser>,
) -> Result<ApiResponse<UserResponse>, ApiError> {
    let user = service.update_user(permission.claims.user_id, body).await?;
    Ok(ApiResponse::new(Http2xx::Ok, user))
}
