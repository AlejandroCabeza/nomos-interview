use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Pokemon {
    name: String,
    sprites: PokemonSprites
}

impl Pokemon {
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn image_url(&self) -> String {
        self.sprites.front_default()
    }
}

#[derive(Deserialize, Debug)]
struct PokemonSprites {
    front_default: String,
}

impl PokemonSprites {
    fn front_default(&self) -> String {
        self.front_default.clone()
    }
}
