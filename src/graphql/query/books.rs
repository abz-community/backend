use crate::database::DB;
use async_graphql;
use async_graphql::{Context, Object, Result};
use entity::books;
use sea_orm::EntityTrait;

#[derive(Default)]
pub struct BooksQuery;

#[Object]
impl BooksQuery {
    async fn get_books(&self, ctx: &Context<'_>) -> Result<Vec<books::Model>> {
        let db = ctx.data::<DB>().unwrap();

        Ok(books::Entity::find()
            .all(db.get_connection())
            .await
            .map_err(|e| e.to_string())?)
    }

    async fn get_book_by_id(&self, ctx: &Context<'_>, id: i32) -> Result<Option<books::Model>> {
        let db = ctx.data::<DB>().unwrap();

        Ok(books::Entity::find_by_id(id)
            .one(db.get_connection())
            .await
            .map_err(|e| e.to_string())?)
    }

    // TODO add book list by genre, by user preferences, mb add some kind of shuffle
}
