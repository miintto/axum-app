use crate::core::{error::ApiError, jwt::encode_jwt};
use crate::dto::auth::{LoginUser, RegisterUser};
use crate::repository::user::UserRepositoryPort;

#[derive(Clone)]
pub struct AuthService<R: UserRepositoryPort> {
    user_repo: R,
}

impl<R: UserRepositoryPort> AuthService<R> {
    pub fn new(user_repo: R) -> Self {
        Self { user_repo }
    }

    pub async fn login(&self, data: LoginUser) -> Result<String, ApiError> {
        let user = self.user_repo.find_by_email(&data.email)
            .await?
            .ok_or_else(|| ApiError::AuthenticationFail)?;
        if !bcrypt::verify(data.password, &user.hashed_password).unwrap() {
            return Err(ApiError::AuthenticationFail)
        }
        Ok(encode_jwt(user.id, &user.email, self.get_permission_level(user.is_admin)))
    }

    pub async fn register(&self, data: RegisterUser) -> Result<String, ApiError> {
        if data.password != data.password_check {
            return Err(ApiError::PasswordMismatched);
        } else if let Some(_) = self.user_repo.find_by_email(&data.email).await? {
            return Err(ApiError::DuplicatedEmail);
        }
        let user = self.user_repo.create_user(data.into()).await?;
        Ok(encode_jwt(user.id, &user.email, self.get_permission_level(user.is_admin)))
    }

    fn get_permission_level(&self, is_admin: bool) -> i8 {
        match is_admin {
            true => 2,
            false => 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use mockall::mock;
    use crate::entity::user::Model;
    use crate::repository::user::{UserCreateCommand, UserUpdateCommand};
    use super::*;

    mock! {
        UserRepository {}

        impl UserRepositoryPort for UserRepository {
            async fn find_all(&self) -> Result<Vec<Model>, ApiError>;
            async fn find_by_id(&self, id: i32) -> Result<Option<Model>, ApiError>;
            async fn find_by_email(&self, email: &String) -> Result<Option<Model>, ApiError>;
            async fn create_user(&self, command: UserCreateCommand) -> Result<Model, ApiError>;
            async fn update_user(&self, user: Model, data: UserUpdateCommand) -> Result<Model, ApiError>;
        }
    }

    fn generate_user(password: &String) -> Model {
        Model {
            id: 1,
            name: "name".to_string(),
            email: "test@example.com".to_string(),
            hashed_password: bcrypt::hash(&password, 10).unwrap(),
            is_active: true,
            is_admin: false,
            updated_dtm: None,
            created_dtm: Utc::now().naive_utc(),
        }        
    }

    #[tokio::test]
    async fn login_success() {
        let password = "password123";
        let user = generate_user(&password.to_string());
        let mut mock_repo = MockUserRepository::new();
        mock_repo.expect_find_by_email()
            .returning(move |_| Ok(Some(user.clone())));
        let service = AuthService::new(mock_repo);

        let req = LoginUser {
            email: "test@example.com".to_string(),
            password: password.to_string(),
        };
        let result = service.login(req).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn login_fail_with_invalid_email() {
        let mut mock_repo = MockUserRepository::new();
        mock_repo.expect_find_by_email()
            .returning(|_| Ok(None));
        let service = AuthService::new(mock_repo);

        let req = LoginUser {
            email: "test@example.com".to_string(),
            password: "password".to_string(),
        };
        let result = service.login(req).await;

        assert!(matches!(result, Err(ApiError::AuthenticationFail)));
    }

    #[tokio::test]
    async fn login_fail_with_invalid_password() {
        let password = "password";
        let user = generate_user(&password.to_string());
        let mut mock_repo = MockUserRepository::new();
        mock_repo.expect_find_by_email()
            .returning(move |_| Ok(Some(user.clone())));
        let service = AuthService::new(mock_repo);

        let req = LoginUser {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        };
        let result = service.login(req).await;

        assert!(matches!(result, Err(ApiError::AuthenticationFail)));
    }

    #[tokio::test]
    async fn register_success() {
        let password = "password";
        let user = generate_user(&password.to_string());
        let mut mock_repo = MockUserRepository::new();
        mock_repo.expect_find_by_email()
            .returning(move |_| Ok(None));
        mock_repo.expect_create_user()
            .returning(move |_| Ok(user.clone()));
        let service = AuthService::new(mock_repo);

        let req = RegisterUser {
            name: "name".to_string(),
            email: "test@example.com".to_string(),
            password: password.to_string(),
            password_check: password.to_string(),
        };
        let result = service.register(req).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn register_fail_with_mismatched_password() {
        let password = "password";
        let password_check = "password_check";
        let mock_repo = MockUserRepository::new();
        let service = AuthService::new(mock_repo);

        let req = RegisterUser {
            name: "name".to_string(),
            email: "test@example.com".to_string(),
            password: password.to_string(),
            password_check: password_check.to_string(),
        };
        let result = service.register(req).await;

        assert!(matches!(result, Err(ApiError::PasswordMismatched)));
    }

    #[tokio::test]
    async fn register_fail_with_duplicated_email() {
        let password = "password";
        let user = generate_user(&password.to_string());
        let mut mock_repo = MockUserRepository::new();
        mock_repo.expect_find_by_email()
            .returning(move |_| Ok(Some(user.clone())));
        let service = AuthService::new(mock_repo);

        let req = RegisterUser {
            name: "name".to_string(),
            email: "test@example.com".to_string(),
            password: password.to_string(),
            password_check: password.to_string(),
        };
        let result = service.register(req).await;

        assert!(matches!(result, Err(ApiError::DuplicatedEmail)));
    }
}
