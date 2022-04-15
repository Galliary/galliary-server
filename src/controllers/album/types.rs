use async_graphql::*;
use crate::controllers::album::validators::{
    AlbumTitleValidator,
    AlbumDescriptionValidator
};
use crate::entity::sea_orm_active_enums::{LockingStatus, SafetyRating};

#[derive(InputObject)]
pub struct CreateAlbumInput {
    pub id: String,
    #[graphql(validator(custom = "AlbumTitleValidator{}"))]
    pub title: Option<String>,
    #[graphql(validator(custom = "AlbumDescriptionValidator{}"))]
    pub description: Option<String>,

    pub cover_ext: String,
    pub colors: Option<String>,
    pub author_id: String,
    pub group_id: Option<String>,
    pub lock_status: LockingStatus,
    pub rating: SafetyRating,
}
