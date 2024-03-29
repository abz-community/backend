//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.4

use async_graphql::*;
use sea_orm::entity::prelude::*;
use sea_orm::DeleteMany;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, SimpleObject)]
#[sea_orm(table_name = "payments")]
#[graphql(concrete(name = "Payments", params()))]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i32,
    pub created_at: Option<DateTime>,
    pub success: Option<bool>,
    pub user_id: Option<i32>,
    pub amount: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Entity {
    pub fn find_by_id(id: i32) -> Select<Entity> {
        Self::find().filter(Column::Id.eq(id))
    }

    pub fn find_by_user(id: i32) -> Select<Entity> {
        Self::find().filter(Column::UserId.eq(id))
    }

    pub fn delete_by_id(id: i32) -> DeleteMany<Entity> {
        Self::delete_many().filter(Column::Id.eq(id))
    }
}
