use thiserror::Error;
use viuer::ViuError;

use crate::questions_repository::errors::QuestionsRepositoryError;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("questions repository error: {0}")]
    QuestionsRepositoryError(#[from] QuestionsRepositoryError),
    #[error("rendering error: {0}")]
    RenderError(#[from] ViuError)
}