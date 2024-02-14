use async_graphql::{Context, Object, Result};
use entity::async_graphql::{self, InputObject, SimpleObject};
use entity::books;
use entity::sea_orm::{ActiveModelTrait, Set};
use entity::sea_orm_active_enums::Genre;
use sea_orm::IntoActiveModel;
use std::str::FromStr;

use crate::database::DB;

use super::to_active;

#[derive(InputObject)]
pub struct CreateBookInput {
    pub author_id: i32,
    pub title: String,
    pub book_path: String,
    pub rating: i32,
    pub short_view: String,
    pub genre: String,
}

#[derive(InputObject)]
pub struct UpdateBookInput {
    pub id: i32,
    pub author_id: Option<i32>,
    pub title: Option<String>,
    pub book_path: Option<String>,
    pub rating: Option<i32>,
    pub short_view: Option<String>,
    pub genre: Option<String>,
}

#[derive(SimpleObject)]
pub struct DeleteBookResult {
    pub success: bool,
    pub rows_affected: u64,
}

#[derive(Default)]
pub struct BookMutation;

#[Object]
impl BookMutation {
    pub async fn create_book(
        &self,
        ctx: &Context<'_>,
        input: CreateBookInput,
    ) -> Result<books::Model> {
        let db = ctx.data::<DB>()?;

        let book = books::ActiveModel {
            author_id: Set(input.author_id),
            title: Set(input.title),
            book_path: Set(input.book_path),
            rating: Set(input.rating),
            short_view: Set(input.short_view),
            genre: Set(Genre::from_str(&input.genre)?),
            ..Default::default()
        };

        Ok(book.insert(db.get_connection()).await?)
    }

    pub async fn update_book(
        &self,
        ctx: &Context<'_>,
        input: UpdateBookInput,
    ) -> Result<books::Model> {
        let db = ctx.data::<DB>()?;
        let book: Option<books::Model> = books::Entity::find_by_id(input.id)
            .one(db.get_connection())
            .await?;
        if let Some(book) = book {
            let genre = input.genre.map(|v| Genre::from_str(&v).ok()).flatten();
            let mut book = book.into_active_model();
            book.author_id = to_active(input.author_id);
            book.title = to_active(input.title);
            book.book_path = to_active(input.book_path);
            book.rating = to_active(input.rating);
            book.short_view = to_active(input.short_view);
            book.genre = to_active(genre);
            Ok(book.update(db.get_connection()).await?)
        } else {
            Err(async_graphql::Error::new(
                "Cannot update non existing instance",
            ))
        }
    }

    pub async fn delete_book(&self, ctx: &Context<'_>, id: i32) -> Result<DeleteBookResult> {
        let db = ctx.data::<DB>().unwrap();

        let res = books::Entity::delete_by_id(id)
            .exec(db.get_connection())
            .await?;

        if res.rows_affected <= 1 {
            Ok(DeleteBookResult {
                success: true,
                rows_affected: res.rows_affected,
            })
        } else {
            unimplemented!()
        }
    }
}
