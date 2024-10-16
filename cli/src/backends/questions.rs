use crate::backends::backend::Backend;
use crate::backends::errors::BackendError;
use crate::backends::questions_settings::ImageGuessBackendSettings;
use crate::backends::utils::request_entity_to_repository;
use crate::utils::terminal_utils::{clear_screen, get_normalized_input};
use crate::utils::viuer_utils::debug_image_config;
use async_trait::async_trait;
use overwatch_rs::overwatch::handle::OverwatchHandle;
use overwatch_rs::services::relay::OutboundRelay;
use persistence::{Persistence, PersistenceQuestionsMessage};
use repository::entities::image_guess::ImageGuess;
use repository::RepositoryMessage;
use std::fmt::Debug;
use tracing::error;
use viuer::{print as print_image_in_terminal, Config};

pub struct ImageGuessBackend<Entity: Debug> {
    overwatch_handle: OverwatchHandle,
    repository_relay: OutboundRelay<RepositoryMessage<Entity>>,
    image_config: Config,
    score: u16,
}

impl<Entity: Debug> Debug for ImageGuessBackend<Entity> {
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

impl<Entity> ImageGuessBackend<Entity>
where
    Entity: ImageGuess + Sync + Debug,
{
    async fn formulate_identify_image_question(
        &self,
        question: Entity,
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
impl<Entity> Backend<Entity> for ImageGuessBackend<Entity>
where
    Entity: ImageGuess + Send + Sync + Debug,
{
    type Settings = ImageGuessBackendSettings;
    type PersistenceMessage = PersistenceQuestionsMessage;

    fn new(
        settings: Self::Settings,
        overwatch_handle: OverwatchHandle,
        repository_relay: OutboundRelay<RepositoryMessage<Entity>>,
    ) -> Self {
        Self {
            overwatch_handle,
            repository_relay,
            image_config: settings.image_config,
            score: 0,
        }
    }

    async fn _init(&mut self) {
        println!("> Welcome to CliQuiz!")
    }

    async fn _finalize(&mut self) {
        println!("> You scored a total of: {}!", self.score);
        let persistence_relay = self.overwatch_handle.relay::<Persistence<Self::PersistenceMessage>>().connect().await.unwrap();
        if let Err(error) = persistence_relay.send(Self::PersistenceMessage::SaveScore(self.score)).await {
            error!("Failed saving user score: {:#?}", error);
        }
    }

    async fn request_entity(&self) -> Result<Entity, BackendError> {
        request_entity_to_repository(&self.repository_relay).await
    }

    async fn handle_entity(&mut self, entity: Entity) -> Result<(), BackendError> {
        clear_screen();
        let expected_answer = self.formulate_identify_image_question(entity).await?;
        let answer = self.get_answer_input()?;
        self.validate_answer(answer, expected_answer);
        Ok(())
    }
}
