extern crate queue;

use anyhow::Result;
use dotenv::dotenv;
use std::{env, time::Duration};
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    ensemble::setup(&env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
        .expect("Failed to set up database pool.");

    dotenv().ok();

    let database_url =
        env::var("DATABASE_URL").map_err(|_| anyhow::anyhow!("Failed to get DATABASE_URL"))?;

    let worker = job_queue::Worker::builder()
        .max_connections(10)
        .worker_count(4)
        .connect(&database_url)
        .await
        .map_err(|_| anyhow::anyhow!("Failed to connect to database"))?;

    worker.start().await?;

    loop {
        sleep(Duration::from_millis(100)).await;
    }
}
