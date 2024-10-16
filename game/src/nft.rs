use cli::{Cli, GalleryBackend, GalleryBackendSettings};
use overwatch_derive::Services;
use overwatch_rs::overwatch::OverwatchRunner;
use overwatch_rs::services::handle::ServiceHandle;
use overwatch_rs::services::ServiceData;
use repository::{NFTScanBackend, NFTScanBackendSettings, Repository, NFT};

type _RepositoryBackend = NFTScanBackend;
type _Repository = Repository<_RepositoryBackend>;
type _RepositoryBackendSettings = <_Repository as ServiceData>::Settings;

type _CliBackend = GalleryBackend<NFT>;
type _Cli = Cli<_CliBackend, _RepositoryBackend, NFT>;
type _CliBackendSettings = <_Cli as ServiceData>::Settings;

#[derive(Services)]
pub struct NFTGallery {
    repository: ServiceHandle<_Repository>,
    cli: ServiceHandle<_Cli>,
}

impl NFTGallery {
    pub fn run(api_key: String, collection_address: String) {
        let repository_backend_settings = NFTScanBackendSettings::new(api_key, collection_address);
        let repository_settings = _RepositoryBackendSettings::new(repository_backend_settings);
        let cli_backend_settings = GalleryBackendSettings::default();
        let cli_settings = _CliBackendSettings::new(cli_backend_settings);
        let settings = NFTGalleryServiceSettings {
            repository: repository_settings,
            cli: cli_settings,
        };
        let game = OverwatchRunner::<Self>::run(settings, None).expect("Overwatch runner failed");
        game.wait_finished();
    }
}
