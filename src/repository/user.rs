use chrono::Utc;
use sea_orm::{
    ActiveModelTrait,
    ActiveValue,
    ColumnTrait,
    DatabaseConnection,
    EntityTrait,
    QueryFilter, QueryOrder,
};

use crate::entity::{prelude::User, user::{ActiveModel, Column, Model}};

pub async fn find_all(
    conn: &DatabaseConnection,
) -> Vec<Model> {
    User::find()
        .order_by_asc(Column::Id)
        .all(conn)
        .await
        .unwrap()
}

pub async fn find_by_id(
    conn: &DatabaseConnection,
    id: i32,
) -> Option<Model> {
    User::find()
        .filter(Column::Id.eq(id))
        .one(conn)
        .await
        .unwrap()
}

pub async fn find_by_email(
    conn: &DatabaseConnection,
    email: &String,
) -> Option<Model> {
    User::find()
        .filter(Column::Email.eq(email))
        .one(conn)
        .await
        .unwrap()
}

pub async fn create_user(
    conn: &DatabaseConnection,
    name: &String,
    email: &String,
    hashed_password: String,
) -> Model {
    let user = ActiveModel {
        id: ActiveValue::NotSet,
        name: ActiveValue::Set(name.to_string()),
        email: ActiveValue::Set(email.to_string()),
        hashed_password: ActiveValue::Set(hashed_password.to_string()),
        is_active: ActiveValue::Set(true),
        is_admin: ActiveValue::Set(false),
        created_dtm: ActiveValue::Set(Utc::now().naive_utc()),
    };
    match user.insert(conn)
        .await
    {
        Ok(model) => model,
        Err(e) => panic!("{}", e),
    }
}
