use tracing;
use overwatch_derive::Services;
use overwatch_rs::overwatch::OverwatchRunner;
use overwatch_rs::services::handle::ServiceHandle;
use cli::Cli;
use repository::Repository;

#[derive(Services)]
pub struct Game {
    repository: ServiceHandle<Repository>,
    cli: ServiceHandle<Cli>
}


fn main() {
    let game = OverwatchRunner::<Game>::run(
        GameServiceSettings { repository: (), cli: () },
        None
    ).expect("Overwatch runner failed");
    game.wait_finished();
}
