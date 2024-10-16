use crate::backends::errors::BackendError;
use overwatch_rs::services::relay::OutboundRelay;
use repository::RepositoryMessage;
use std::fmt::Debug;
use tokio::sync::oneshot;
use tracing::error;

pub async fn request_entity_to_repository<Entity>(
    repository_relay: &OutboundRelay<RepositoryMessage<Entity>>
) -> Result<Entity, BackendError>
where
    Entity: Debug,
{
    let (sender, receiver) = oneshot::channel();
    match repository_relay
        .send(RepositoryMessage::RequestEntity(sender))
        .await
    {
        Ok(_) => Ok(receiver.await.map_err(BackendError::OneshotReceive)?),
        Err((relay_error, repository_message)) => {
            error!(
                "Could not send {:?} Request to QuestionsRepository: {:?}",
                repository_message, relay_error
            );
            Err(BackendError::Relay(relay_error))
        }
    }
}