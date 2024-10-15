use cli::backends::questions::ImageGuessBackend;
use cli::backends::questions_settings::QuestionsBackendSettings;
use cli::Cli;
use overwatch_derive::Services;
use overwatch_rs::overwatch::OverwatchRunner;
use overwatch_rs::services::handle::ServiceHandle;
use overwatch_rs::services::ServiceData;
use repository::backends::pokemon::RandomPokemonBackend;
use repository::backends::pokemon_serializers::Pokemon;
use repository::backends::pokemon_settings::WhoIsThatPokemonBackendSettings;
use repository::Repository;

type _RepositoryBackend = RandomPokemonBackend;
type _RepositoryBackendSettings = <Repository<_RepositoryBackend> as ServiceData>::Settings;
type _CliBackend = ImageGuessBackend<Pokemon>;
type _CliBackendSettings = <Cli<_CliBackend, _RepositoryBackend, Pokemon> as ServiceData>::Settings;

#[derive(Services)]
pub struct WhoIsThatPokemonGame {
    repository: ServiceHandle<Repository<_RepositoryBackend>>,
    cli: ServiceHandle<Cli<_CliBackend, _RepositoryBackend, Pokemon>>,
}

impl WhoIsThatPokemonGame {
    pub fn run() {
        let repository_backend_settings = WhoIsThatPokemonBackendSettings::new(0, 151);
        let repository_settings = _RepositoryBackendSettings::new(repository_backend_settings);
        let cli_backend_settings = QuestionsBackendSettings::default();
        let cli_settings = _CliBackendSettings::new(cli_backend_settings);
        let settings = WhoIsThatPokemonGameServiceSettings {
            repository: repository_settings,
            cli: cli_settings,
        };
        let game = OverwatchRunner::<Self>::run(settings, None).expect("Overwatch runner failed");
        game.wait_finished();
    }
}
