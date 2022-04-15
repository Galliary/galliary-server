use async_graphql::{Context, Object, Result};
use sea_orm::{DatabaseConnection, Set};
use sea_orm::EntityTrait;
use crate::controllers::album::types::{CreateAlbumInput};
use crate::entity::album;

#[derive(Default)]
pub struct AlbumMutation;

#[Object]
impl AlbumMutation {
    async fn create_album(&self, ctx: &Context<'_>, album_create_request: CreateAlbumInput) -> Result<bool> {
        let db = ctx.data::<DatabaseConnection>()?;

        let album = album::ActiveModel {
            id: Set(album_create_request.id.to_owned()),
            title: Set(album_create_request.title.to_owned()),
            ..Default::default()
        };

        match album::Entity::insert(album).exec(db).await {
            Ok(_) => Ok(true),
            Err(err) => Err(err.into()),
        }
    }
}