use crate::backends::backend::Backend;
use crate::backends::errors::BackendError;
use crate::backends::gallery_settings::GallerySettings;
use crate::utils::terminal_utils::clear_screen;
use crate::utils::viuer_utils::debug_image_config;
use async_trait::async_trait;
use overwatch_rs::services::relay::OutboundRelay;
use repository::entities::ranked_image::RankedImage;
use repository::RepositoryMessage;
use std::fmt::{Debug, Formatter};
use tokio::sync::oneshot;
use tracing::error;
use viuer::{print as print_image_in_terminal, Config};

pub struct Gallery {
    repository_network_relay: OutboundRelay<RepositoryMessage<RankedImage>>,
    image_config: Config,
}

impl Debug for Gallery {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let image_config_debug = debug_image_config(&self.image_config);
        f.debug_struct("ImageVisualizerBackend")
            .field(
                "repository_network_relay",
                &"OutboundRelay<RepositoryMessage<NFTImage>>",
            )
            .field("image_config", &image_config_debug)
            .finish()
    }
}

#[async_trait]
impl Backend for Gallery {
    type Settings = GallerySettings;
    type Entity = RankedImage;

    fn new(
        settings: Self::Settings,
        outbound_relay: OutboundRelay<RepositoryMessage<Self::Entity>>,
    ) -> Self
    where
        Self::Entity: Debug,
    {
        Self {
            repository_network_relay: outbound_relay,
            image_config: settings.image_config,
        }
    }

    async fn _init(&self) {
        println!("> Welcome to CliGallery!")
    }

    async fn request_entity(&self) -> Result<Self::Entity, BackendError> {
        let (sender, receiver) = oneshot::channel();
        match self
            .repository_network_relay
            .send(RepositoryMessage::RequestEntity(sender))
            .await
        {
            Ok(_) => Ok(receiver.await.map_err(BackendError::OneshotReceive)?),
            Err((relay_error, repository_message)) => {
                // TODO: if more than 2 failures then exit? or maybe just warning.
                // TODO: Don't log and just ?.map_err, let upper level log
                error!(
                    "Could not send {:?} Request to QuestionsRepository: {:?}",
                    repository_message, relay_error
                );
                Err(BackendError::Relay(relay_error))
            }
        }
    }

    async fn handle_entity(&mut self, entity: Self::Entity) -> Result<(), BackendError> {
        clear_screen();
        let image = entity.image().await.map_err(BackendError::Repository)?;
        print_image_in_terminal(&image, &self.image_config).map_err(BackendError::ImageRender)?;
        println!("> On Display: {}", entity.name());
        Ok(())
    }
}
