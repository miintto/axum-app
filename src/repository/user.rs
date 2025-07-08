use chrono::Utc;
use sea_orm::{
    ActiveModelTrait,
    ActiveValue,
    ColumnTrait,
    DatabaseConnection,
    EntityTrait,
    QueryFilter,
    QueryOrder,
};

use crate::{core::http::Http4xx, entity::{prelude::User, user::{ActiveModel, Column, Model}}};

pub trait UserRepositoryPort: Send + Sync {
    async fn find_all(&self) -> Vec<Model>;

    async fn find_by_id(&self, id: i32) -> Option<Model>;

    async fn find_by_email(&self, email: &String) -> Option<Model>;

    async fn create_user(
        &self,
        name: &String,
        email: &String,
        hashed_password: String,
    ) -> Result<Model, Http4xx>;
}

#[derive(Clone)]
pub struct UserRepository {
    db: DatabaseConnection,
}

impl UserRepository {
    pub fn new(db: &DatabaseConnection) -> Self {
        Self { db: db.clone() }
    }
}

impl UserRepositoryPort for UserRepository {
    async fn find_all(&self) -> Vec<Model> {
        User::find()
            .order_by_asc(Column::Id)
            .all(&self.db)
            .await
            .unwrap()
    }

    async fn find_by_id(&self, id: i32) -> Option<Model> {
        User::find()
            .filter(Column::Id.eq(id))
            .one(&self.db)
            .await
            .unwrap()
    }

    async fn find_by_email(&self, email: &String) -> Option<Model> {
        User::find()
            .filter(Column::Email.eq(email))
            .one(&self.db)
            .await
            .unwrap()
    }

    async fn create_user(
        &self,
        name: &String,
        email: &String,
        hashed_password: String,
    ) -> Result<Model, Http4xx> {
        let user = ActiveModel {
            id: ActiveValue::NotSet,
            name: ActiveValue::Set(name.to_string()),
            email: ActiveValue::Set(email.to_string()),
            hashed_password: ActiveValue::Set(hashed_password.to_string()),
            is_active: ActiveValue::Set(true),
            is_admin: ActiveValue::Set(false),
            created_dtm: ActiveValue::Set(Utc::now().naive_utc()),
        };
        match user.insert(&self.db)
            .await
        {
            Ok(model) => Ok(model),
            Err(_) => Err(Http4xx::BadRequest)
        }
    }
}
