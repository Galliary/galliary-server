use sea_orm::DatabaseConnection;

use crate::AppSchema;

#[derive(Clone)]
pub struct State {
    pub schema: AppSchema,
}

impl State {
    pub fn new(schema: AppSchema) -> Self {
        Self { schema }
    }
}
