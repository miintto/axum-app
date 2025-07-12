use axum::{Extension, extract::Path};
use sea_orm::DatabaseConnection;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::core::{
    error::ApiError,
    http::Http2xx,
    permission::{AdminOnly, Authenticated},
    response::{ApiResponse, ResponseSchema},
    validate::ValidJson,
};
use crate::dto::user::{UpdateUser, UserResponse};
use crate::repository::user::UserRepository;
use crate::service::user::UserService;

pub fn get_router(db: &DatabaseConnection) -> OpenApiRouter {
    let service = UserService::new(UserRepository::new(&db));

    OpenApiRouter::new()
        .routes(routes!(get_user_list))
        .routes(routes!(get_user))
        .routes(routes!(update_user_info))
        .routes(routes!(get_my_info))
        .routes(routes!(update_my_info))
        .layer(Extension(service))
}

#[utoipa::path(
    get,
    path = "",
    responses(
        (
            status = OK,
            body = ResponseSchema<UserResponse>,
            description = "성공",
            example = json!({
                "code": "S001",
                "message": "성공",
                "data": [
                    {
                        "id": 1,
                        "name": "미민또",
                        "email": "miintto",
                        "is_active": true,
                        "updated_dtm": "2025-07-12T07:29:50.749618",
                        "created_dtm": "2025-04-12T07:03:20",
                    }
                ],
            }),
        ),
        (
            status = UNAUTHORIZED,
            body = ResponseSchema<String>,
            description = "인증 에러",
            example = json!({"code": "F002", "message": "인증 실패", "data": null}),
        ),
        (
            status = FORBIDDEN,
            body = ResponseSchema<String>,
            description = "권한 에러",
            example = json!({"code": "F003", "message": "권한이 없습니다", "data": null}),
        ),
    ),
    summary = "사용자 리스트 조회",
    tag = "User",
)]
async fn get_user_list(
    _: AdminOnly,
    Extension(service): Extension<UserService<UserRepository>>,
) -> Result<ApiResponse<Vec<UserResponse>>, ApiError> {
    let users = service.get_user_list().await?;
    Ok(ApiResponse::new(Http2xx::Ok, users))
}

#[utoipa::path(
    get,
    path = "/{id}",
    responses(
        (
            status = OK,
            body = ResponseSchema<UserResponse>,
            description = "성공",
            example = json!({
                "code": "S001",
                "message": "성공",
                "data": {
                    "id": 1,
                    "name": "미민또",
                    "email": "miintto",
                    "is_active": true,
                    "updated_dtm": "2025-07-12T07:29:50.749618",
                    "created_dtm": "2025-04-12T07:03:20",
                },
            }),
        ),
        (
            status = UNAUTHORIZED,
            body = ResponseSchema<String>,
            description = "인증 에러",
            example = json!({"code": "F002", "message": "인증 실패", "data": null}),
        ),
        (
            status = FORBIDDEN,
            body = ResponseSchema<String>,
            description = "권한 에러",
            example = json!({"code": "F003", "message": "권한이 없습니다", "data": null}),
        ),
        (
            status = NOT_FOUND,
            body = ResponseSchema<String>,
            description = "조회 에러",
            example = json!({"code": "F005", "message": "사용자를 찾을 수 없습니다", "data": null}),
        ),
    ),
    summary = "사용자 정보 조회",
    tag = "User",
)]
async fn get_user(
    _: AdminOnly,
    Extension(service): Extension<UserService<UserRepository>>,
    Path(id): Path<i32>,
) -> Result<ApiResponse<UserResponse>, ApiError> {
    let user = service.get_user(id).await?;
    Ok(ApiResponse::new(Http2xx::Ok, user))
}


#[utoipa::path(
    patch,
    path = "/{id}",
    request_body = UpdateUser,
    responses(
        (
            status = OK,
            body = ResponseSchema<UserResponse>,
            description = "성공",
            example = json!({
                "code": "S001",
                "message": "성공",
                "data": {
                    "id": 1,
                    "name": "미민또",
                    "email": "miintto",
                    "is_active": true,
                    "updated_dtm": "2025-07-12T07:29:50.749618",
                    "created_dtm": "2025-04-12T07:03:20",
                },
            }),
        ),
        (
            status = UNAUTHORIZED,
            body = ResponseSchema<String>,
            description = "인증 에러",
            example = json!({"code": "F002", "message": "인증 실패", "data": null}),
        ),
        (
            status = FORBIDDEN,
            body = ResponseSchema<String>,
            description = "권한 에러",
            example = json!({"code": "F003", "message": "권한이 없습니다", "data": null}),
        ),
        (
            status = NOT_FOUND,
            body = ResponseSchema<String>,
            description = "조회 에러",
            example = json!({"code": "F005", "message": "사용자를 찾을 수 없습니다", "data": null}),
        ),
    ),
    summary = "사용자 정보 수정",
    tag = "User",
)]
async fn update_user_info(
    _: AdminOnly,
    Extension(service): Extension<UserService<UserRepository>>,
    Path(id): Path<i32>,
    ValidJson(body): ValidJson<UpdateUser>,
) -> Result<ApiResponse<UserResponse>, ApiError> {
    let user = service.update_user(id, body).await?;
    Ok(ApiResponse::new(Http2xx::Ok, user))
}

#[utoipa::path(
    get,
    path = "/me",
    responses(
        (
            status = OK,
            body = ResponseSchema<String>,
            description = "성공",
            example = json!({
                "code": "S001",
                "message": "성공",
                "data": {
                    "id": 1,
                    "name": "미민또",
                    "email": "miintto",
                    "is_active": true,
                    "updated_dtm": "2025-07-12T07:29:50.749618",
                    "created_dtm": "2025-04-12T07:03:20",
                },
            }),
        ),
        (
            status = UNAUTHORIZED,
            body = ResponseSchema<String>,
            description = "인증 에러",
            example = json!({"code": "F002", "message": "인증 실패", "data": null}),
        ),
        (
            status = FORBIDDEN,
            body = ResponseSchema<String>,
            description = "권한 에러",
            example = json!({"code": "F003", "message": "권한이 없습니다", "data": null}),
        ),
        (
            status = NOT_FOUND,
            body = ResponseSchema<String>,
            description = "조회 에러",
            example = json!({"code": "F005", "message": "사용자를 찾을 수 없습니다", "data": null}),
        ),
    ),
    summary = "내 정보 조회",
    tag = "User",
)]

async fn get_my_info(
    permission: Authenticated,
    Extension(service): Extension<UserService<UserRepository>>,
) -> Result<ApiResponse<UserResponse>, ApiError> {
    let user = service.get_user(permission.claims.user_id).await?;
    Ok(ApiResponse::new(Http2xx::Ok, user))
}


#[utoipa::path(
    patch,
    path = "/me",
    request_body = UpdateUser,
    responses(
        (
            status = OK,
            body = ResponseSchema<UserResponse>,
            description = "성공",
            example = json!({
                "code": "S001",
                "message": "성공",
                "data": {
                    "id": 1,
                    "name": "미민또",
                    "email": "miintto",
                    "is_active": true,
                    "updated_dtm": "2025-07-12T07:29:50.749618",
                    "created_dtm": "2025-04-12T07:03:20",
                },
            }),
        ),
        (
            status = UNAUTHORIZED,
            body = ResponseSchema<String>,
            description = "인증 에러",
            example = json!({"code": "F002", "message": "인증 실패", "data": null}),
        ),
        (
            status = FORBIDDEN,
            body = ResponseSchema<String>,
            description = "권한 에러",
            example = json!({"code": "F003", "message": "권한이 없습니다", "data": null}),
        ),
        (
            status = NOT_FOUND,
            body = ResponseSchema<String>,
            description = "조회 에러",
            example = json!({"code": "F005", "message": "사용자를 찾을 수 없습니다", "data": null}),
        ),
    ),
    summary = "내 정보 수정",
    tag = "User",
)]
async fn update_my_info(
    permission: Authenticated,
    Extension(service): Extension<UserService<UserRepository>>,
    ValidJson(body): ValidJson<UpdateUser>,
) -> Result<ApiResponse<UserResponse>, ApiError> {
    let user = service.update_user(permission.claims.user_id, body).await?;
    Ok(ApiResponse::new(Http2xx::Ok, user))
}
