use crate::controllers::{Mutation, Query};
use async_graphql::{EmptySubscription, Schema};

type AppSchema = Schema<Query, Mutation, EmptySubscription>;

#[derive(Clone)]
pub struct State {
    pub schema: AppSchema,
}

impl State {
    pub fn new(schema: AppSchema) -> Self {
        Self { schema }
    }
}
