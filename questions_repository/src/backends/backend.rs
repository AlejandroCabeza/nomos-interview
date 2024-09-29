use crate::backends::errors::BackendError;
use crate::questions::Question;

#[async_trait::async_trait]
pub trait QuestionsRepositoryBackend {
    async fn next(&self) -> Result<Question, BackendError>;
}
