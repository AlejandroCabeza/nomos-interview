mod nft;
mod pokemon;

use crate::nft::NFTViewer;
use crate::pokemon::WhoIsThatPokemonGame;
use std::env::var;
use std::io;

fn main() {
    println!("List of Programs:");
    println!("1. Who is that Pokemon?");
    println!("2. Deathbat NFT Viewer");
    println!("Type the index to run the program.");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    match choice.trim() {
        "1" => WhoIsThatPokemonGame::run(),
        "2" => {
            let api_key = var("API_KEY__NFT_SCAN").unwrap();
            let collection_address = var("ADDRESS__DEATHBAT").unwrap();
            NFTViewer::run(api_key, collection_address)
        }
        _ => {
            println!("Invalid choice. Exiting...");
        }
    };
}
