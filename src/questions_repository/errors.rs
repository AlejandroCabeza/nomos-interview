use image::ImageError;

// use crate::questions_repository::backends::errors::BackendError;

#[derive(Debug)]
pub enum QuestionsRepositoryError {
    // BackendError(BackendError),
    FailedFetchingImage(reqwest::Error),
    FailedParsingImage(reqwest::Error),
    FailedLoadingImage(ImageError)
}