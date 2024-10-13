use crate::backends::backend::Backend;
use crate::backends::errors::BackendError;
use crate::backends::questions_settings::QuestionsBackendSettings;
use crate::utils::terminal_utils::{clear_screen, get_normalized_input};
use crate::utils::viuer_utils::debug_image_config;
use async_trait::async_trait;
use overwatch_rs::services::relay::OutboundRelay;
use repository::entities::image_guess::ImageGuess;
use repository::RepositoryMessage;
use std::fmt::Debug;
use tokio::sync::oneshot;
use tracing::error;
use viuer::{print as print_image_in_terminal, Config};

pub struct QuestionsBackend {
    repository_network_relay: OutboundRelay<RepositoryMessage<ImageGuess>>,
    image_config: Config,
    score: u16,
}

impl Debug for QuestionsBackend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let image_config_debug = debug_image_config(&self.image_config);
        f.debug_struct("QuestionsBackend")
            .field(
                "repository_network_relay",
                &"OutboundRelay<RepositoryMessage<IdentifyImageQuestion>>",
            )
            .field("image_config", &image_config_debug)
            .field("score", &self.score)
            .finish()
    }
}

impl QuestionsBackend {
    async fn formulate_identify_image_question(
        &self,
        question: ImageGuess,
    ) -> Result<String, BackendError> {
        let image = question.image().await.map_err(BackendError::Repository)?;
        print_image_in_terminal(&image, &self.image_config).map_err(BackendError::ImageRender)?;
        println!("> {}", question.prompt());
        Ok(String::from(question.answer()))
    }

    fn get_answer_input(&self) -> Result<String, BackendError> {
        match get_normalized_input() {
            Ok(answer) => Ok(answer),
            Err(error) => {
                println!("> Error parsing input");
                error!("Could not parse answer input: {}", error);
                Err(BackendError::IO(error))
            }
        }
    }

    fn validate_answer(&mut self, answer: String, expected_answer: String) -> bool {
        if answer == expected_answer {
            self.score += 1;
            println!("> That's a hit! Your current score is: {}", self.score);
            true
        } else {
            println!(
                "> Whoops! That was not quite it, the answer was: {}",
                expected_answer
            );
            false
        }
    }
}

#[async_trait]
impl Backend for QuestionsBackend {
    type Settings = QuestionsBackendSettings;
    type Entity = ImageGuess;

    fn new(
        settings: Self::Settings,
        outbound_relay: OutboundRelay<RepositoryMessage<Self::Entity>>,
    ) -> Self {
        Self {
            repository_network_relay: outbound_relay,
            image_config: settings.image_config,
            score: 0,
        }
    }

    async fn _init(&self) {
        println!("> Welcome to CliQuiz!")
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
        let expected_answer = self.formulate_identify_image_question(entity).await?;
        let answer = self.get_answer_input()?;
        self.validate_answer(answer, expected_answer);
        Ok(())
    }

    async fn _finalize(&mut self) {
        println!("> You scored a total of: {}!", self.score);
    }
}
