use overwatch_rs::services::relay::RelayMessage;
use crate::questions_repository::backends::backend::QuestionsRepositoryBackend;

#[derive(Debug)]
pub enum QuestionsRepositoryMessage {
    ChangeBackend(Box<dyn QuestionsRepositoryBackend + Send>),
    Request
}

impl RelayMessage for QuestionsRepositoryMessage {}
