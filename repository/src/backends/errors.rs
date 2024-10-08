use thiserror::Error;

#[derive(Error, Debug)]
pub enum BackendError {
    #[error("failed making request: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("failed serializing/deserializing: {0}")]
    SerDe(#[from] serde_json::Error),
    #[error("no more entities available")]
    NoMoreEntities
}
