use crate::entity::image;
use async_graphql::{Context, Object, Result};

#[derive(Default)]
pub struct ImageQuery;

#[Object]
impl ImageQuery {
    async fn get_sample_image(&self, _ctx: &Context<'_>) -> Result<image::Model> {
        todo!()
    }
    async fn get_image(&self, ctx: &Context<'_>, id: String) -> Result<Option<image::Model>> {
        todo!()
    }
}
