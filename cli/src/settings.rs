#[derive(Clone, Debug)]
pub struct CliSettings<BackendSettings> {
    pub backend: BackendSettings,
}

impl<BackendSettings> CliSettings<BackendSettings> {
    pub fn new(backend: BackendSettings) -> Self {
        Self { backend }
    }
}
