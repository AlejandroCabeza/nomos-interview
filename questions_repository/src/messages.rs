use std::fmt::{Debug};
use overwatch_rs::services::relay::RelayMessage;
use crate::backends::backend_message::BackendMessage;

#[derive(Debug)]
pub enum QuestionsRepositoryMessage {
    UpdateBackend(BackendMessage),
    Request
}

impl RelayMessage for QuestionsRepositoryMessage {}
