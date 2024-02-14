use crate::graphql::mutation::to_active;
use async_graphql::{Context, Object, Result, Upload};
use chrono::NaiveTime;
use entity::async_graphql::{self, InputObject, SimpleObject};
use entity::sea_orm::{ActiveModelTrait, Set};
use entity::sea_orm_active_enums::Genre;
use entity::{authors, books, reads};
use sea_orm::{IntoActiveModel, ModelTrait};

use crate::database::DB;
use crate::local_storage::LocalStorage;

#[derive(SimpleObject)]
pub struct DeleteResult {
    pub success: bool,
    pub rows_affected: u64,
}

#[derive(InputObject)]
pub struct ReadOpts {
    pub id: Option<i32>,
    pub user_id: i32,
    pub book_id: i32,
    pub page: Option<i32>,
    pub symbol: Option<i32>,
    pub amount_to_send: Option<i32>,
    pub time_to_send: Option<String>,
}

#[derive(InputObject)]
pub struct NextReadingQuery {
    pub user_id: i32,
    pub book_id: i32,
}

#[derive(SimpleObject)]
pub struct NextReadingResult {
    pub text: String,
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
            author
                .insert(db.get_connection())
                .await
                .map_err(|e| anyhow::anyhow!("Error adding author: {e}"))?
                .id
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

        book.insert(db.get_connection())
            .await
            .map_err(|e| async_graphql::Error::new(format!("Error adding book: {e}")))
    }

    pub async fn update_reading(&self, ctx: &Context<'_>, opts: ReadOpts) -> Result<reads::Model> {
        let db = ctx.data::<DB>()?;
        // not a good variant to flatten parsing error
        let time = opts
            .time_to_send
            .map(|v| NaiveTime::parse_from_str(&v, "%H:%M:%S").ok())
            .flatten();
        if let Some(id) = opts.id {
            let read: Option<reads::Model> = reads::Entity::find_by_id(id)
                .one(db.get_connection())
                .await?;
            if let Some(read) = read {
                let mut read = read.into_active_model();
                read.page = to_active(opts.page);
                read.symbol = to_active(opts.symbol);
                read.time_to_send = to_active(time);
                read.amount_to_send = to_active(opts.amount_to_send);
                return Ok(read.update(db.get_connection()).await?);
            }
            Err(async_graphql::Error::new("Provided id doesn't exist"))
        } else {
            let read = reads::ActiveModel {
                book_id: Set(opts.book_id),
                user_id: Set(opts.user_id),
                page: to_active(opts.page),
                symbol: to_active(opts.symbol),
                amount_to_send: to_active(opts.amount_to_send),
                time_to_send: to_active(time),
                ..Default::default()
            };
            read.insert(db.get_connection())
                .await
                .map_err(|e| async_graphql::Error::new(format!("Error adding book: {e}")))
        }
    }

    pub async fn get_next_reading(
        &self,
        ctx: &Context<'_>,
        opts: NextReadingQuery,
    ) -> Result<NextReadingResult> {
        let db = ctx.data::<DB>()?;
        let local_storage = ctx.data::<LocalStorage>()?;
        let read = reads::Entity::find_by_user_and_book(opts.user_id, opts.book_id)
            .one(db.get_connection())
            .await?;
        if let Some(read) = read {
            let book = read
                .find_related(books::Entity)
                .one(db.get_connection())
                .await?;
            let book = book.ok_or(async_graphql::Error::new(
                "Book for current read doesn't exist",
            ))?;
            let text = local_storage.read_from(
                book.book_path,
                read.page,
                read.symbol,
                read.amount_to_send as usize,
            ).await?;
            Ok(NextReadingResult { text })
        } else {
            Err(async_graphql::Error::new(
                "Cannot find necessary params for current book and user",
            ))
        }
    }
}
