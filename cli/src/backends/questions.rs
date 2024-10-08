use async_trait::async_trait;
use crate::backends::backends::Backend;

pub struct QuestionsBackend {
    
}

#[async_trait]
impl Backend for QuestionsBackend {
    async fn init(&self) {
        println!("Welcome to CliQuiz!")
    }

    async fn loop_iteration(&self) {
        println!("Iter")
    }
}