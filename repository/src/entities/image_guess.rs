use crate::backends::pokemon_serializers::Pokemon;
use crate::errors::RepositoryError;
use crate::utils::url_to_dynamic_image_url;
use image::DynamicImage;

#[derive(Debug)]
pub struct ImageGuess {
    prompt: String,
    image_url: String,
    answer: String,
}

impl ImageGuess {
    pub fn new(prompt: String, image_url: String, answer: String) -> ImageGuess {
        Self {
            prompt,
            image_url,
            answer,
        }
    }

    pub fn prompt(&self) -> &str {
        &self.prompt
    }

    pub fn image_url(&self) -> &str {
        &self.image_url
    }

    pub async fn image(&self) -> Result<DynamicImage, RepositoryError> {
        url_to_dynamic_image_url(&self.image_url).await
    }

    pub fn answer(&self) -> &str {
        &self.answer
    }
}

impl From<Pokemon> for ImageGuess {
    fn from(pokemon: Pokemon) -> Self {
        ImageGuess::new(
            String::from("Who's that Pokemon?"),
            pokemon.image_url(),
            pokemon.name(),
        )
    }
}
