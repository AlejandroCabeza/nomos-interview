use crate::errors::RepositoryError;
use crate::utils::url_to_dynamic_image_url;
use async_trait::async_trait;
use image::DynamicImage;

#[async_trait]
pub trait RankedImage {
    fn description(&self) -> &str;
    fn image_uri(&self) -> &str;
    fn rarity_rank(&self) -> &u16;

    async fn image(&self) -> Result<DynamicImage, RepositoryError> {
        url_to_dynamic_image_url(self.image_uri()).await
    }
}
