use sea_orm::{EnumIter, Iterable};
use sea_orm_migration::{
    prelude::*,
    sea_query::extension::postgres::{TypeCreateStatement, TypeDropStatement},
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Users::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Users::Name).string().not_null())
                    .col(ColumnDef::new(Users::LastPaymentId).integer().not_null())
                    .col(ColumnDef::new(Users::TgId).integer().not_null())
                    .col(
                        ColumnDef::new(Users::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Authors::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Authors::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Authors::Name).string().not_null())
                    .col(ColumnDef::new(Authors::Surname).string().not_null())
                    .col(ColumnDef::new(Authors::Bio).text().not_null())
                    .to_owned(),
            )
            .await?;
        let mut type_statement = TypeCreateStatement::new();
        let type_statement = type_statement.as_enum(Genre::Type).values([
            Genre::Biology,
            Genre::Fiction,
            Genre::IT,
            Genre::NonFiction,
        ]);
        manager.create_type(type_statement.to_owned()).await?;
        manager
            .create_table(
                Table::create()
                    .table(Books::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Books::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Books::AuthorId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-book-author-id")
                            .from(Books::Table, Books::AuthorId)
                            .to(Authors::Table, Authors::Id),
                    )
                    .col(ColumnDef::new(Books::Title).string().not_null())
                    .col(ColumnDef::new(Books::BookPath).string().not_null())
                    .col(ColumnDef::new(Books::Rating).integer().not_null())
                    .col(ColumnDef::new(Books::ShortView).text().not_null())
                    .col(
                        ColumnDef::new(Books::Genre)
                            .enumeration(Genre::Type, Genre::iter().skip(1))
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Books::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Pages::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Pages::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Pages::BookId).integer().not_null())
                    .col(ColumnDef::new(Pages::Chapter).integer().not_null())
                    .col(ColumnDef::new(Pages::Pages).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-pages-book-id")
                            .from(Pages::Table, Pages::BookId)
                            .to(Books::Table, Books::Id),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Reads::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Reads::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Reads::BookId).integer().not_null())
                    .col(ColumnDef::new(Reads::UserId).integer().not_null())
                    .col(ColumnDef::new(Reads::Page).integer().not_null())
                    .col(ColumnDef::new(Reads::Symbol).integer().not_null())
                    .col(ColumnDef::new(Reads::AmountToSend).integer().not_null())
                    .col(ColumnDef::new(Reads::TimeToSend).date_time().not_null())
                    .col(ColumnDef::new(Reads::Finished).boolean().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-reads-book-id")
                            .from(Reads::Table, Reads::BookId)
                            .to(Books::Table, Books::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-reads-user-id")
                            .from(Reads::Table, Reads::UserId)
                            .to(Users::Table, Users::Id),
                    )
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_foreign_key(
                ForeignKeyDropStatement::new()
                    .name("fk-reads-book-id")
                    .table(Reads::Table)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_foreign_key(
                ForeignKeyDropStatement::new()
                    .name("fk-reads-user-id")
                    .table(Reads::Table)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_foreign_key(
                ForeignKeyDropStatement::new()
                    .name("fk-pages-book-id")
                    .table(Pages::Table)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_foreign_key(
                ForeignKeyDropStatement::new()
                    .name("fk-book-author-id")
                    .table(Books::Table)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(Table::drop().table(Reads::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Pages::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Books::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Authors::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await?;
        let mut type_drop = TypeDropStatement::new();
        manager
            .drop_type(type_drop.name(Genre::Type).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
    LastPaymentId,
    TgId,
    Name,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Authors {
    Table,
    Id,
    Bio,
    Name,
    Surname,
}

#[derive(DeriveIden)]
enum Books {
    Table,
    Id,
    AuthorId,
    Title,
    ShortView,
    CreatedAt,
    Genre,
    BookPath,
    Rating,
}

#[derive(DeriveIden)]
enum Pages {
    Table,
    Id,
    BookId,
    Chapter,
    Pages,
}

#[derive(DeriveIden)]
enum Reads {
    Table,
    Id,
    BookId,
    UserId,
    Page,
    Symbol,
    TimeToSend,
    AmountToSend,
    Finished,
}

#[derive(Iden, EnumIter)]
pub enum Genre {
    Type,
    Fiction,
    NonFiction,
    Biology,
    IT,
}
