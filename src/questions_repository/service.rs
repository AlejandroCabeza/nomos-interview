use async_trait::async_trait;
use overwatch_rs::services::{ServiceCore, ServiceData, ServiceId};
use overwatch_rs::services::handle::ServiceStateHandle;
use overwatch_rs::services::state::{NoOperator, NoState};
use tracing::error;
use crate::cli::messages::CliMessage;
use crate::cli::service::{Cli};
use crate::questions_repository::backends::backend::QuestionsRepositoryBackend;
use crate::questions_repository::backends::pokemon::PokemonBackend;
use crate::questions_repository::messages::QuestionsRepositoryMessage;

pub struct QuestionsRepository {
    service_state: ServiceStateHandle<Self>,
    questions_backend: Box<dyn QuestionsRepositoryBackend + Send>
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
        let questions_backend: Box<dyn QuestionsRepositoryBackend + Send> = Box::new(PokemonBackend::new(Some(0), Some(250)));
        Ok(Self { service_state, questions_backend })
    }

    async fn run(self) -> Result<(), overwatch_rs::DynError> {
        let Self {
            service_state,
            mut questions_backend,
        } = self;

        let cli_network_relay = service_state.overwatch_handle.relay::<Cli>().connect().await.expect("Failed fetching relay to Cli from QuestionsRepository.");
        let mut inbound_relay = service_state.inbound_relay;

        let questions_loop = async {
            loop {
                while let Some(message) = inbound_relay.recv().await {
                    match message {
                        QuestionsRepositoryMessage::ChangeBackend(new_backend) => {
                            questions_backend = new_backend
                        }
                        QuestionsRepositoryMessage::Request => {
                            loop {
                                match questions_backend.next().await {
                                    Ok(next_question) => {
                                        let send_result= cli_network_relay.send(CliMessage::New(next_question)).await;
                                        match send_result {
                                            Ok(()) => {
                                                break;
                                            }
                                            Err(error) => {
                                                error!("Could not send message to Cli network relay: {:?}", error);
                                            }
                                        }
                                    }
                                    Err(error) => {
                                        error!("Could get the next question: {:?} ", error);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        };

        questions_loop.await;

        Ok(())
    }
}