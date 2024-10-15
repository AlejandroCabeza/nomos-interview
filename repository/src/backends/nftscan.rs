use crate::backends::backend::Backend;
use crate::backends::errors::BackendError;
use crate::backends::nftscan_serializers::{
    NFTScanParameters, NFTScanResponseSerializer, SearchNFTsSerializer, NFT,
};
use crate::backends::nftscan_settings::NFTScanBackendSettings;
use reqwest;
use reqwest::{Client, RequestBuilder, Response};

#[derive(Debug)]
pub struct NFTScanBackend {
    api_key: String,
    collection_address: String,
    nft_cache: Vec<NFT>,
    cursor: Option<String>,
}

impl NFTScanBackend {
    const fn get_auth_header_key() -> &'static str {
        "X-API-KEY"
    }

    const fn build_search_nfts_url() -> &'static str {
        // https://docs.nftscan.com/reference/evm/search-nfts
        "https://restapi.nftscan.com/api/v2/assets/filters"
    }

    async fn serialize_response(
        &self,
        response: Response,
    ) -> Result<NFTScanResponseSerializer<SearchNFTsSerializer>, BackendError> {
        Ok(response.json().await?)
    }

    async fn build_request(&self, with_cursor: bool) -> Result<RequestBuilder, BackendError> {
        let url = Self::build_search_nfts_url();
        let cursor = if with_cursor {
            self.cursor.clone()
        } else {
            None
        };
        let parameters = NFTScanParameters::new(
            vec![self.collection_address.clone()],
            String::from("false"),
            cursor,
        );
        let client = Client::builder()
            .build()
            .map_err(BackendError::RequestError)?;
        Ok(client
            .post(url)
            .json(&parameters)
            .header(Self::get_auth_header_key(), &self.api_key))
    }

    async fn _fill_current_collection(&mut self, response: Response) -> Result<(), BackendError> {
        let response = self.serialize_response(response).await?;
        self.cursor = Some(response.cursor());
        let elements = response.collection();
        self.nft_cache.extend(elements);
        Ok(())
    }

    async fn fill_current_collection(&mut self) -> Result<(), BackendError> {
        match self.build_request(true).await?.send().await {
            Ok(response) => {
                self._fill_current_collection(response).await?;
                Ok(())
            }
            Err(error) => Err(BackendError::RequestError(error)),
        }
    }
}

#[async_trait::async_trait]
impl Backend for NFTScanBackend {
    type Settings = NFTScanBackendSettings;
    type Entity = NFT;

    fn new(settings: Self::Settings) -> Self {
        Self {
            api_key: settings.api_key,
            collection_address: settings.collection_address,
            nft_cache: vec![],
            cursor: None,
        }
    }

    async fn next(&mut self) -> Result<Self::Entity, BackendError> {
        if self.nft_cache.is_empty() {
            self.fill_current_collection().await?;
        }

        match self.nft_cache.pop() {
            Some(item) => Ok(item),
            None => Err(BackendError::NoMoreEntities),
        }
    }
}
