use crate::questions_repository::backends::errors::BackendError;
use crate::questions_repository::question::Question;

#[async_trait::async_trait]
pub trait QuestionsRepositoryBackend {
    async fn next(&self) -> Result<Question, BackendError>;
}
