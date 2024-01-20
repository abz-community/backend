use async_graphql::{Context, Object, Result, Upload};
use entity::async_graphql::{self, InputObject, SimpleObject};
use entity::sea_orm::{ActiveModelTrait, Set};
use entity::sea_orm_active_enums::Genre;
use entity::{authors, books};
use std::str::FromStr;

use crate::database::DB;
use crate::local_storage::LocalStorage;

#[derive(SimpleObject)]
pub struct DeleteResult {
    pub success: bool,
    pub rows_affected: u64,
}

#[derive(Default)]
pub struct CommonMutation;

#[Object]
impl CommonMutation {
    pub async fn upload_book(&self, ctx: &Context<'_>, input: Upload) -> Result<books::Model> {
        let db = ctx.data::<DB>()?;
        let local_storage = ctx.data::<LocalStorage>()?;
        let file = input.value(ctx)?;
        let data = local_storage
            .parse_to_txt(file.content, file.filename)
            .await?;

        let author: Option<authors::Model> = authors::Entity::find_by_name(&data.author)
            .one(db.get_connection())
            .await?;

        let author_id = if let Some(author) = author {
            author.id
        } else {
            let author = authors::ActiveModel {
                name: Set(data.author),
                ..Default::default()
            };
            author.insert(db.get_connection()).await.map_err(|e|anyhow::anyhow!("Error adding author: {e}"))?.id
        };

        let book = books::ActiveModel {
            author_id: Set(author_id),
            title: Set(data.book_name),
            book_path: Set(data.path),
            rating: Set(0),
            short_view: Set(String::from("not filled")),
            genre: Set(Genre::Fiction),
            ..Default::default()
        };

        Ok(book.insert(db.get_connection()).await.map_err(|e|anyhow::anyhow!("Error adding book: {e}"))?)
    }
}
