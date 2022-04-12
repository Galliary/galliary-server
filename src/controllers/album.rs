use diesel::expression::nullable::Nullable;
use diesel::{PgConnection, QueryResult};
use diesel::sql_types::Array;
use juniper::{FieldResult, graphql_object};
use crate::models::album::{Album};
use crate::structs::enums::{LockingStatus, SafetyRating};

pub fn get_album() -> QueryResult<i8> {
    Ok(0)
}

pub fn get_albums() -> QueryResult<Vec<i8>> {
    Ok(vec![0, 1, 2])
}