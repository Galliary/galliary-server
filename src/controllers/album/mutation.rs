use crate::controllers::album::types::CreateAlbumInput;
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
        album_create_request: CreateAlbumInput,
    ) -> Result<bool> {
        let db = ctx.data::<DatabaseConnection>()?;

        let album = album::ActiveModel {
            id: Set(album_create_request.id.to_owned()),
            title: Set(album_create_request.title.to_owned()),
            ..Default::default()
        };

        Ok(album::Entity::insert(album).exec(db).await.is_ok())
    }
}
