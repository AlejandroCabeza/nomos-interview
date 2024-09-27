mod cli;
mod questions_repository;

use overwatch_derive::Services;
use overwatch_rs::overwatch::OverwatchRunner;
use overwatch_rs::services::handle::ServiceHandle;
use crate::cli::service::Cli;
use crate::questions_repository::service::QuestionsRepository;

#[derive(Services)]
pub struct Game {
    questions_repository: ServiceHandle<QuestionsRepository>,
    cli: ServiceHandle<Cli>
}


fn main() {
    let game = OverwatchRunner::<Game>::run(
        GameServiceSettings { questions_repository: (), cli: () },
        None
    ).expect("Overwatch runner failed");
    game.wait_finished();
}
