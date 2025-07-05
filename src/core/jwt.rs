use chrono::{Duration, Utc};
use jsonwebtoken::{
    Algorithm,
    DecodingKey,
    EncodingKey,
    Header,
    TokenData,
    Validation,
    decode,
    encode,
};
use serde::{Serialize, Deserialize};

use crate::{config::settings::SECRET_KEY, core::http::Http4xx};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: i32,
    email: String,
    pub permission: i8,
    exp: usize,
    iat: usize,
}

pub fn encode_jwt(user_id: i32, email: &str, permission_level: i8) -> String {
    let now = Utc::now();
    let claims = Claims {
        user_id: user_id,
        email: email.to_string(),
        permission: permission_level,
        exp: (now + Duration::hours(12)).timestamp() as usize,
        iat: now.timestamp() as usize
    };
    encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(SECRET_KEY.as_bytes()),
    ).unwrap()
}

pub fn decode_jwt(token: &str) -> Result<TokenData<Claims>, Http4xx>{
    decode(
        &token,
        &DecodingKey::from_secret(SECRET_KEY.as_bytes()),
        &Validation::new(Algorithm::HS256),
    )
    .map_err(|_| Http4xx::Unauthenticated)
}
