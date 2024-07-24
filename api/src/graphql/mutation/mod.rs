pub mod tcc_mutation;

use entity::async_graphql;
pub use tcc_mutation::RecipeMutation;

#[derive(async_graphql::MergedObject, Default)]
pub struct Mutation(RecipeMutation);
