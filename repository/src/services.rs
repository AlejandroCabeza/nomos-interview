use crate::backends::backend::Backend;
use crate::messages::RepositoryMessage;
use async_trait::async_trait;
use overwatch_rs::services::handle::ServiceStateHandle;
use overwatch_rs::services::relay::InboundRelay;
use overwatch_rs::services::state::{NoOperator, NoState};
use overwatch_rs::services::{ServiceCore, ServiceData, ServiceId};
use std::fmt::Debug;
use tokio::sync::oneshot::Sender;
use tracing::error;

#[derive(Clone, Debug)]
pub struct RepositorySettings<BackendSettings> {
    backend: BackendSettings,
}

impl<BackendSettings> RepositorySettings<BackendSettings> {
    pub fn new(backend: BackendSettings) -> Self {
        Self { backend }
    }
}

pub struct Repository<RepositoryBackend>
where
    RepositoryBackend: Backend + Debug + Send + 'static,
    RepositoryBackend::Entity: Debug,
    RepositoryBackend::Settings: Clone,
{
    service_state: ServiceStateHandle<Self>,
    backend: RepositoryBackend,
}

impl<RepositoryBackend> ServiceData for Repository<RepositoryBackend>
where
    RepositoryBackend: Backend + Debug + Send + 'static,
    RepositoryBackend::Entity: Debug,
    RepositoryBackend::Settings: Clone,
{
    const SERVICE_ID: ServiceId = "Repository";
    type Settings = RepositorySettings<RepositoryBackend::Settings>;
    type State = NoState<Self::Settings>;
    type StateOperator = NoOperator<Self::State>;
    type Message = RepositoryMessage<RepositoryBackend::Entity>;
}

#[async_trait]
impl<RepositoryBackend> ServiceCore for Repository<RepositoryBackend>
where
    RepositoryBackend: Backend + Debug + Send + 'static,
    RepositoryBackend::Entity: Send + Debug,
    RepositoryBackend::Settings: Send + Sync + Clone,
{
    fn init(service_state: ServiceStateHandle<Self>) -> Result<Self, overwatch_rs::DynError> {
        let backend_settings = service_state.settings_reader.get_updated_settings().backend;
        let backend = <RepositoryBackend as Backend>::new(backend_settings);
        Ok(Self {
            service_state,
            backend,
        })
    }

    async fn run(self) -> Result<(), overwatch_rs::DynError> {
        let Self {
            service_state,
            mut backend,
        } = self;

        let mut inbound_relay = service_state.inbound_relay;
        Self::get_repository_loop(&mut backend, &mut inbound_relay).await;

        Ok(())
    }
}

impl<RepositoryBackend> Repository<RepositoryBackend>
where
    RepositoryBackend: Backend + Debug + Send + 'static,
    RepositoryBackend::Entity: Debug,
    RepositoryBackend::Settings: Clone,
{
    async fn get_repository_loop(
        backend: &mut RepositoryBackend,
        inbound_relay: &mut InboundRelay<RepositoryMessage<RepositoryBackend::Entity>>,
    ) {
        while let Some(message) = inbound_relay.recv().await {
            match message {
                RepositoryMessage::RequestEntity(sender) => {
                    Self::request_entity(backend, sender).await;
                }
            }
        }
    }

    async fn request_entity(
        backend: &mut RepositoryBackend,
        sender: Sender<RepositoryBackend::Entity>,
    ) {
        loop {
            match backend.next().await {
                Ok(entity) => {
                    Self::send_entity(sender, entity);
                    break;
                }
                Err(error) => {
                    error!("Could not get the next entity: {:?} ", error);
                }
            }
        }
    }

    fn send_entity(sender: Sender<RepositoryBackend::Entity>, entity: RepositoryBackend::Entity) {
        if let Err(error) = sender.send(entity) {
            error!("Could not send message to Cli network relay: {:?}", error)
        }
    }
}
