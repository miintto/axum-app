use serde::{Deserialize};

use crate::repository::user::UserCreateCommand;

#[derive(Debug, Deserialize)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct RegisterUser {
    pub name: String,
    pub email: String,
    pub password: String,
    pub password_check: String,
}

impl From<RegisterUser> for UserCreateCommand {
    fn from(data: RegisterUser) -> Self {
        UserCreateCommand {
            name: data.name,
            email: data.email,
            hashed_password: bcrypt::hash(&data.password, 10).unwrap(),
        }
    }
}
