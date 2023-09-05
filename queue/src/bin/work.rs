use std::{env, time::Duration};

use anyhow::Result;
use dotenv::dotenv;
use fang::{asynk::async_worker_pool::AsyncWorkerPool, AsyncQueue, NoTls};
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    ensemble::setup(&env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
        .await
        .expect("Failed to set up database pool.");

    let queue = queue::Queue::queue().await?;

    let mut pool: AsyncWorkerPool<AsyncQueue<NoTls>> = AsyncWorkerPool::builder()
        .number_of_workers(3_u32)
        .queue(queue.clone())
        .build();

    pool.start().await;

    loop {
        sleep(Duration::from_millis(300)).await;
    }
}
