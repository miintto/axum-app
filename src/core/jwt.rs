use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
use serde::{Serialize, Deserialize};

use crate::config::settings::SECRET_KEY;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    user_id: i32,
    email: String,
    exp: usize,
    iat: usize,
}

pub fn encode_jwt(user_id: i32, email: &str) -> String {
    let now = Utc::now();
    let claims = Claims {
        user_id: user_id,
        email: email.to_string(),
        exp: (now + Duration::hours(12)).timestamp() as usize,
        iat: now.timestamp() as usize
    };
    encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(SECRET_KEY.as_bytes()),
    ).unwrap()
}
