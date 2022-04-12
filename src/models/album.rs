use diesel::dsl::Nullable;
use diesel::sql_types::Array;
use crate::structs::enums::{LockingStatus, SafetyRating};

#[derive(GraphQLObject)]
#[graphql(description="An album lol")]
#[derive(Queryable)]
pub struct Album {
    pub id: String,
    pub createdAt: NaiveDateTime,
    pub updatedAt: NaiveDateTime,
    pub sourceId: String,
    pub colors: Option<Array<String>>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub authorId: String,
    pub groupId: Option<String>,
    pub lockStatus: LockingStatus,
    pub rating: SafetyRating,
    pub userFavouriteIds: Option<Array<String>>,
}