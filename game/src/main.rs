use tracing;
use overwatch_derive::Services;
use overwatch_rs::overwatch::OverwatchRunner;
use overwatch_rs::services::handle::ServiceHandle;
use cli::Cli;
use questions_repository::QuestionsRepository;

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
