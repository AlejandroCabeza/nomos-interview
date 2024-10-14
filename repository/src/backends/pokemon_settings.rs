#[derive(Debug, Clone)]
pub struct WhoIsThatPokemonBackendSettings {
    pub(crate) id_min: u16,
    pub(crate) id_max: u16,
}

impl WhoIsThatPokemonBackendSettings {
    pub fn new(id_min: u16, id_max: u16) -> Self {
        Self { id_min, id_max }
    }
}
