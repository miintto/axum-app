use sea_orm::{
    ColumnTrait,
    DatabaseConnection,
    EntityTrait,
    QueryFilter, QueryOrder,
};

use crate::entity::{prelude::User, user::{Column, Model}};

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
