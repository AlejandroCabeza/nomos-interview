use overwatch_rs::DynError;
use overwatch_rs::services::handle::ServiceStateHandle;
use overwatch_rs::services::{ServiceCore, ServiceData, ServiceId};
use overwatch_rs::services::relay::{InboundRelay, OutboundRelay};
use overwatch_rs::services::state::{NoOperator, NoState};
use viuer::{print as print_image_in_terminal, Config};
use tracing::{error};
use crate::cli::errors::CliError;
use crate::questions_repository::question::{IdentifyImage, Question};
use crate::questions_repository::service::{QuestionsRepository};
use crate::cli::messages::CliMessage;
use crate::cli::utils::{clear_screen, get_input, press_enter_to_continue};
use crate::questions_repository::messages::QuestionsRepositoryMessage;

pub struct Cli {
    service_state: ServiceStateHandle<Self>,
    score: u16
}

impl ServiceData for Cli {
    const SERVICE_ID: ServiceId = "Cli";
    type Settings = ();
    type State = NoState<Self::Settings>;
    type StateOperator = NoOperator<Self::State>;
    type Message = CliMessage;
}

#[async_trait::async_trait]
impl ServiceCore for Cli {
    fn init(service_state: ServiceStateHandle<Self>) -> Result<Self, DynError> {
        Ok(Self { service_state, score: 0 })
    }

    async fn run(self) -> Result<(), DynError> {
        let Self {
            service_state,
            mut score
        } = self;
        let image_config = get_image_config();

        let questions_repository_network_relay = service_state.overwatch_handle.relay::<QuestionsRepository>().connect().await?;
        let mut inbound_relay = service_state.inbound_relay;

        get_cli_service_loop(&questions_repository_network_relay, &mut inbound_relay, &image_config, &mut score).await;

        service_state.overwatch_handle.shutdown().await;
        Ok(())
    }
}

async fn get_cli_service_loop(
    questions_repository_network_relay: &OutboundRelay<QuestionsRepositoryMessage>,
    inbound_relay: &mut InboundRelay<CliMessage>,
    image_config: &Config,
    score: &mut u16
) {
    println!("Welcome to CliQuiz! To exit, type `quit`.");
    loop {
        // Request Question
        if let Err(error) = questions_repository_network_relay.send(QuestionsRepositoryMessage::Request).await {
            // TODO: if more than 2 failures then exit? or maybe just warning.
            error!("Could not send Request to QuestionsRepository: {:?}", error);
            continue
        }

        // Make Question
        if let Some(message) = inbound_relay.recv().await {
            let expected_answer = match message {
                CliMessage::New(Question::IdentifyImage(identify_image_question)) => {
                    clear_screen();
                    match formulate_identify_image_question(&identify_image_question, image_config).await {
                        Ok(answer) => answer,
                        Err(error) => {
                            println!("> An error happened. Let's try again!");
                            error!("Could not formulate IdentifyImageQuestion: {:?}", error);
                            continue
                        }
                    }
                }
            };

            // Parse Answer
            let answer_input = get_input();
            match answer_input {
                Ok(answer) => {
                    let answer = answer.trim().to_ascii_lowercase();
                    if answer == "quit" {
                        println!("> You reached a score of: {}!", score);
                        break
                    }
                    check_answer(answer, expected_answer, score);
                }
                Err(error) => {
                    println!("> Error parsing input");
                    error!("Could not parse answer input: {}", error);
                }
            }
        }
    }
    println!("> Exiting...")
}

async fn formulate_identify_image_question(question: &IdentifyImage, image_config: &Config) -> Result<String, CliError> {
    let image = question.image().await.map_err(CliError::QuestionsRepositoryError)?;
    print_image_in_terminal(&image, image_config).map_err(CliError::RenderError)?;
    println!("> {}", question.prompt());
    Ok(String::from(question.answer()))
}

fn get_image_config() -> Config {
    Config {
        width: Some(48),
        height: Some(48),
        ..Default::default()
    }
}

fn check_answer(answer: String, expected_answer: String, score: &mut u16) {
    if answer == expected_answer {
        *score += 1;
        println!("> That's a hit! Your current score is: {}", score)
    }
    else {
        println!("> Woops! That was not quite it, the answer was: {}", expected_answer)
    }

    press_enter_to_continue()
}
