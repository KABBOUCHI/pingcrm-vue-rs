use std::env;
use std::sync::{Mutex, OnceLock};

use dotenv::dotenv;
use fang::asynk::async_queue::AsyncQueue;
use fang::NoTls;

pub use fang::asynk::async_queue::AsyncQueueable;
pub use fang::AsyncRunnable;

pub mod tasks;

static QUEUE: OnceLock<Mutex<AsyncQueue<NoTls>>> = OnceLock::new();

pub struct Queue;

impl Queue {
    pub async fn dispatch(task: &impl AsyncRunnable) -> anyhow::Result<()> {
        let mut queue = get_queue().await?;

        queue.insert_task(task).await?;

        Ok(())
    }

    pub async fn queue() -> anyhow::Result<AsyncQueue<NoTls>> {
        get_queue().await
    }
}

async fn get_queue() -> anyhow::Result<AsyncQueue<NoTls>> {
    dotenv().ok();

    match QUEUE.get() {
        Some(queue) => {
            let queue = queue.lock().map_err(|_| anyhow::anyhow!("Failed to get queue"))?;

            let queue = queue.clone();

            Ok(queue)
        }
        None => {
            let max_pool_size: u32 = 2;

            let db_uri = &env::var("DATABASE_URL")
                .map_err(|_| anyhow::anyhow!("Failed to get DATABASE_URL"))?;

            let mut queue = AsyncQueue::builder()
                // Postgres database url
                .uri(db_uri)
                // Max number of connections that are allowed
                .max_pool_size(max_pool_size)
                .build();

            queue.connect(NoTls).await?;

            let _ = QUEUE.set(Mutex::new(queue));

            match QUEUE.get() {
                Some(queue) => {
                    let queue = queue.lock().map_err(|_| anyhow::anyhow!("Failed to get queue"))?;

                    let queue = queue.clone();

                    Ok(queue)
                }
                None => Err(anyhow::anyhow!("Failed to get queue")),
            }
        }
    }
}
