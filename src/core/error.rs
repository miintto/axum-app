use axum::{http::StatusCode, response::{IntoResponse, Response}};

use crate::core::{http::HttpCode, response::ApiResponse};

#[derive(Debug)]
pub enum ApiError {
    BadRequest,
    Unauthenticated,
    PermissionDenied,
    InvalidParameter,
    UserNotFound,
    PasswordMismatched,
    DuplicatedEmail,
    AuthenticationFail,
    ServerError,
}

impl HttpCode for ApiError {
    fn status(&self) -> StatusCode {
        match self {
            ApiError::BadRequest => StatusCode::BAD_REQUEST,
            ApiError::Unauthenticated => StatusCode::UNAUTHORIZED,
            ApiError::PermissionDenied => StatusCode::FORBIDDEN,
            ApiError::InvalidParameter => StatusCode::UNPROCESSABLE_ENTITY,
            ApiError::UserNotFound => StatusCode::NOT_FOUND,
            ApiError::PasswordMismatched => StatusCode::NOT_FOUND,
            ApiError::DuplicatedEmail => StatusCode::NOT_FOUND,
            ApiError::AuthenticationFail => StatusCode::UNPROCESSABLE_ENTITY,
            ApiError::ServerError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn code(&self) -> &'static str {
        match self {
            ApiError::BadRequest => "F001",
            ApiError::Unauthenticated => "F002",
            ApiError::PermissionDenied => "F003",
            ApiError::InvalidParameter => "F004",
            ApiError::UserNotFound => "F005",
            ApiError::PasswordMismatched => "F006",
            ApiError::DuplicatedEmail => "F007",
            ApiError::AuthenticationFail => "F008",
            ApiError::ServerError => "E001",
        }
    }

    fn message(&self) -> &'static str {
        match self {
            ApiError::BadRequest => "잘못된 요청",
            ApiError::Unauthenticated => "인증 실패",
            ApiError::PermissionDenied => "권한이 없습니다",
            ApiError::InvalidParameter => "파라미터 에러",
            ApiError::UserNotFound => "사용자를 찾을 수 없습니다",
            ApiError::PasswordMismatched => "패스워드가 서로 일치하지 않습니다",
            ApiError::DuplicatedEmail => "이미 사용중인 이메일입니다",
            ApiError::AuthenticationFail => "이메일 혹은 비밀번호가 일치하지 않습니다",
            ApiError::ServerError => "서버 에러",
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        ApiResponse::new(self, ()).into_response()
    }
}
