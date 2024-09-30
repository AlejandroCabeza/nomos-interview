use async_trait::async_trait;
use overwatch_rs::services::{ServiceCore, ServiceData, ServiceId};
use overwatch_rs::services::handle::ServiceStateHandle;
use overwatch_rs::services::relay::{InboundRelay};
use overwatch_rs::services::state::{NoOperator, NoState};
use tokio::sync::oneshot::Sender;
use tracing::error;

use crate::backends::backend::Backend;
use crate::backends::backend_message::BackendMessage;
use crate::backends::pokemon::PokemonBackend;
use crate::messages::RepositoryMessage;
use crate::entities::Entity;

type BoxedBackend = Box<dyn Backend + Send>;

pub struct Repository {
    service_state: ServiceStateHandle<Self>,
    backend: BoxedBackend
}

impl ServiceData for Repository {
    const SERVICE_ID: ServiceId = "Repository";
    type Settings = ();
    type State = NoState<Self::Settings>;
    type StateOperator = NoOperator<Self::State>;
    type Message = RepositoryMessage;
}

#[async_trait]
impl ServiceCore for Repository {
    fn init(service_state: ServiceStateHandle<Self>) -> Result<Self, overwatch_rs::DynError> {
        let backend: BoxedBackend = Box::new(PokemonBackend::new(Some(0), Some(250)));
        Ok(Self { service_state, backend })
    }

    async fn run(self) -> Result<(), overwatch_rs::DynError> {
        let Self {
            service_state,
            mut backend,
        } = self;

        let mut inbound_relay = service_state.inbound_relay;
        get_repository_loop(&mut backend, &mut inbound_relay).await;

        Ok(())
    }
}

async fn get_repository_loop(
    backend: &mut BoxedBackend,
    inbound_relay: &mut InboundRelay<RepositoryMessage>
) {
    while let Some(message) = inbound_relay.recv().await {
        match message {
            RepositoryMessage::UpdateBackend(backend_message) => {
                update_backend(backend, backend_message);
            }
            RepositoryMessage::RequestEntity(sender) => {
                request_entity(backend, sender).await;
            }
        }
    }
}

fn update_backend(backend: &mut BoxedBackend, backend_message: BackendMessage) {
    match backend_message {
        BackendMessage::Pokemon => {
            *backend = Box::new(PokemonBackend::new(Some(0), Some(250)));
        }
    }
}

async fn request_entity(backend: &mut BoxedBackend, sender: Sender<Entity>) {
    loop {
        match backend.next().await {
            Ok(entity) => {
                send_entity(sender, entity);
                break;
            }
            Err(error) => {
                error!("Could get the next entity: {:?} ", error);
            }
        }
    }
}

fn send_entity(sender: Sender<Entity>, entity: Entity) {
    if let Err(error) = sender.send(entity) {
        error!("Could not send message to Cli network relay: {:?}", error)
    }
}
