use cli::{Cli, ImageGuessBackend, ImageGuessBackendSettings};
use overwatch_derive::Services;
use overwatch_rs::overwatch::OverwatchRunner;
use overwatch_rs::services::handle::ServiceHandle;
use overwatch_rs::services::ServiceData;
use persistence::{Persistence, PersistenceQuestionsMessage};
use repository::{Pokemon, RandomPokemonBackend, RandomPokemonBackendSettings, Repository};

type _PersistenceMessage = PersistenceQuestionsMessage;
type _Persistence = Persistence<_PersistenceMessage>;
type _PersistenceSettings = <_Persistence as ServiceData>::Settings;

type _RepositoryBackend = RandomPokemonBackend;
type _Repository = Repository<_RepositoryBackend>;
type _RepositoryBackendSettings = <_Repository as ServiceData>::Settings;

type _CliBackend = ImageGuessBackend<Pokemon>;
type _Cli = Cli<_CliBackend, _RepositoryBackend, Pokemon, _PersistenceMessage>;
type _CliBackendSettings = <_Cli as ServiceData>::Settings;


#[derive(Services)]
pub struct WhoIsThatPokemonGame {
    repository: ServiceHandle<_Repository>,
    cli: ServiceHandle<_Cli>,
    persistence: ServiceHandle<_Persistence>,
}

impl WhoIsThatPokemonGame {
    pub fn run(database_url: String) {
        let repository_backend_settings = RandomPokemonBackendSettings::new(0, 151);
        let repository_settings = _RepositoryBackendSettings::new(repository_backend_settings);
        let cli_backend_settings = ImageGuessBackendSettings::default();
        let cli_settings = _CliBackendSettings::new(cli_backend_settings);
        let persistence_settings = _PersistenceSettings::new(database_url);
        let settings = WhoIsThatPokemonGameServiceSettings {
            repository: repository_settings,
            cli: cli_settings,
            persistence: persistence_settings,
        };
        let game = OverwatchRunner::<Self>::run(settings, None).expect("Overwatch runner failed");
        game.wait_finished();
    }
}
