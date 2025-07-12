use axum::{Extension, Router, routing::post};
use sea_orm::DatabaseConnection;

use crate::core::{
    error::ApiError,
    http::Http2xx,
    response::ApiResponse,
    validate::ValidJson,
};
use crate::dto::auth::{LoginUser, RegisterUser};
use crate::repository::user::UserRepository;
use crate::service::auth::AuthService;

pub fn get_router(db: &DatabaseConnection) -> Router {
    let service = AuthService::new(UserRepository::new(&db));

    Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
        .layer(Extension(service))
}

async fn login(
    Extension(service): Extension<AuthService<UserRepository>>,
    ValidJson(body): ValidJson<LoginUser>,
) -> Result<ApiResponse<String>, ApiError> {
    let token = service.login(body).await?;
    Ok(ApiResponse::new(Http2xx::Ok, token))
}

async fn register(
    Extension(service): Extension<AuthService<UserRepository>>,
    ValidJson(body): ValidJson<RegisterUser>,
) -> Result<ApiResponse<String>, ApiError> {
    let token = service.register(body).await?;
    Ok(ApiResponse::new(Http2xx::Created, token))
}
