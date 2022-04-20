use async_graphql::{Context, InputObject, Object, Result};

use crate::entity::image;

#[derive(InputObject)]
struct CreateImage {
    id: String,
}

#[derive(Default)]
pub struct ImageMutation;

#[Object]
impl ImageMutation {
    async fn create_image(
        &self,
        _ctx: &Context<'_>,
        image_create_request: CreateImage,
    ) -> Result<image::Model> {
        todo!()
    }
}
