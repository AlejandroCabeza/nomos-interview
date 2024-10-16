use crate::backends::backend::Backend;
use crate::backends::errors::BackendError;
use crate::backends::gallery_settings::GalleryBackendSettings;
use crate::backends::utils::request_entity_to_repository;
use crate::utils::terminal_utils::clear_screen;
use crate::utils::viuer_utils::debug_image_config;
use async_trait::async_trait;
use overwatch_rs::overwatch::handle::OverwatchHandle;
use overwatch_rs::services::relay::OutboundRelay;
use repository::entities::ranked_image::RankedImage;
use repository::RepositoryMessage;
use std::fmt::{Debug, Formatter};
use viuer::{print as print_image_in_terminal, Config};

pub struct GalleryBackend<Entity: Debug> {
    overwatch_handle: OverwatchHandle,
    repository_relay: OutboundRelay<RepositoryMessage<Entity>>,
    image_config: Config,
}

impl<Entity: Debug> Debug for GalleryBackend<Entity> {
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
impl<Entity> Backend<Entity> for GalleryBackend<Entity>
where
    Entity: RankedImage + Send + Sync + Debug,
{
    type Settings = GalleryBackendSettings;
    type PersistenceMessage = ();

    fn new(
        settings: Self::Settings,
        overwatch_handle: OverwatchHandle,
        repository_relay: OutboundRelay<RepositoryMessage<Entity>>,
    ) -> Self {
        Self {
            overwatch_handle,
            repository_relay,
            image_config: settings.image_config,
        }
    }

    async fn _init(&mut self) {
        println!("> Welcome to CliGallery!")
    }

    async fn request_entity(&self) -> Result<Entity, BackendError> {
        request_entity_to_repository(&self.repository_relay).await
    }

    async fn handle_entity(&mut self, entity: Entity) -> Result<(), BackendError> {
        clear_screen();
        let image = entity.image().await.map_err(BackendError::Repository)?;
        print_image_in_terminal(&image, &self.image_config).map_err(BackendError::ImageRender)?;
        println!("> On Display: {}", entity.description());
        Ok(())
    }
}
