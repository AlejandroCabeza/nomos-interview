use rand::Rng;
use reqwest;
use crate::backends::backend::Backend;
use crate::entities::{Entity, IdentifyImageQuestion};
use crate::backends::errors::BackendError;
use crate::backends::pokemon_serializers::Pokemon;

#[derive(Debug)]
pub struct WhoIsThatPokemonBackend {
    id_min: u16,
    id_max: u16,
}

impl WhoIsThatPokemonBackend {
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

    fn build_question_from_body(&self, body: &str) -> Entity {
        let pokemon = self.parse_body(body);
        Entity::IdentifyImageQuestion(IdentifyImageQuestion::new(String::from("Who's that Pokemon?"), pokemon.image_url(), pokemon.name()))
    }
}

#[async_trait::async_trait]
impl Backend for WhoIsThatPokemonBackend {
    async fn next(&mut self) -> Result<Entity, BackendError> {
        let url = self.get_url_for_random_pokemon();
        let response = reqwest::get(&url).await.map_err(BackendError::RequestError)?;
        let body = response.text().await.map_err(BackendError::RequestError)?;
        Ok(self.build_question_from_body(body.as_str()))
    }
}
