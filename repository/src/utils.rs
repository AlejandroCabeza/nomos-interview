use crate::errors::RepositoryError;
use image::DynamicImage;

pub async fn url_to_dynamic_image_url(url: &str) -> Result<DynamicImage, RepositoryError> {
    let response = reqwest::get(url)
        .await
        .map_err(RepositoryError::FetchingImage);
    let bytes = response?
        .bytes()
        .await
        .map_err(RepositoryError::ParsingImage);
    image::load_from_memory(&bytes?).map_err(RepositoryError::LoadingImage)
}
