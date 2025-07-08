use crate::core::http::Http4xx;
use crate::dto::user::UserResponse;
use crate::repository::user::UserRepositoryPort;

#[derive(Clone)]
pub struct UserService<R: UserRepositoryPort> {
    user_repo: R,
}

impl<R: UserRepositoryPort> UserService<R> {
    pub fn new(user_repo: R) -> Self {
        Self { user_repo }
    }

    pub async fn get_user_list(&self) -> Vec<UserResponse> {
        let users = self.user_repo.find_all().await;
        users.into_iter().map(UserResponse::from).collect()
    }

    pub async fn get_user(&self, id: i32) -> Result<UserResponse, Http4xx> {
        match self.user_repo.find_by_id(id).await
        {
            Some(user) => Ok(UserResponse::from(user)),
            None => Err(Http4xx::UserNotFound),
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use mockall::mock;
    use crate::entity::user::Model;
    use super::*;

    mock! {
        UserRepository {}

        impl UserRepositoryPort for UserRepository {
            async fn find_all(&self) -> Vec<Model>;
            async fn find_by_id(&self, id: i32) -> Option<Model>;
            async fn find_by_email(&self, email: &String) -> Option<Model>;
            async fn create_user(&self, name: &String, email: &String, hashed_password: String) -> Result<Model, Http4xx>;
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
            created_dtm: Utc::now().naive_utc(),
        }        
    }

    #[tokio::test]
    async fn find_all_user() {
        let mut mock = MockUserRepository::new();
        mock.expect_find_all()
            .returning(move || vec![generate_user(), generate_user(), generate_user()]);
        let service = UserService::new(mock);

        let result = service.get_user_list().await;

        assert!(result.len() == 3);
    }

    #[tokio::test]
    async fn get_user() {
        let mut mock = MockUserRepository::new();
        mock.expect_find_by_id()
            .returning(move |_| Some(generate_user()));
        let service = UserService::new(mock);

        let result = service.get_user(1).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn user_not_found() {
        let mut mock = MockUserRepository::new();
        mock.expect_find_by_id()
            .returning(move |_| None);
        let service = UserService::new(mock);

        let result = service.get_user(1).await;

        assert!(matches!(result, Err(Http4xx::UserNotFound)));
    }
}
