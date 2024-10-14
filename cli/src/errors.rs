use crate::backends::errors::BackendError;
use repository::errors::RepositoryError;
use std::io;
use thiserror::Error;
use viuer::ViuError;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Repository(#[from] RepositoryError),
    #[error(transparent)]
    Render(#[from] ViuError),
    #[error(transparent)]
    IO(#[from] io::Error),
    #[error(transparent)]
    Backend(#[from] BackendError),
}
