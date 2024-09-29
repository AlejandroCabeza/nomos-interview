use image::ImageError;
use thiserror::Error;
use crate::backends::errors::BackendError;

#[derive(Error, Debug)]
pub enum QuestionsRepositoryError {
    #[error("backend error: {0}")]
    BackendError(#[from] BackendError),
    #[error("failed fetching image: {0}")]
    FetchingImage(reqwest::Error),
    #[error("failed parsing image: {0}")]
    ParsingImage(reqwest::Error),
    #[error("failed loading image: {0}")]
    LoadingImage(#[from] ImageError)
}