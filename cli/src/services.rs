use crate::backends::backend::Backend;
use overwatch_rs::services::handle::ServiceStateHandle;
use overwatch_rs::services::relay::NoMessage;
use overwatch_rs::services::state::{NoOperator, NoState};
use overwatch_rs::services::{ServiceCore, ServiceData, ServiceId};
use overwatch_rs::DynError;
use repository::Repository;
use std::fmt::Debug;

#[derive(Clone, Debug)]
pub struct CliSettings<BackendSettings> {
    pub backend: BackendSettings,
}

impl<BackendSettings> CliSettings<BackendSettings> {
    pub fn new(backend: BackendSettings) -> Self {
        Self { backend }
    }
}

impl<CliBackend, RepositoryBackend, BackendEntity> ServiceData
    for Cli<CliBackend, RepositoryBackend, BackendEntity>
where
    BackendEntity: Debug,
    CliBackend: Backend<BackendEntity>,
    CliBackend::Settings: Clone,
    RepositoryBackend: repository::backends::backend::Backend,
{
    const SERVICE_ID: ServiceId = "Cli";
    type Settings = CliSettings<CliBackend::Settings>;
    type State = NoState<Self::Settings>;
    type StateOperator = NoOperator<Self::State>;
    type Message = NoMessage;
}

pub struct Cli<CliBackend, RepositoryBackend, BackendEntity>
where
    BackendEntity: Debug,
    CliBackend: Backend<BackendEntity>,
    CliBackend::Settings: Clone,
    RepositoryBackend: repository::backends::backend::Backend,
{
    service_state: ServiceStateHandle<Self>,
}

#[async_trait::async_trait]
impl<CliBackend, RepositoryBackend, BackendEntity> ServiceCore
    for Cli<CliBackend, RepositoryBackend, BackendEntity>
where
    BackendEntity: Debug,
    CliBackend: Backend<BackendEntity> + Send + Debug + 'static,
    CliBackend::Settings: Send + Sync + Clone,
    RepositoryBackend:
        repository::backends::backend::Backend<Entity = BackendEntity> + Send + Debug + 'static,
    RepositoryBackend::Settings: Send + Sync + Clone,
    RepositoryBackend::Entity: Debug,
{
    fn init(service_state: ServiceStateHandle<Self>) -> Result<Self, DynError> {
        Ok(Self { service_state })
    }

    async fn run(self) -> Result<(), DynError> {
        let Self { service_state } = self;
        let backend_settings = service_state.settings_reader.get_updated_settings().backend;
        let outbound_relay = service_state
            .overwatch_handle
            .relay::<Repository<RepositoryBackend>>()
            .connect()
            .await?;

        let mut backend = CliBackend::new(backend_settings, outbound_relay);
        backend.run().await;

        service_state.overwatch_handle.shutdown().await;
        Ok(())
    }
}
