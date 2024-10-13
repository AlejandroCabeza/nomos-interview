use overwatch_rs::services::relay::RelayError;
use repository::errors::RepositoryError;
use std::io::Error as IoError;
use thiserror::Error;
use tokio::sync::oneshot::error::RecvError;
use viuer::ViuError;

#[derive(Error, Debug)]
pub enum BackendError {
    #[error(transparent)]
    Relay(#[from] RelayError),
    #[error(transparent)]
    OneshotReceive(#[from] RecvError),
    #[error(transparent)]
    Repository(#[from] RepositoryError),
    #[error(transparent)]
    ImageRender(#[from] ViuError),
    #[error(transparent)]
    IO(#[from] IoError),
    #[error("wrong entity")]
    WrongEntity,
}
