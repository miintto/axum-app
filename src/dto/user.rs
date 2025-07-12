use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::entity::user::Model;
use crate::repository::user::UserUpdateCommand;

#[derive(Debug, Deserialize)]
pub struct UpdateUser {
    pub name: Option<String>,
    pub email: Option<String>,
}

impl From<UpdateUser> for UserUpdateCommand {
    fn from(data: UpdateUser) -> Self {
        UserUpdateCommand {
            name: data.name,
            email: data.email,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    id: i32,
    name: String,
    email: String,
    is_active: bool,
    updated_dtm: Option<NaiveDateTime>,
    created_dtm: NaiveDateTime,
}

impl From<Model> for UserResponse {
    fn from(user: Model) -> Self {
        Self {
            id: user.id,
            name: user.name,
            email: user.email,
            is_active: user.is_active,
            updated_dtm: user.updated_dtm,
            created_dtm: user.created_dtm,
        }
    }
}
