use crate::backends::errors::BackendError;
use crate::utils::terminal_utils::{continue_or_exit, ContinueCommand};
use async_trait::async_trait;
use overwatch_rs::services::relay::OutboundRelay;
use repository::RepositoryMessage;
use std::fmt::Debug;
use tracing::error;

#[async_trait]
pub trait Backend<Entity: Debug> {
    type Settings;

    fn new(
        settings: Self::Settings,
        outbound_relay: OutboundRelay<RepositoryMessage<Entity>>,
    ) -> Self;

    async fn _init(&self);

    async fn _main_loop(&mut self) {
        loop {
            let entity = match self.request_entity().await {
                Ok(receiver) => receiver,
                Err(error) => {
                    error!("Could not fetch entity: {:?}", error);
                    continue;
                }
            };

            if let Err(error) = self.handle_entity(entity).await {
                println!("> An error happened. Let's try again!");
                error!("Could not handle entity: {}", error);
                continue;
            }

            match continue_or_exit() {
                Ok(ContinueCommand::Continue) => continue,
                Ok(ContinueCommand::Exit) => break,
                Err(error) => {
                    error!("Found an error when parsing input: {}", error);
                    break;
                }
            };
        }
    }

    async fn request_entity(&self) -> Result<Entity, BackendError>;
    async fn handle_entity(&mut self, entity: Entity) -> Result<(), BackendError>;
    async fn _finalize(&mut self) {}

    async fn run(&mut self) {
        self._init().await;
        self._main_loop().await;
        self._finalize().await;
    }
}
