use crate::errors::PersistenceError;
use crate::messages::handleable_message::HandleableMessage;
use crate::third_party::database::operations::set_score;
use async_trait::async_trait;
use overwatch_rs::services::relay::RelayMessage;
use sqlx::{Pool, Sqlite};
use tracing::info;

#[derive(Debug)]
pub enum PersistenceQuestionsMessage {
    SaveScore(u16)
}

impl RelayMessage for PersistenceQuestionsMessage {}

#[async_trait]
impl HandleableMessage for PersistenceQuestionsMessage
{
    async fn handle(self, pool: &Pool<Sqlite>) -> Result<(), PersistenceError> {
        match self {
            PersistenceQuestionsMessage::SaveScore(score) => {
                set_score(score, pool).await?;
                info!("Score saved!");
                Ok(())
            }
        }
    }
}
