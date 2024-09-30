use async_trait::async_trait;
use overwatch_rs::services::{ServiceCore, ServiceData, ServiceId};
use overwatch_rs::services::handle::ServiceStateHandle;
use overwatch_rs::services::relay::{InboundRelay};
use overwatch_rs::services::state::{NoOperator, NoState};
use tokio::sync::oneshot::Sender;
use tracing::error;

use crate::backends::backend::QuestionsRepositoryBackend;
use crate::backends::backend_message::BackendMessage;
use crate::backends::pokemon::PokemonBackend;
use crate::messages::QuestionsRepositoryMessage;
use crate::questions::Question;

type BoxedBackend = Box<dyn QuestionsRepositoryBackend + Send>;

pub struct QuestionsRepository {
    service_state: ServiceStateHandle<Self>,
    questions_backend: BoxedBackend
}

impl ServiceData for QuestionsRepository {
    const SERVICE_ID: ServiceId = "QuestionsRepository";
    type Settings = ();
    type State = NoState<Self::Settings>;
    type StateOperator = NoOperator<Self::State>;
    type Message = QuestionsRepositoryMessage;
}

#[async_trait]
impl ServiceCore for QuestionsRepository {
    fn init(service_state: ServiceStateHandle<Self>) -> Result<Self, overwatch_rs::DynError> {
        let questions_backend: BoxedBackend = Box::new(PokemonBackend::new(Some(0), Some(250)));
        Ok(Self { service_state, questions_backend })
    }

    async fn run(self) -> Result<(), overwatch_rs::DynError> {
        let Self {
            service_state,
            mut questions_backend,
        } = self;

        let mut inbound_relay = service_state.inbound_relay;
        get_questions_repository_loop(&mut questions_backend, &mut inbound_relay).await;

        Ok(())
    }
}

async fn get_questions_repository_loop(
    questions_backend: &mut BoxedBackend,
    inbound_relay: &mut InboundRelay<QuestionsRepositoryMessage>
) {
    while let Some(message) = inbound_relay.recv().await {
        match message {
            QuestionsRepositoryMessage::UpdateBackend(backend_message) => {
                update_backend(questions_backend, backend_message);
            }
            QuestionsRepositoryMessage::Request(sender) => {
                request_question(questions_backend, sender).await;
            }
        }
    }
}

fn update_backend(questions_backend: &mut BoxedBackend, backend_message: BackendMessage) {
    match backend_message {
        BackendMessage::Pokemon => {
            *questions_backend = Box::new(PokemonBackend::new(Some(0), Some(250)));
        }
    }
}

async fn request_question(questions_backend: &mut BoxedBackend, sender: Sender<Question>) {
    loop {
        match questions_backend.next().await {
            Ok(next_question) => {
                send_question(sender, next_question);
                break;
            }
            Err(error) => {
                error!("Could get the next question: {:?} ", error);
            }
        }
    }
}

fn send_question(sender: Sender<Question>, question: Question) {
    if let Err(error) = sender.send(question) {
        error!("Could not send message to Cli network relay: {:?}", error)
    }
}
