use crate::controllers::album::mutation::AlbumMutation;
use album::query::AlbumQuery;

// Controllers
pub mod album;

// Query
#[derive(async_graphql::MergedObject, Default)]
pub struct Query(AlbumQuery);

// Mutation
#[derive(async_graphql::MergedObject, Default)]
pub struct Mutation(AlbumMutation);
