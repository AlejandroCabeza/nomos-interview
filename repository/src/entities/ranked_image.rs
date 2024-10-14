use crate::backends::nftscan_serializers::NFT;
use crate::errors::RepositoryError;
use crate::utils::url_to_dynamic_image_url;
use image::DynamicImage;

#[derive(Debug)]
pub struct RankedImage {
    name: String,
    image_uri: String,
    rarity_rank: u16,
}

impl RankedImage {
    pub fn new(name: String, image_uri: String, rarity_rank: u16) -> RankedImage {
        RankedImage {
            name,
            image_uri,
            rarity_rank,
        }
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

impl From<NFT> for RankedImage {
    fn from(value: NFT) -> Self {
        Self::new(
            value.name().into(),
            value.image_uri().into(),
            value.rarity_rank(),
        )
    }
}
