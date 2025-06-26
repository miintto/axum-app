mod api;
mod config;
mod entity;

use axum::{
    Router,
    routing::{get},
};
use sea_orm::DatabaseConnection;

use api::user::{get_user};
use config::db::{init_db};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let conn: DatabaseConnection = init_db().await;

    let user_router = Router::new()
        .route("/{id}", get(get_user));

    let app = Router::new()
        .route("/", get(|| async move { "ok" }))
        .nest("/users", user_router)
        .with_state(conn);

    let listner = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();
    axum::serve(listner, app).await.unwrap();
}
