use image::{DynamicImage};

use crate::backends::nftscan_serializers::NFT;
use crate::errors::RepositoryError;

async fn url_to_dynamic_image_url(url: &String) -> Result<DynamicImage, RepositoryError> {
    let response = reqwest::get(url).await.map_err(RepositoryError::FetchingImage);
    let bytes = response?.bytes().await.map_err(RepositoryError::ParsingImage);
    image::load_from_memory(&bytes?).map_err(RepositoryError::LoadingImage)
}

#[derive(Debug)]
pub enum Entity {
    IdentifyImageQuestion(IdentifyImageQuestion),
    NFTEntity(NFTEntity)
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
        url_to_dynamic_image_url(&self.image_url).await
    }

    pub fn answer(&self) -> &str {
        &self.answer
    }
}

#[derive(Debug)]
pub struct NFTEntity {
    name: String,
    image_uri: String,
    rarity_rank: u16
}

impl NFTEntity {
    pub fn new(name: String, image_uri: String, rarity_rank: u16) -> NFTEntity {
        NFTEntity { name, image_uri, rarity_rank }
    }
    
    pub fn name(&self) -> &str {
        &self.name
    }
    
    pub async fn image(&self) -> Result<DynamicImage, RepositoryError> {
        url_to_dynamic_image_url(&self.image_uri).await
    }
    
    pub fn rarity_rank(&self) -> &u16 {
        &self.rarity_rank
    }
}

impl From<NFT> for NFTEntity {
    fn from(value: NFT) -> Self {
        Self::new(value.name().into(), value.image_uri().into(), value.rarity_rank())
    }
}
