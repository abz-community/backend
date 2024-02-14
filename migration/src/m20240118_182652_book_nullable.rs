use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        // todo!();

        manager
            .alter_table(
                Table::alter()
                    .table(Reads::Table)
                    .add_column(ColumnDef::new(Reads::SendTimeout).integer().not_null())
                    .modify_column(ColumnDef::new(Reads::TimeToSend).time().not_null())
                    .drop_column(Reads::Page)
                    .add_column(ColumnDef::new(Reads::Chapter).integer())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .alter_table(
                Table::alter()
                    .table(Reads::Table)
                    .drop_column(Reads::SendTimeout)
                    .modify_column(ColumnDef::new(Reads::TimeToSend).date_time().not_null())
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Reads {
    Table,
    Id,
    BookId,
    UserId,
    Chapter,
    Page,
    Symbol,
    TimeToSend,
    SendTimeout,
    AmountToSend,
    Finished,
}
