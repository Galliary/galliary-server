use super::types::CreateAlbum;
use crate::entity::album;
use async_graphql::{Context, Object, Result};
use sea_orm::{DatabaseConnection, EntityTrait, Set};

#[derive(Default)]
pub struct AlbumMutation;

#[Object]
impl AlbumMutation {
    async fn create_album(
        &self,
        ctx: &Context<'_>,
        album_create_request: CreateAlbum,
    ) -> Result<bool> {
        let db = ctx.data::<DatabaseConnection>()?;

        let album = album::ActiveModel {
            id: Set(album_create_request.id),
            title: Set(album_create_request.title),
            ..Default::default()
        };

        Ok(album::Entity::insert(album).exec(db).await.is_ok())
    }
}
