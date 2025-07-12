use crate::core::error::ApiError;
use crate::dto::user::{UpdateUser, UserResponse};
use crate::repository::user::UserRepositoryPort;

#[derive(Clone)]
pub struct UserService<R: UserRepositoryPort> {
    user_repo: R,
}

impl<R: UserRepositoryPort> UserService<R> {
    pub fn new(user_repo: R) -> Self {
        Self { user_repo }
    }

    pub async fn get_user_list(&self) -> Result<Vec<UserResponse>, ApiError> {
        let users = self.user_repo.find_all().await?;
        Ok(users.into_iter().map(UserResponse::from).collect())
    }

    pub async fn get_user(&self, id: i32) -> Result<UserResponse, ApiError> {
        let user = self.user_repo.find_by_id(id)
            .await?
            .ok_or_else(|| ApiError::UserNotFound)?;
        Ok(user.into())
    }

    pub async fn update_user(&self, id: i32, data: UpdateUser) -> Result<UserResponse, ApiError> {
        let user = self.user_repo.find_by_id(id)
            .await?
            .ok_or_else(|| ApiError::UserNotFound)?;
        let updated_user = self.user_repo.update_user(user, data.into()).await?;
        Ok(updated_user.into())
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

    fn generate_user() -> Model {
        Model {
            id: 1,
            name: "name".to_string(),
            email: "test@example.com".to_string(),
            hashed_password: "password".to_string(),
            is_active: true,
            is_admin: false,
            updated_dtm: None,
            created_dtm: Utc::now().naive_utc(),
        }        
    }

    #[tokio::test]
    async fn find_all_user() {
        let mut mock_repo = MockUserRepository::new();
        mock_repo.expect_find_all()
            .returning(move || Ok(vec![generate_user(), generate_user(), generate_user()]));
        let service = UserService::new(mock_repo);

        let result = service.get_user_list().await.unwrap();

        assert!(result.len() == 3);
    }

    #[tokio::test]
    async fn get_user() {
        let mut mock_repo = MockUserRepository::new();
        mock_repo.expect_find_by_id()
            .returning(move |_| Ok(Some(generate_user())));
        let service = UserService::new(mock_repo);

        let result = service.get_user(1).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn user_not_found() {
        let mut mock_repo = MockUserRepository::new();
        mock_repo.expect_find_by_id()
            .returning(move |_| Ok(None));
        let service = UserService::new(mock_repo);

        let result = service.get_user(1).await;

        assert!(matches!(result, Err(ApiError::UserNotFound)));
    }

    #[tokio::test]
    async fn update_success() {
        let mut mock_repo = MockUserRepository::new();
        mock_repo.expect_find_by_id()
            .returning(move |_| Ok(Some(generate_user())));
        mock_repo.expect_update_user()
            .returning(move |_, _| Ok(generate_user()));
        let service = UserService::new(mock_repo);

        let req = UpdateUser {
            name: Some("name".to_string()),
            email: Some("test@example.com".to_string()),
        };
        let result = service.update_user(1, req).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn update_user_not_found() {
        let mut mock_repo = MockUserRepository::new();
        mock_repo.expect_find_by_id()
            .returning(move |_| Ok(None));
        let service = UserService::new(mock_repo);

        let req = UpdateUser {
            name: Some("name".to_string()),
            email: Some("test@example.com".to_string()),
        };
        let result = service.update_user(1, req).await;

        assert!(matches!(result, Err(ApiError::UserNotFound)));
    }
}
