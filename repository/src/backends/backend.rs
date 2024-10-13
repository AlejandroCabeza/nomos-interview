use crate::backends::errors::BackendError;

#[async_trait::async_trait]
pub trait Backend {
    type Settings;
    type Entity;

    fn new(settings: Self::Settings) -> Self;
    async fn next(&mut self) -> Result<Self::Entity, BackendError>;
}
