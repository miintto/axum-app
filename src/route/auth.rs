use axum::{extract::State, routing::post, Extension, Router};
use sea_orm::DatabaseConnection;

use crate::core::{
    http::{Http2xx, Http4xx},
    response::ApiResponse,
    validate::Json,
};
use crate::dto::auth::{LoginUser, RegisterUser};
use crate::repository::user::UserRepository;
use crate::service::auth::AuthService;

pub fn get_router() -> Router<DatabaseConnection> {
    let service = AuthService::new(UserRepository::new());

    Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
        .layer(Extension(service))
}

async fn login(
    Extension(service): Extension<AuthService>,
    State(conn): State<DatabaseConnection>,
    Json(body): Json<LoginUser>,
) -> Result<ApiResponse<String>, Http4xx> {
    let token = service.login(conn, body).await?;
    Ok(ApiResponse::new(Http2xx::Ok, token))
}

async fn register(
    Extension(service): Extension<AuthService>,
    State(conn): State<DatabaseConnection>,
    Json(body): Json<RegisterUser>,
) -> Result<ApiResponse<String>, Http4xx> {
    let token = service.register(conn, body).await?;
    Ok(ApiResponse::new(Http2xx::Created, token))
}
