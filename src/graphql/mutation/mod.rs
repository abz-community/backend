use entity::async_graphql;

pub mod authors;

pub use authors::AuthorMutation;

// Add your other ones here to create a unified Mutation object
// e.x. Mutation(NoteMutation, OtherMutation, OtherOtherMutation)
#[derive(async_graphql::MergedObject, Default)]
pub struct Mutation(AuthorMutation);
