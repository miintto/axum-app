pub mod config;
mod core;
mod dto;
mod entity;
mod repository;
mod route;
mod service;

use axum::{Router, routing::get};
use sea_orm::DatabaseConnection;
use tracing::info;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_redoc::{Redoc, Servable};

use config::db::init_db;
use route::{
    auth::get_router as get_auth_router,
    user::get_router as get_user_router,
};

#[derive(OpenApi)]
#[openapi(
    tags(
        (name = "Auth", description = "인증"),
        (name = "User", description = "사용자 관련 작업"),
    ),
)]
struct ApiDoc;

pub async fn app() -> Router {
    let db: DatabaseConnection = init_db().await;
    info!("Connect Database!");

    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .route("/", get(|| async move { "ok" }))
        .nest("/auth", get_auth_router(&db))
        .nest("/users", get_user_router(&db))
        .split_for_parts();

    let router = router.merge(Redoc::with_url("/docs", api));
    router
}
