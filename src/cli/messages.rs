use overwatch_rs::services::relay::RelayMessage;
use crate::questions_repository::question::Question;

#[derive(Debug)]
pub enum CliMessage {
    New(Question)
}

impl RelayMessage for CliMessage {}
