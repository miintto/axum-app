use axum::{extract::FromRequestParts, http::request::Parts};

use crate::core::{authentication::Authentication, http::Http4xx, jwt::Claims};

pub struct AdminOnly(pub Claims);

pub struct Authenticated(pub Claims);

impl<S> FromRequestParts<S> for AdminOnly
where
    S: Send + Sync,
{
    type Rejection = Http4xx;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Authentication(claims) = Authentication::from_request_parts(parts, state).await?;
        if claims.permission > 1 {
            Ok(AdminOnly(claims))
        } else {
            Err(Http4xx::PermissionDenied)
        }
    }
}

impl<S> FromRequestParts<S> for Authenticated
where
    S: Send + Sync,
{
    type Rejection = Http4xx;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Authentication(claims) = Authentication::from_request_parts(parts, state).await?;
        if claims.permission > 0 {
            Ok(Authenticated(claims))
        } else {
            Err(Http4xx::PermissionDenied)
        }
    }
}
