use crate::backends::errors::BackendError;
use image::ImageError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error(transparent)]
    BackendError(#[from] BackendError),
    #[error(transparent)]
    FetchingImage(reqwest::Error),
    #[error(transparent)]
    ParsingImage(reqwest::Error),
    #[error(transparent)]
    LoadingImage(#[from] ImageError),
}
