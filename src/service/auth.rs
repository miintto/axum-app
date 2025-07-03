use axum::{Json, extract::State};
use sea_orm::DatabaseConnection;
use serde::Deserialize;

use crate::core::{http::{Http2xx, Http4xx}, jwt::encode_jwt, response::ApiResponse};
use crate::repository::user::{create_user, find_by_email};

#[derive(Debug, Deserialize)]
pub struct LoginUser {
    email: String,
    password: String,
}

#[derive(Debug, Deserialize)]
pub struct RegisterUser {
    name: String,
    email: String,
    password: String,
    password_check: String,
}

pub async fn login(
    State(conn): State<DatabaseConnection>,
    Json(body): Json<LoginUser>,
) -> Result<ApiResponse<String>, Http4xx> {
    let user = find_by_email(&conn, &body.email)
        .await
        .ok_or_else(||Http4xx::AuthenticationFail)?;
    if !bcrypt::verify(body.password, &user.hashed_password).unwrap() {
        return Err(Http4xx::AuthenticationFail)
    }
    Ok(ApiResponse::new(Http2xx::Ok, encode_jwt(user.id, &user.email)))
}

pub async fn register(
    State(conn): State<DatabaseConnection>,
    Json(body): Json<RegisterUser>,
) -> Result<ApiResponse<String>, Http4xx> {
    if body.password != body.password_check {
        return Err(Http4xx::PasswordMismatched);
    } else if let Some(_) = find_by_email(&conn, &body.email).await {
        return Err(Http4xx::DuplicatedEmail);
    }
    let user = create_user(
        &conn,
        &body.name,
        &body.email,
        bcrypt::hash(&body.password, 10).unwrap()
    ).await;
    Ok(ApiResponse::new(Http2xx::Created, encode_jwt(user.id, &user.email)))
}
