use crate::backends::errors::BackendError;
use crate::entities::Entity;

#[async_trait::async_trait]
pub trait Backend {
    async fn next(&self) -> Result<Entity, BackendError>;
}
