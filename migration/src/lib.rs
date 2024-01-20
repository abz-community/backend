pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20231217_113117_authors_name;
mod m20240118_182652_book_nullable;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20231217_113117_authors_name::Migration),
            Box::new(m20240118_182652_book_nullable::Migration),
        ]
    }
}
