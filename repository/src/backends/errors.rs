use thiserror::Error;

#[derive(Error, Debug)]
pub enum BackendError {
    #[error(transparent)]
    RequestError(#[from] reqwest::Error),
    #[error(transparent)]
    SerDe(#[from] serde_json::Error),
    #[error("no more entities available")]
    NoMoreEntities,
}
