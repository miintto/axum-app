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
use tracing::info;

use crate::{core::error::ApiError, entity::{prelude::User, user::{ActiveModel, Column, Model}}};

pub struct UserCreateCommand {
    pub name: String,
    pub email: String,
    pub hashed_password: String,
}

pub struct UserUpdateCommand {
    pub name: Option<String>,
    pub email: Option<String>,
}

pub trait UserRepositoryPort: Send + Sync {
    async fn find_all(&self) -> Result<Vec<Model>, ApiError>;

    async fn find_by_id(&self, id: i32) -> Result<Option<Model>, ApiError>;

    async fn find_by_email(&self, email: &String) -> Result<Option<Model>, ApiError>;

    async fn create_user(&self, command: UserCreateCommand) -> Result<Model, ApiError>;

    async fn update_user(&self, user: Model, command: UserUpdateCommand) -> Result<Model, ApiError>;
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
    async fn find_all(&self) -> Result<Vec<Model>, ApiError> {
        match User::find()
            .order_by_asc(Column::Id)
            .all(&self.db)
            .await
        {
            Ok(model) => Ok(model),
            Err(err) => {
                info!("Database Error : {}", err);
                Err(ApiError::ServerError)
            },
        }
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<Model>, ApiError> {
        match User::find()
            .filter(Column::Id.eq(id))
            .one(&self.db)
            .await
        {
            Ok(model) => Ok(model),
            Err(err) => {
                info!("Database Error : {}", err);
                Err(ApiError::ServerError)
            },
        }
    }

    async fn find_by_email(&self, email: &String) -> Result<Option<Model>, ApiError> {
        match User::find()
            .filter(Column::Email.eq(email))
            .one(&self.db)
            .await
        {
            Ok(model) => Ok(model),
            Err(err) => {
                info!("Database Error : {}", err);
                Err(ApiError::ServerError)
            },
        }
    }

    async fn create_user(&self, command: UserCreateCommand) -> Result<Model, ApiError>{
        let user = ActiveModel {
            id: ActiveValue::NotSet,
            name: ActiveValue::Set(command.name),
            email: ActiveValue::Set(command.email),
            hashed_password: ActiveValue::Set(command.hashed_password),
            is_active: ActiveValue::Set(true),
            is_admin: ActiveValue::Set(false),
            updated_dtm: ActiveValue::NotSet,
            created_dtm: ActiveValue::Set(Utc::now().naive_utc()),
        };
        match user.insert(&self.db)
            .await
        {
            Ok(model) => Ok(model),
            Err(err) => {
                info!("Database Error : {}", err);
                Err(ApiError::ServerError)
            },
        }
    }

    async fn update_user(&self, user: Model, command: UserUpdateCommand) -> Result<Model, ApiError> {
        let mut model: ActiveModel = user.into();
        if let Some(name) = command.name {
            model.name = ActiveValue::Set(name.to_string());
        }
        if let Some(email) = command.email {
            model.email = ActiveValue::Set(email.to_string());
        }
        model.updated_dtm = ActiveValue::Set(Some(Utc::now().naive_utc()));
        match model.update(&self.db).await {
            Ok(updated) => Ok(updated),
            Err(err) => {
                info!("Database Error : {}", err);
                Err(ApiError::ServerError)
            },
        }
    }
}
