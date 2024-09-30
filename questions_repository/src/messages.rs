use std::fmt::{Debug};
use overwatch_rs::services::relay::RelayMessage;
use tokio::sync::oneshot::Sender;
use crate::backends::backend_message::BackendMessage;
use crate::questions::Question;

#[derive(Debug)]
pub enum QuestionsRepositoryMessage {
    UpdateBackend(BackendMessage),
    Request(Sender<Question>)
}

impl RelayMessage for QuestionsRepositoryMessage {}
