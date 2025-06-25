use axum::{
    Json,
    extract::Path,
    http::StatusCode,
};

#[derive(serde::Serialize)]
pub struct User {
    id: i32,
    name: &'static str,
}

pub async fn get_user(Path(id): Path<i32>) -> (StatusCode, Json<User>) {
    (
        StatusCode::OK,
        Json(
            User { id: id, name: "username" }
        ),
    )
}
