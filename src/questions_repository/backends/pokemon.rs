use rand::{Rng};
use crate::questions_repository::backends::backend::QuestionsRepositoryBackend;
use crate::questions_repository::question::{Question, IdentifyImage};
use reqwest;
use serde::Deserialize;
use crate::questions_repository::backends::errors::BackendError;

#[derive(Deserialize, Debug)]
struct Pokemon {
    id: u16,
    name: String,
    sprites: PokemonSprites
}

#[derive(Deserialize, Debug)]
struct PokemonSprites {
    front_default: String,
}

#[derive(Debug)]
pub struct PokemonBackend {
    id_min: u16,
    id_max: u16,
}

impl PokemonBackend {
    pub fn new(id_min: Option<u16>, id_max: Option<u16>) -> Self {
        Self {
            id_min: id_min.unwrap_or(0),
            id_max: id_max.unwrap_or(151)
        }
    }

    fn get_random_pokemon_id(&self) -> u16 {
        let mut thread_rng = rand::thread_rng();
        thread_rng.gen_range(self.id_min..self.id_max)
    }

    fn get_url_for_pokemon_with_id(&self, id: u16) -> String {
        format!("https://pokeapi.co/api/v2/pokemon/{}", id)
    }

    fn get_url_for_random_pokemon(&self) -> String {
        let random_id = self.get_random_pokemon_id();
        self.get_url_for_pokemon_with_id(random_id)
    }

    fn parse_body(&self, body: &str) -> Pokemon {
        serde_json::from_str(body).unwrap()
    }

    fn build_question_from_body(&self, body: &str) -> Question {
        let pokemon = self.parse_body(body);
        Question::IdentifyImage(IdentifyImage::new(String::from("Who's that Pokemon?"), pokemon.sprites.front_default, pokemon.name))
    }
}

#[async_trait::async_trait]
impl QuestionsRepositoryBackend for PokemonBackend {
    async fn next(&self) -> Result<Question, BackendError> {
        let url = self.get_url_for_random_pokemon();
        let response = reqwest::get(&url).await.map_err(BackendError::RequestError)?;
        let body = response.text().await.map_err(BackendError::RequestError)?;
        Ok(self.build_question_from_body(body.as_str()))
    }
}
