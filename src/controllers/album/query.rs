use crate::entity::{
    album,
    sea_orm_active_enums::{LockingStatus, SafetyRating},
};
use async_graphql::{Context, Object, Result};
use chrono::Utc;
use sea_orm::{DatabaseConnection, EntityTrait};

#[derive(Default)]
pub struct AlbumQuery;

#[Object]
impl AlbumQuery {
    async fn get_sample_album(&self, _ctx: &Context<'_>) -> Result<album::Model> {
        Ok(album::Model {
            id: "879980781240451122".to_string(),
            title: Some("Hello from Album Title!".to_string()),
            description: Some(
                "This is just some general description for your new album :)".to_string(),
            ),
            rating: SafetyRating::Unknown,
            colors: Some("#000000".to_string()),
            author_id: "879980781240451122".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            cover_ext: "png".to_string(),
            lock_status: LockingStatus::None,
            group_id: None,
            user_favourite_ids: None,
        })
    }

    async fn get_album(&self, ctx: &Context<'_>, id: String) -> Result<Option<album::Model>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let album = album::Entity::find_by_id(id).one(db).await?;

        Ok(album)
    }

    async fn get_albums(&self, ctx: &Context<'_>) -> Result<Vec<album::Model>> {
        let db_connection = ctx.data::<DatabaseConnection>()?;

        let albums = album::Entity::find().all(db_connection).await?;

        Ok(albums)
    }
}
