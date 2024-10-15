use cli::backends::gallery::Gallery;
use cli::backends::gallery_settings::GallerySettings;
use cli::Cli;
use overwatch_derive::Services;
use overwatch_rs::overwatch::OverwatchRunner;
use overwatch_rs::services::handle::ServiceHandle;
use overwatch_rs::services::ServiceData;
use repository::backends::nftscan::NFTScanBackend;
use repository::backends::nftscan_serializers::NFT;
use repository::backends::nftscan_settings::NFTScanBackendSettings;
use repository::Repository;

type _RepositoryBackend = NFTScanBackend;
type _RepositoryBackendSettings = <Repository<_RepositoryBackend> as ServiceData>::Settings;
type _CliBackend = Gallery<NFT>;
type _CliBackendSettings = <Cli<_CliBackend, _RepositoryBackend, NFT> as ServiceData>::Settings;

#[derive(Services)]
pub struct NFTViewer {
    repository: ServiceHandle<Repository<_RepositoryBackend>>,
    cli: ServiceHandle<Cli<_CliBackend, _RepositoryBackend, NFT>>,
}

impl NFTViewer {
    pub fn run(api_key: String, collection_address: String) {
        let repository_backend_settings = NFTScanBackendSettings::new(api_key, collection_address);
        let repository_settings = _RepositoryBackendSettings::new(repository_backend_settings);
        let cli_backend_settings = GallerySettings::default();
        let cli_settings = _CliBackendSettings::new(cli_backend_settings);
        let settings = NFTViewerServiceSettings {
            repository: repository_settings,
            cli: cli_settings,
        };
        let game = OverwatchRunner::<Self>::run(settings, None).expect("Overwatch runner failed");
        game.wait_finished();
    }
}
