use axum::{Json, http::StatusCode, response::{IntoResponse, Response}};
use serde::Serialize;
use serde_json::json;

use crate::core::http::HttpCode;

pub struct ApiResponse<T> {
    status: StatusCode,
    code: String,
    message: String,
    data: T,
}

impl<T> ApiResponse<T> {
    pub fn new<C: HttpCode>(http: C, data: T) -> Self {
        Self {
            code: http.code().to_string(),
            status: http.status(),
            message: http.message().to_string(),
            data,
        }
    }
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        let message = json!({
            "code": self.code,
            "message": self.message,
            "data": self.data,
        });
        (self.status, Json(message)).into_response()
    }
}
