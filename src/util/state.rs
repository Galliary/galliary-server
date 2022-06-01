use async_graphql::{EmptyMutation, EmptySubscription, Schema};

// type AppSchema = Schema<Default::default(), EmptyMutation, EmptySubscription>;

pub struct State {
    // pub schema: AppSchema,
}

impl State {
    pub fn new() -> Self {
        Self {}
        // Self { schema }
    }
}
