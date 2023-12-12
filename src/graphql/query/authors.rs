use crate::database::DB;
use async_graphql;
use async_graphql::{Context, Object, Result};
use entity::authors;
use sea_orm::EntityTrait;

#[derive(Default)]
pub struct AuthorQuery;

#[Object]
impl AuthorQuery {
    async fn get_authors(&self, ctx: &Context<'_>) -> Result<Vec<authors::Model>> {
        let db = ctx.data::<DB>().unwrap();

        Ok(authors::Entity::find()
            .all(db.get_connection())
            .await
            .map_err(|e| e.to_string())?)
    }

    async fn get_author_by_id(&self, ctx: &Context<'_>, id: i32) -> Result<Option<authors::Model>> {
        let db = ctx.data::<DB>().unwrap();

        Ok(authors::Entity::find_by_id(id)
            .one(db.get_connection())
            .await
            .map_err(|e| e.to_string())?)
    }
}
