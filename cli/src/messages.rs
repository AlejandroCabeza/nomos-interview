use overwatch_rs::services::relay::RelayMessage;
use questions_repository::questions::Question;

#[derive(Debug)]
pub enum CliMessage {
    New(Question)
}

impl RelayMessage for CliMessage {}
