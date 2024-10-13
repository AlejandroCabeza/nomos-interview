#[derive(Debug, Clone)]
pub struct NFTScanBackendSettings {
    pub(crate) api_key: String,
    pub(crate) collection_address: String,
}

impl NFTScanBackendSettings {
    pub fn new(api_key: String, collection_address: String) -> NFTScanBackendSettings {
        NFTScanBackendSettings {
            api_key,
            collection_address,
        }
    }
}
