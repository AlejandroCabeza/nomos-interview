use overwatch_rs::services::relay::RelayMessage;
use std::fmt::Debug;
use tokio::sync::oneshot::Sender;

#[derive(Debug)]
pub enum RepositoryMessage<B>
where
    B: Debug,
{
    RequestEntity(Sender<B>),
}

impl<B> RelayMessage for RepositoryMessage<B> where B: Debug + 'static {}
