#[derive(Clone, Debug)]
pub struct PersistenceSettings {
    database_url: String,
}

impl PersistenceSettings {
    pub fn new(database_url: String) -> Self {
        Self { database_url }
    }

    pub fn database_url(&self) -> &str {
        &self.database_url
    }
}
