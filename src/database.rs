use dotenv::dotenv;
use entity::*;
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};

#[derive(Debug)]
pub struct DB {
    pool: DatabaseConnection,
}

impl DB {
    pub async fn init() -> Result<Self, sea_orm::DbErr> {
        dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = Database::connect(database_url).await;
        println!("{:?}", pool);
        let pool = pool?;
        // Migrator::up(&pool, None).await.map_err(|e| {
        //     println!("{}", e);
        //     e
        // })?;
        Ok(DB { pool })
    }

    pub fn get_connection(&self) -> &DatabaseConnection {
        &self.pool
    }
}
