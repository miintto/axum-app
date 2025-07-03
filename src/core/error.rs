use axum::response::{IntoResponse, Response};

use crate::core::{http::{Http4xx, Http5xx}, response::ApiResponse};

impl IntoResponse for Http4xx {
    fn into_response(self) -> Response {
        ApiResponse::new(self, ()).into_response()
    }
}

impl IntoResponse for Http5xx {
    fn into_response(self) -> Response {
        ApiResponse::new(self, ()).into_response()
    }
}
