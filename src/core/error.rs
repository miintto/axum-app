use axum::{Json, response::{IntoResponse, Response}};
use serde_json::json;

use crate::core::http::HttpCode;

pub struct ApiError<T: HttpCode> {
    pub http: T,
}

impl<T: HttpCode> ApiError<T> {
    pub fn new(http: T) -> Self{
        Self { http }
    }
}

impl<T: HttpCode> IntoResponse for ApiError<T> {
    fn into_response(self) -> Response {
        let message = json!({
            "code": self.http.code(),
            "message": self.http.message(),
            "data": null,
        });
        (self.http.status(), Json(message)).into_response()
    }
}
