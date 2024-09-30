use std::fmt::{Debug};
use overwatch_rs::services::relay::RelayMessage;
use tokio::sync::oneshot::Sender;
use crate::backends::backend_message::BackendMessage;
use crate::entities::Entity;

#[derive(Debug)]
pub enum RepositoryMessage {
    UpdateBackend(BackendMessage),
    RequestEntity(Sender<Entity>)
}

impl RelayMessage for RepositoryMessage {}
