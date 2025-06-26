use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::entity::user::{Column, Entity, Model};

pub async fn get_user(
    State(conn): State<DatabaseConnection>,
    Path(id): Path<i32>,
) -> (StatusCode, Json<Model>) {
    let user: Model = Entity::find()
        .filter(Column::Id.eq(id))
        .one(&conn)
        .await
        .unwrap()
        .unwrap();
    (
        StatusCode::OK,
        Json(user),
    )
}
