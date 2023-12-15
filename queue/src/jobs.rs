use ensemble::Model;
use job_queue::{async_trait, serde, typetag, Error, Job};
use models::User;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(crate = "job_queue::serde")]
pub struct MyJob {
    pub number: u16,
}

impl MyJob {
    pub fn new(number: u16) -> Self {
        Self { number }
    }
}

#[async_trait::async_trait]
#[typetag::serde]
impl Job for MyJob {
    async fn handle(&self) -> Result<(), Error> {
        println!("MyTask: {}", self.number);

        let users = User::all()
            .await
            .map_err(|_| Error::Message("Failed to get users".to_string()))?;

        dbg!(users);

        Ok(())
    }
}
