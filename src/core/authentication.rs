use axum::{extract::FromRequestParts, http::request::Parts};

use crate::core::{error::ApiError, jwt::{Claims, decode_jwt}};

pub struct Authentication(pub Claims);

impl<S> FromRequestParts<S> for Authentication
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get("Authorization")
            .ok_or(ApiError::Unauthenticated)?;
        
        let token = auth_header.to_str()
            .map_err(|_| ApiError::Unauthenticated)?
            .strip_prefix("Bearer ")
            .ok_or(ApiError::Unauthenticated)?;

        let token_data = decode_jwt(token)?;
        Ok(Authentication(token_data.claims))
    }
}
