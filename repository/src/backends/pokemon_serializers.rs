use crate::entities::image_guess::ImageGuess;
use async_trait::async_trait;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Pokemon {
    name: String,
    sprites: PokemonSprites,
}

#[derive(Deserialize, Debug)]
struct PokemonSprites {
    front_default: String,
}

#[async_trait]
impl ImageGuess for Pokemon {
    fn prompt(&self) -> &str {
        "Who's that Pokemon?"
    }

    fn image_uri(&self) -> &str {
        self.sprites.front_default.as_str()
    }

    fn answer(&self) -> &str {
        self.name.as_str()
    }
}
