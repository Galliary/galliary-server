use album::mutation::AlbumMutation;
use album::query::AlbumQuery;

use image::mutation::ImageMutation;
use image::query::ImageQuery;

// Controllers
pub mod album;
pub mod image;

// Query
#[derive(async_graphql::MergedObject, Default)]
pub struct Query(AlbumQuery, ImageQuery);

// Mutation
#[derive(async_graphql::MergedObject, Default)]
pub struct Mutation(AlbumMutation, ImageMutation);
