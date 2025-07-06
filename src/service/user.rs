use sea_orm::DatabaseConnection;

use crate::core::http::Http4xx;
use crate::dto::user::UserResponse;
use crate::repository::user::UserRepository;

#[derive(Clone)]
pub struct UserService {
    user_repo: UserRepository,
}

impl UserService {
    pub fn new(user_repo: UserRepository) -> Self {
        Self { user_repo }
    }

    pub async fn get_user_list(
        &self,
        conn: DatabaseConnection,
    ) -> Vec<UserResponse> {
        let users = self.user_repo.find_all(&conn).await;
        users.into_iter().map(UserResponse::from).collect()
    }

    pub async fn get_user(
        &self,
        conn: DatabaseConnection,
        id: i32,
    ) -> Result<UserResponse, Http4xx> {
        match self.user_repo.find_by_id(&conn, id).await
        {
            Some(user) => Ok(UserResponse::from(user)),
            None => Err(Http4xx::UserNotFound),
        }
    }
}
