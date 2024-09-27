use image::{DynamicImage};

#[derive(Debug)]
pub enum Question {
    IdentifyImage(IdentifyImage),
}

#[derive(Debug)]
pub struct IdentifyImage {
    prompt: String,
    image_url: String,
    answer: String,
}

impl IdentifyImage {
    pub fn new(prompt: String, image_url: String, answer: String) -> IdentifyImage {
        Self { prompt, image_url, answer }
    }

    pub fn prompt(&self) -> &str {
        &self.prompt
    }

    pub fn image_url(&self) -> &str {
        &self.image_url
    }

    pub async fn image(&self) -> Result<DynamicImage, String> {
        let response = reqwest::get(&self.image_url).await.map_err(|error| format!("Failed fetching image: {}", error));
        let bytes = response?.bytes().await.map_err(|error| format!("Failed reading image: {}", error));
        image::load_from_memory(&bytes?).map_err(|error| format!("Failed reading image: {}", error))
    }

    pub fn answer(&self) -> &str {
        &self.answer
    }
}
