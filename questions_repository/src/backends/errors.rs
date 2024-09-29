use thiserror::Error;

#[derive(Error, Debug)]
pub enum BackendError {
    #[error("failed making request: {0}")]
    RequestError(#[from] reqwest::Error),
}
