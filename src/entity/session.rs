//! SeaORM Entity. Generated by sea-orm-codegen 0.7.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "Session")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
    pub id: String,
    #[sea_orm(column_name = "createdAt")]
    pub created_at: DateTimeUtc,
    #[sea_orm(column_name = "updatedAt")]
    pub updated_at: DateTimeUtc,
    #[sea_orm(column_name = "expiresAt")]
    pub expires_at: Option<DateTimeUtc>,
    #[sea_orm(column_type = "Text")]
    pub handle: String,
    #[sea_orm(column_name = "hashedSessionToken", column_type = "Text", nullable)]
    pub hashed_session_token: Option<String>,
    #[sea_orm(column_name = "antiCSRFToken", column_type = "Text", nullable)]
    pub anti_csrf_token: Option<String>,
    #[sea_orm(column_name = "publicData", column_type = "Text", nullable)]
    pub public_data: Option<String>,
    #[sea_orm(column_name = "privateData", column_type = "Text", nullable)]
    pub private_data: Option<String>,
    #[sea_orm(column_name = "userId", column_type = "Text", nullable)]
    pub user_id: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id",
        on_update = "Cascade",
        on_delete = "SetNull"
    )]
    User,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
