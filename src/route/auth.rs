use axum::Extension;
use sea_orm::DatabaseConnection;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::core::{
    error::ApiError,
    http::Http2xx,
    response::{ApiResponse, ResponseSchema},
    validate::ValidJson,
};
use crate::dto::auth::{LoginUser, RegisterUser};
use crate::repository::user::UserRepository;
use crate::service::auth::AuthService;

pub fn get_router(db: &DatabaseConnection) -> OpenApiRouter {
    let service = AuthService::new(UserRepository::new(&db));

    OpenApiRouter::new()
        .routes(routes!(login))
        .routes(routes!(register))
        .layer(Extension(service))
}

#[utoipa::path(
    post,
    path = "/login",
    request_body = LoginUser,
    responses(
        (
            status = OK,
            body = ResponseSchema<String>,
            description = "성공",
            example = json!({"code": "S001", "message": "성공", "data": "eyJ0eXAi..."}),
        ),
        (
            status = UNPROCESSABLE_ENTITY,
            body = ResponseSchema<String>,
            description = "파라미터 에러",
            example = json!({"code": "F008", "message": "이메일 혹은 비밀번호가 일치하지 않습니다.", "data": null}),
        ),
    ),
    summary = "로그인",
    tag = "Auth",
)]
async fn login(
    Extension(service): Extension<AuthService<UserRepository>>,
    ValidJson(body): ValidJson<LoginUser>,
) -> Result<ApiResponse<String>, ApiError> {
    let token = service.login(body).await?;
    Ok(ApiResponse::new(Http2xx::Ok, token))
}

#[utoipa::path(
    post,
    path = "/register",
    request_body = RegisterUser,
    responses(
        (
            status = OK,
            body = ResponseSchema<String>,
            description = "성공",
            example = json!({"code": "S002", "message": "생성 완료", "data": "eyJ0eXAi..."}),
        ),
        (
            status = UNPROCESSABLE_ENTITY,
            body = ResponseSchema<String>,
            description = "파라미터 에러",
            example = json!({"code": "F006", "message": "패스워드가 서로 일치하지 않습니다.", "data": null}),
        ),
    ),
    summary = "회원가입",
    tag = "Auth",
)]
async fn register(
    Extension(service): Extension<AuthService<UserRepository>>,
    ValidJson(body): ValidJson<RegisterUser>,
) -> Result<ApiResponse<String>, ApiError> {
    let token = service.register(body).await?;
    Ok(ApiResponse::new(Http2xx::Created, token))
}
