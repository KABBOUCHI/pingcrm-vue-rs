use std::env;
use std::sync::OnceLock;

use dotenv::dotenv;
use fang::asynk::async_queue::AsyncQueue;
use fang::NoTls;

pub use fang::asynk::async_queue::AsyncQueueable;
pub use fang::AsyncRunnable;

pub mod tasks;

static mut QUEUE: OnceLock<AsyncQueue<NoTls>> = OnceLock::new();

pub struct Queue;

impl Queue {
    pub async fn dispatch(task: &impl AsyncRunnable) -> anyhow::Result<()> {
        let queue = get_queue().await?;

        queue.insert_task(task).await?;

        Ok(())
    }

    pub async fn queue() -> anyhow::Result<&'static mut AsyncQueue<NoTls>> {
        get_queue().await
    }
}

async fn get_queue() -> anyhow::Result<&'static mut AsyncQueue<NoTls>> {
    dotenv().ok();

    match unsafe { QUEUE.get_mut() } {
        Some(queue) => Ok(queue),
        None => {
            let max_pool_size: u32 = 2;

            let db_uri = &env::var("DATABASE_URL").expect("DATABASE_URL must be set");

            let mut queue = AsyncQueue::builder()
                // Postgres database url
                .uri(db_uri)
                // Max number of connections that are allowed
                .max_pool_size(max_pool_size)
                .build();

            queue.connect(NoTls).await?;

            let _ = unsafe { QUEUE.set(queue) };

            match unsafe { QUEUE.get_mut() } {
                Some(queue) => Ok(queue),
                None => Err(anyhow::anyhow!("Failed to get queue")),
            }
        }
    }
}
