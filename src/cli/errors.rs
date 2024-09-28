use viuer::ViuError;

use crate::questions_repository::errors::QuestionsRepositoryError;

#[derive(Debug)]
pub enum CliError {
    QuestionsRepositoryError(QuestionsRepositoryError),
    RenderError(ViuError)
}