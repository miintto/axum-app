use axum::{
    Json,
    extract::Path,
    http::StatusCode,
};
use sea_orm::{ColumnTrait, Condition, Database, EntityTrait, QueryFilter};

use crate::entity::user::{Column, Entity, Model};

const DATABASE_URL: &str = "postgresql://postgres:password@localhost:5432/db";

pub async fn get_user(Path(id): Path<i32>) -> (StatusCode, Json<Model>) {
    let conn = Database::connect(DATABASE_URL).await.unwrap();
    let mut condition = Condition::any();
    condition = condition.add(Column::Id.eq(id));

    let user = Entity::find()
        .filter(condition)
        .one(&conn)
        .await
        .unwrap()
        .unwrap();
    (
        StatusCode::OK,
        Json(user),
    )
}
