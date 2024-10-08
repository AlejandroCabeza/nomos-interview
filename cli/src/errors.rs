use std::io;
use thiserror::Error;
use viuer::ViuError;

use repository::errors::RepositoryError;

#[derive(Error, Debug)]
pub enum Error {
    #[error("questions repository error: {0}")]
    Repository(#[from] RepositoryError),
    #[error("rendering error: {0}")]
    Render(#[from] ViuError),
    #[error("input error: {0}")]
    Input(#[from] io::Error)
}