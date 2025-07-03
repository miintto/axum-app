use chrono::NaiveDateTime;
use serde::{Serialize};

use crate::entity::user::Model;

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub is_active: bool,
    pub created_dtm: NaiveDateTime,
}

impl From<Model> for UserResponse {
    fn from(user: Model) -> Self {
        Self {
            id: user.id,
            name: user.name,
            email: user.email,
            is_active: user.is_active,
            created_dtm: user.created_dtm,
        }
    }
}
