use sea_orm::{Database, DatabaseConnection};

use crate::config::settings::DATABASE_URL;

pub async fn init_db() -> DatabaseConnection {
    match Database::connect(DATABASE_URL.to_string()).await {
        Ok(db) => db,
        Err(e) => panic!("Error {}", e)
    }
}
