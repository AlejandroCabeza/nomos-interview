use overwatch_rs::DynError;
use overwatch_rs::services::handle::ServiceStateHandle;
use overwatch_rs::services::{ServiceCore, ServiceData, ServiceId};
use overwatch_rs::services::relay::{NoMessage, OutboundRelay};
use overwatch_rs::services::state::{NoOperator, NoState};
use viuer::{print as print_image_in_terminal, Config};
use tracing::{error};
use repository::entities::{IdentifyImageQuestion, Entity};
use repository::{Repository, RepositoryMessage};
use tokio::sync::oneshot;
use crate::errors::Error;
use crate::utils::{clear_screen, continue_or_exit, get_input, press_enter_to_continue, ContinueCommand};

pub struct Cli {
    service_state: ServiceStateHandle<Self>,
    score: u16
}

impl ServiceData for Cli {
    const SERVICE_ID: ServiceId = "Cli";
    type Settings = ();
    type State = NoState<Self::Settings>;
    type StateOperator = NoOperator<Self::State>;
    type Message = NoMessage;
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
        let repository_network_relay = service_state.overwatch_handle.relay::<Repository>().connect().await?;

        get_cli_service_loop(&repository_network_relay, &image_config, &mut score).await;

        service_state.overwatch_handle.shutdown().await;
        Ok(())
    }
}

async fn get_cli_service_loop(
    repository_network_relay: &OutboundRelay<RepositoryMessage>,
    image_config: &Config,
    score: &mut u16
) {
    println!("Welcome to Cli! To exit, type `quit`.");
    loop {
        // Request Entity
        let (sender, receiver) = oneshot::channel();
        if let Err(error) = repository_network_relay.send(RepositoryMessage::RequestEntity(sender)).await {
            // TODO: if more than 2 failures then exit? or maybe just warning.
            error!("Could not send Request to QuestionsRepository: {:?}", error);
            continue
        }

        // Receive Entity
        match receiver.await {
            Ok(entity) => {
                match entity {
                    Entity::IdentifyImageQuestion(identify_image_question) => {
                        clear_screen();
                        let expected_answer = match formulate_identify_image_question(&identify_image_question, image_config).await {
                            Ok(answer) => answer,
                            Err(error) => {
                                println!("> An error happened. Let's try again!");
                                error!("Could not formulate IdentifyImageQuestion: {:?}", error);
                                continue
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
                    Entity::NFTEntity(element) => {
                        // clear_screen();
                        println!("-> {}", &element.name());
                        if let Ok(image) = element.image().await.map_err(Error::Repository) {
                            if let Err(error) = print_image_in_terminal(&image, image_config) {
                                error!("Error while printing image: {}", error);
                            }
                        }
                        press_enter_to_continue();
                    }
                };
            }
            Err(error) => {
                error!("Could not receive message: {}", error);
                println!("An error happened. Let's try again!");
            }
        }

        match continue_or_exit() {
            Ok(ContinueCommand::Continue) => continue,
            Ok(ContinueCommand::Exit) => {
                println!("> Exiting...");
                break
            },
            Err(error) => {
                error!("Found an error when parsing input: {}", error);
                println!("An error happened. Let's try again!");
                continue
            }
        };
    }
}

async fn formulate_identify_image_question(question: &IdentifyImageQuestion, image_config: &Config) -> Result<String, Error> {
    let image = question.image().await.map_err(Error::Repository)?;
    print_image_in_terminal(&image, image_config).map_err(Error::Render)?;
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
