use dotenv::dotenv;
use entity::*;
use sea_orm::{Database, DatabaseConnection};

pub struct DB {
    pool: DatabaseConnection,
}

impl DB {
    pub async fn init() -> Result<Self, sea_orm::DbErr> {
        dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        Database::connect(database_url)
            .await
            .map(|pool| DB { pool })
    }

    pub fn get_connection(&self) -> &DatabaseConnection {
        &self.pool
    }
}
