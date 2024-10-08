use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct NFTScanParameters {
    contract_address_list: Vec<String>,
    show_attribute: String,
    cursor: Option<String>,
    limit: u8
}

impl NFTScanParameters {
    pub fn new(contract_address_list: Vec<String>, show_attribute: String, cursor: Option<String>) -> NFTScanParameters {
        Self { contract_address_list, show_attribute, cursor, limit: 3 }
    }
}

#[derive(Deserialize, Debug)]
pub struct NFTScanResponseSerializer<T> {
    code: u16,
    msg: Option<String>,
    data: T
}

impl NFTScanResponseSerializer<SearchNFTsSerializer> {
    pub fn collection(self) -> Vec<NFT> {
        self.data.content
    }
    
    pub fn cursor(&self) -> String {
        self.data.next.clone()
    }
}

#[derive(Deserialize, Debug)]
pub struct SearchNFTsSerializer {
    total: u16,
    next: String,
    content: Vec<NFT>
}

#[derive(Deserialize, Debug)]
pub struct NFT {
    name: String,
    image_uri: String,
    rarity_rank: u16
}

impl NFT {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn image_uri(&self) -> &str {
        &self.image_uri
    }

    pub fn rarity_rank(&self) -> u16 {
        self.rarity_rank
    }
}