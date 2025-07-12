use axum::{Extension, Router, extract::Path, routing::get};
use sea_orm::DatabaseConnection;

use crate::core::{
    http::{Http2xx, Http4xx},
    permission::{AdminOnly, Authenticated},
    response::ApiResponse,
    validate::Json,
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
    AdminOnly(_): AdminOnly,
    Extension(service): Extension<UserService<UserRepository>>,
) -> Result<ApiResponse<Vec<UserResponse>>, Http4xx> {
    let users = service.get_user_list().await;
    Ok(ApiResponse::new(Http2xx::Ok, users))
}

async fn get_user(
    AdminOnly(_): AdminOnly,
    Extension(service): Extension<UserService<UserRepository>>,
    Path(id): Path<i32>,
) -> Result<ApiResponse<UserResponse>, Http4xx> {
    let user = service.get_user(id).await?;
    Ok(ApiResponse::new(Http2xx::Ok, user))
}

async fn update_user_info(
    AdminOnly(_): AdminOnly,
    Extension(service): Extension<UserService<UserRepository>>,
    Path(id): Path<i32>,
    Json(body): Json<UpdateUser>,
) -> Result<ApiResponse<UserResponse>, Http4xx> {
    let user = service.update_user(id, body).await?;
    Ok(ApiResponse::new(Http2xx::Ok, user))
}

async fn get_my_info(
    Authenticated(claims): Authenticated,
    Extension(service): Extension<UserService<UserRepository>>,
) -> Result<ApiResponse<UserResponse>, Http4xx> {
    let user = service.get_user(claims.user_id).await?;
    Ok(ApiResponse::new(Http2xx::Ok, user))
}

async fn update_my_info(
    Authenticated(claims): Authenticated,
    Extension(service): Extension<UserService<UserRepository>>,
    Json(body): Json<UpdateUser>,
) -> Result<ApiResponse<UserResponse>, Http4xx> {
    let user = service.update_user(claims.user_id, body).await?;
    Ok(ApiResponse::new(Http2xx::Ok, user))
}
