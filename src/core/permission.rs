use axum::{extract::FromRequestParts, http::request::Parts};

use crate::core::{authentication::Authentication, error::ApiError, jwt::Claims};

pub type AdminOnly = ClaimsWrapper<2>;

pub type Authenticated = ClaimsWrapper<1>;

pub struct ClaimsWrapper<const LEVEL: i8> {
    pub claims: Claims,
}

impl<S, const LEVEL: i8> FromRequestParts<S> for ClaimsWrapper<LEVEL>
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Authentication(claims) = Authentication::from_request_parts(parts, state).await?;
        if claims.permission >= LEVEL {
            Ok(ClaimsWrapper { claims })
        } else {
            Err(ApiError::PermissionDenied)
        }
    }
}
