use crate::messages::handleable_message::HandleableMessage;
use crate::settings::PersistenceSettings;
use overwatch_rs::services::handle::ServiceStateHandle;
use overwatch_rs::services::relay::RelayMessage;
use overwatch_rs::services::state::{NoOperator, NoState};
use overwatch_rs::services::{ServiceCore, ServiceData, ServiceId};
use overwatch_rs::DynError;
use sqlx::SqlitePool;
use std::fmt::Debug;
use tracing::error;

pub struct Persistence<MessageType>
where
    MessageType: HandleableMessage + RelayMessage + Debug,
{
    service_state: ServiceStateHandle<Self>,
}

impl<MessageType> ServiceData for Persistence<MessageType>
where
    MessageType: HandleableMessage + RelayMessage + Debug,
{
    const SERVICE_ID: ServiceId = "Persistence";
    type Settings = PersistenceSettings;
    type State = NoState<Self::Settings>;
    type StateOperator = NoOperator<Self::State>;
    type Message = MessageType;
}

#[async_trait::async_trait]
impl<MessageType> ServiceCore for Persistence<MessageType>
where
    MessageType: HandleableMessage + RelayMessage + Send + Debug,
{
    fn init(service_state: ServiceStateHandle<Self>) -> Result<Self, DynError> {
        Ok(Self { service_state })
    }

    async fn run(self) -> Result<(), DynError> {
        let Self {
            service_state,
        } = self;

        let mut inbound_relay = service_state.inbound_relay;
        let settings = service_state.settings_reader.get_updated_settings();
        let pool = SqlitePool::connect(settings.database_url()).await?;

        while let Some(message) = inbound_relay.recv().await {
            if let Err(error) = message.handle(&pool).await {
                error!("Error handling message: {:#?}", error);
            }
        }

        Ok(())
    }
}



