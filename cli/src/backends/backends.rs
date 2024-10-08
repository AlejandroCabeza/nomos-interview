use async_trait::async_trait;
use tracing::error;
use crate::utils::{continue_or_exit, ContinueCommand};

#[async_trait]
pub trait Backend {
    async fn init(&self);

    async fn loop_iteration(&self);

    async fn main_loop(&self) {
        loop {
            self.loop_iteration().await;
            match continue_or_exit() {
                Ok(ContinueCommand::Continue) => continue,
                Ok(ContinueCommand::Exit) => break,
                Err(error) => {
                    error!("Found an error when parsing input: {}", error);
                    break;
                }
            };
        }
    }

    async fn run(&self) {
        self.init().await;
        self.main_loop().await;
    }
}
