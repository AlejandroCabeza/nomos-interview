use std::fmt::{Debug, Formatter};
use crate::questions_repository::question::Question;

#[async_trait::async_trait]
pub trait QuestionsRepositoryBackend {
    async fn next(&self) -> Result<Question, String>;
}

impl Debug for dyn QuestionsRepositoryBackend + Send {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "QuestionsRepositoryBackend")
    }
}