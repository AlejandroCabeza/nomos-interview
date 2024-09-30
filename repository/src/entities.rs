use image::{DynamicImage};

use crate::errors::RepositoryError;

#[derive(Debug)]
pub enum Entity {
    IdentifyImageQuestion(IdentifyImageQuestion),
}

#[derive(Debug)]
pub struct IdentifyImageQuestion {
    prompt: String,
    image_url: String,
    answer: String,
}

impl IdentifyImageQuestion {
    pub fn new(prompt: String, image_url: String, answer: String) -> IdentifyImageQuestion {
        Self { prompt, image_url, answer }
    }

    pub fn prompt(&self) -> &str {
        &self.prompt
    }

    pub fn image_url(&self) -> &str {
        &self.image_url
    }

    pub async fn image(&self) -> Result<DynamicImage, RepositoryError> {
        let response = reqwest::get(&self.image_url).await.map_err(RepositoryError::FetchingImage);
        let bytes = response?.bytes().await.map_err(RepositoryError::ParsingImage);
        image::load_from_memory(&bytes?).map_err(RepositoryError::LoadingImage)
    }

    pub fn answer(&self) -> &str {
        &self.answer
    }
}
