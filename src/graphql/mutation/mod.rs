use entity::async_graphql;

pub mod authors;
pub mod books;
pub mod common;

pub use authors::AuthorMutation;

use self::{books::BookMutation, common::CommonMutation};

// Add your other ones here to create a unified Mutation object
// e.x. Mutation(NoteMutation, OtherMutation, OtherOtherMutation)
#[derive(async_graphql::MergedObject, Default)]
pub struct Mutation(AuthorMutation, BookMutation, CommonMutation);
