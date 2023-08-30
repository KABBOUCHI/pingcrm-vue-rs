// TODO: generate this file automatically

mod m_2023_09_30_000000_create_users_table;
mod m_2023_09_30_000001_create_posts_table;

use anyhow::Ok;
use ensemble::migrations::Migrator;

async fn migrator() -> anyhow::Result<Migrator> {
    let mut migrator = Migrator::new().await?;

    migrator.register(
        "m_2023_09_30_000000_create_users_table".to_string(),
        Box::new(m_2023_09_30_000000_create_users_table::CreateUsersTable),
    );

    migrator.register(
        "m_2023_09_30_000001_create_posts_table".to_string(),
        Box::new(m_2023_09_30_000001_create_posts_table::CreatePostsTable),
    );

    Ok(migrator)
}

pub async fn migrate() -> anyhow::Result<()> {
    migrator().await?.run().await?;

    println!("Migrated database.");

    Ok(())
}

pub async fn rollback(batches: u64) -> anyhow::Result<()> {
    migrator().await?.rollback(batches).await?;

    println!("Rolled back database.");
    
    Ok(())
}

pub async fn status() -> anyhow::Result<()> {
    let store = migrator().await?.status();

    println!("Database migrations status:");

    for migration in store {
        dbg!(&migration);
    }

    Ok(())
}
