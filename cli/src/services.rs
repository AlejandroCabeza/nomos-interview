use crate::backends::backend::Backend;
use crate::settings::CliSettings;
use async_trait::async_trait;
use overwatch_rs::services::handle::ServiceStateHandle;
use overwatch_rs::services::relay::NoMessage;
use overwatch_rs::services::state::{NoOperator, NoState};
use overwatch_rs::services::{ServiceCore, ServiceData, ServiceId};
use overwatch_rs::DynError;
use repository::Repository;
use std::fmt::Debug;
use std::time::Duration;
use tokio::time::sleep;

pub struct Cli<CliBackend, RepositoryBackend, BackendEntity, PersistenceMessage = ()>
where
    CliBackend: Backend<BackendEntity, PersistenceMessage=PersistenceMessage>,
    CliBackend::Settings: Clone,
    RepositoryBackend: repository::backends::backend::Backend,
    BackendEntity: Debug,
{
    service_state: ServiceStateHandle<Self>,
}

impl<CliBackend, RepositoryBackend, BackendEntity, PersistenceMessage> ServiceData
for Cli<CliBackend, RepositoryBackend, BackendEntity, PersistenceMessage>
where
    CliBackend: Backend<BackendEntity, PersistenceMessage=PersistenceMessage>,
    CliBackend::Settings: Clone,
    RepositoryBackend: repository::backends::backend::Backend,
    BackendEntity: Debug,
{
    const SERVICE_ID: ServiceId = "Cli";
    type Settings = CliSettings<CliBackend::Settings>;
    type State = NoState<Self::Settings>;
    type StateOperator = NoOperator<Self::State>;
    type Message = NoMessage;
}

#[async_trait]
impl<CliBackend, RepositoryBackend, BackendEntity, PersistenceMessage> ServiceCore
for Cli<CliBackend, RepositoryBackend, BackendEntity, PersistenceMessage>
where
    CliBackend: Backend<BackendEntity, PersistenceMessage=PersistenceMessage> + Send + Debug + 'static,
    CliBackend::Settings: Send + Sync + Clone,
    RepositoryBackend: repository::backends::backend::Backend<Entity=BackendEntity> + Send + Debug + 'static,
    RepositoryBackend::Settings: Send + Sync + Clone,
    RepositoryBackend::Entity: Debug,
    BackendEntity: Debug,
{
    fn init(service_state: ServiceStateHandle<Self>) -> Result<Self, DynError> {
        Ok(Self { service_state })
    }

    async fn run(self) -> Result<(), DynError> {
        let Self { service_state } = self;
        let backend_settings = service_state.settings_reader.get_updated_settings().backend;
        let repository_relay = service_state
            .overwatch_handle
            .relay::<Repository<RepositoryBackend>>()
            .connect()
            .await?;

        let mut backend = CliBackend::new(
            backend_settings, service_state.overwatch_handle.clone(), repository_relay,
        );
        backend.run().await;

        sleep(Duration::from_secs(1)).await;
        service_state.overwatch_handle.shutdown().await;
        Ok(())
    }
}
