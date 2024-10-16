use crate::utils::viuer_utils::{clone_image_config, debug_image_config, get_image_config};
use std::fmt::Debug;
use viuer::Config;

pub struct ImageGuessBackendSettings {
    pub(crate) image_config: Config,
}

impl ImageGuessBackendSettings {
    pub fn new(image_config: Config) -> Self {
        Self { image_config }
    }
}

impl Default for ImageGuessBackendSettings {
    fn default() -> Self {
        Self {
            image_config: get_image_config(),
        }
    }
}

impl Debug for ImageGuessBackendSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let image_config_debug = debug_image_config(&self.image_config);
        f.debug_struct("ImageGuessBackendSettings")
            .field("image_config", &image_config_debug)
            .finish()
    }
}

impl Clone for ImageGuessBackendSettings {
    fn clone(&self) -> Self {
        Self {
            image_config: clone_image_config(&self.image_config),
        }
    }
}
