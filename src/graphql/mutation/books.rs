use async_graphql::{Context, Object, Result};
use entity::async_graphql::{self, InputObject, SimpleObject};
use entity::books;
use entity::sea_orm::{ActiveModelTrait, Set};
use entity::sea_orm_active_enums::Genre;
use std::str::FromStr;

use crate::database::DB;

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

        let author = books::ActiveModel {
            author_id: Set(input.author_id),
            title: Set(input.title),
            book_path: Set(input.book_path),
            rating: Set(input.rating),
            short_view: Set(input.short_view),
            genre: Set(Genre::from_str(&input.genre)?),
            ..Default::default()
        };

        Ok(author.insert(db.get_connection()).await?)
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
        let mut book: books::ActiveModel = book.unwrap().into();
        if let Some(author_id) = input.author_id {
            book.author_id = Set(author_id.to_owned());
        }
        if let Some(title) = input.title {
            book.title = Set(title.to_owned());
        }
        if let Some(book_path) = input.book_path {
            book.book_path = Set(book_path.to_owned());
        }
        if let Some(rating) = input.rating {
            book.rating = Set(rating.to_owned());
        }
        if let Some(short_view) = input.short_view {
            book.short_view = Set(short_view.to_owned());
        }
        if let Some(genre) = input.genre {
            book.genre = Set(Genre::from_str(&genre)?);
        }
        Ok(book.update(db.get_connection()).await?)
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
