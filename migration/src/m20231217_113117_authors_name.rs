use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Authors::Table)
                    .drop_column(Authors::Surname)
                    .modify_column(ColumnDef::new(Authors::Bio).text().null())
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Authors::Table)
                    .add_column(ColumnDef::new(Authors::Surname).string().null())
                    .modify_column(ColumnDef::new(Authors::Bio).text().not_null())
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Authors {
    Table,
    Id,
    Bio,
    Name,
    Surname,
}
