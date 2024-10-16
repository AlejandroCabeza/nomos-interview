use crate::errors::PersistenceError;
use async_trait::async_trait;
use overwatch_rs::services::relay::RelayMessage;
use sqlx::{Pool, Sqlite};

#[async_trait]
pub trait HandleableMessage: RelayMessage {
    async fn handle(self, pool: &Pool<Sqlite>) -> Result<(), PersistenceError>;
}
