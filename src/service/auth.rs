use sea_orm::DatabaseConnection;

use crate::core::{http::Http4xx, jwt::encode_jwt};
use crate::dto::auth::{LoginUser, RegisterUser};
use crate::repository::user::UserRepository;

#[derive(Clone)]
pub struct AuthService {
    user_repo: UserRepository,
}

impl AuthService {
    pub fn new(user_repo: UserRepository) -> Self {
        Self { user_repo }
    }

    pub async fn login(
        &self,
        conn: DatabaseConnection,
        body: LoginUser,
    ) -> Result<String, Http4xx> {
        let user = self.user_repo.find_by_email(&conn, &body.email)
            .await
            .ok_or_else(||Http4xx::AuthenticationFail)?;
        if !bcrypt::verify(body.password, &user.hashed_password).unwrap() {
            return Err(Http4xx::AuthenticationFail)
        }
        Ok(encode_jwt(user.id, &user.email, self.get_permission_level(user.is_admin)))
    }

    pub async fn register(
        &self,
        conn: DatabaseConnection,
        body: RegisterUser,
    ) -> Result<String, Http4xx> {
        if body.password != body.password_check {
            return Err(Http4xx::PasswordMismatched);
        } else if let Some(_) = self.user_repo.find_by_email(&conn, &body.email).await {
            return Err(Http4xx::DuplicatedEmail);
        }
        let user = self.user_repo.create_user(
            &conn,
            &body.name,
            &body.email,
            bcrypt::hash(&body.password, 10).unwrap()
        ).await?;
        Ok(encode_jwt(user.id, &user.email, self.get_permission_level(user.is_admin)))
    }

    fn get_permission_level(&self, is_admin: bool) -> i8 {
        match is_admin {
            true => 2,
            false => 1,
        }
    }
}
