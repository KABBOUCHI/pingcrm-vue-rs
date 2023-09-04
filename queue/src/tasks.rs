use async_trait::async_trait;
use ensemble::Model;
use fang::{FangError, AsyncQueueable, AsyncRunnable};
use models::User;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "fang::serde")]
pub struct MyTask {
    pub number: u16,
}

impl MyTask {
    pub fn new(number: u16) -> Self {
        Self { number }
    }
}

#[async_trait]
#[typetag::serde]
impl AsyncRunnable for MyTask {
    async fn run(&self, _queue: &mut dyn AsyncQueueable) -> Result<(), FangError> {
        println!("MyTask: {}", self.number);

        let users = User::all().await.map_err(|_| FangError {
            description: "Failed to get users".to_string(),
        })?;

        dbg!(users);

        Ok(())
    }
}
