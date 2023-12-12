use async_graphql::{Context, Object, Result};
use entity::async_graphql::{self, InputObject, SimpleObject};
use entity::authors;
use entity::sea_orm::{ActiveModelTrait, Set};

use crate::database::DB;

#[derive(InputObject)]
pub struct CreateAuthorInput {
    pub name: String,
    pub surname: String,
    pub bio: String,
}

#[derive(SimpleObject)]
pub struct DeleteResult {
    pub success: bool,
    pub rows_affected: u64,
}

#[derive(Default)]
pub struct AuthorMutation;

#[Object]
impl AuthorMutation {
    pub async fn create_authors(
        &self,
        ctx: &Context<'_>,
        input: CreateAuthorInput,
    ) -> Result<authors::Model> {
        let db = ctx.data::<DB>()?;

        let note = authors::ActiveModel {
            name: Set(input.name),
            surname: Set(input.surname),
            bio: Set(input.bio),
            ..Default::default()
        };

        Ok(note.insert(db.get_connection()).await?)
    }

    pub async fn delete_author(&self, ctx: &Context<'_>, id: i32) -> Result<DeleteResult> {
        let db = ctx.data::<DB>().unwrap();

        let res = authors::Entity::delete_by_id(id)
            .exec(db.get_connection())
            .await?;

        if res.rows_affected <= 1 {
            Ok(DeleteResult {
                success: true,
                rows_affected: res.rows_affected,
            })
        } else {
            unimplemented!()
        }
    }
}
