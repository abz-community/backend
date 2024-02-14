use async_graphql::{Context, Object, Result};
use entity::async_graphql::{self, InputObject, SimpleObject};
use entity::authors;
use entity::sea_orm::{ActiveModelTrait, Set};
use sea_orm::IntoActiveModel;

use crate::database::DB;

use super::to_active;

#[derive(InputObject)]
pub struct CreateAuthorInput {
    pub name: String,
    pub surname: String,
    pub bio: Option<String>,
}

#[derive(InputObject)]
pub struct UpdateAuthorInput {
    pub id: i32,
    pub name: Option<String>,
    pub bio: Option<String>,
}

#[derive(SimpleObject)]
pub struct DeleteAuthorResult {
    pub success: bool,
    pub rows_affected: u64,
}

#[derive(Default)]
pub struct AuthorMutation;

#[Object]
impl AuthorMutation {
    pub async fn create_author(
        &self,
        ctx: &Context<'_>,
        input: CreateAuthorInput,
    ) -> Result<authors::Model> {
        let db = ctx.data::<DB>()?;

        let author = authors::ActiveModel {
            name: Set(input.name),
            bio: Set(input.bio),
            ..Default::default()
        };

        Ok(author.insert(db.get_connection()).await?)
    }

    pub async fn update_author(
        &self,
        ctx: &Context<'_>,
        input: UpdateAuthorInput,
    ) -> Result<authors::Model> {
        let db = ctx.data::<DB>()?;
        let author: Option<authors::Model> = authors::Entity::find_by_id(input.id)
            .one(db.get_connection())
            .await?;
        if let Some(author) = author {
            let mut author = author.into_active_model();
            author.name = to_active(input.name.to_owned());
            author.bio = to_active(Some(input.bio.to_owned()));
            Ok(author.update(db.get_connection()).await?)
        } else {
            Err(async_graphql::Error::new(
                "Cannot update non existing instance",
            ))
        }
    }

    pub async fn delete_author(&self, ctx: &Context<'_>, id: i32) -> Result<DeleteAuthorResult> {
        let db = ctx.data::<DB>().unwrap();

        let res = authors::Entity::delete_by_id(id)
            .exec(db.get_connection())
            .await?;

        if res.rows_affected <= 1 {
            Ok(DeleteAuthorResult {
                success: true,
                rows_affected: res.rows_affected,
            })
        } else {
            unimplemented!()
        }
    }
}
