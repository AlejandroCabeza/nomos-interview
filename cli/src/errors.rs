use thiserror::Error;
use viuer::ViuError;

use repository::errors::RepositoryError;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("questions repository error: {0}")]
    QuestionsRepositoryError(#[from] RepositoryError),
    #[error("rendering error: {0}")]
    RenderError(#[from] ViuError)
}