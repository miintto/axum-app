use axum::{extract::FromRequestParts, http::request::Parts};

use crate::core::{http::Http4xx, jwt::{Claims, decode_jwt}};

pub struct Authentication(pub Claims);

impl<S> FromRequestParts<S> for Authentication
where
    S: Send + Sync,
{
    type Rejection = Http4xx;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get("Authorization")
            .ok_or(Http4xx::Unauthenticated)?;
        
        let token = auth_header.to_str()
            .map_err(|_| Http4xx::Unauthenticated)?
            .strip_prefix("Bearer ")
            .ok_or(Http4xx::Unauthenticated)?;

        let token_data = decode_jwt(token)?;
        Ok(Authentication(token_data.claims))
    }
}
