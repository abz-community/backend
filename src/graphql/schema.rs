use async_graphql::{EmptySubscription, Schema};
use entity::async_graphql;

use crate::{
    database::DB,
    graphql::{mutation::Mutation, query::Query},
    local_storage::LocalStorage,
};

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

/// Builds the GraphQL Schema, attaching the Database to the context
pub async fn build_schema() -> AppSchema {
    let db = DB::init().await;
    println!("{:?}", db);
    let db = db.unwrap();
    let local_storage = LocalStorage::new();

    Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(db)
        .data(local_storage)
        .finish()
}
