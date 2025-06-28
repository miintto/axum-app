use sea_orm::{
    ColumnTrait,
    DatabaseConnection,
    EntityTrait,
    QueryFilter,
};

use crate::entity::{prelude::User, user::{Column, Model}};

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
