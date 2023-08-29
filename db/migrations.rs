// TODO: generate this file automatically

mod m_2023_09_30_000000_create_users_table;
mod m_2023_09_30_000001_create_posts_table;

use ensemble::migrations::Migrator;

pub async fn migrate() -> anyhow::Result<()> {
    let mut migrator = Migrator::new().await?;

    migrator.register(
        "m_2023_09_30_000000_create_users_table".to_string(),
        Box::new(m_2023_09_30_000000_create_users_table::CreateUsersTable),
    );

    migrator.register(
        "m_2023_09_30_000001_create_posts_table".to_string(),
        Box::new(m_2023_09_30_000001_create_posts_table::CreatePostsTable),
    );

    migrator.run().await?;

    println!("Migrated database.");

    Ok(())
}

pub async fn rollback() -> anyhow::Result<()> {
    todo!("rollback migrations")
}

pub async fn status() -> anyhow::Result<()> {
    todo!("migrations status")
}
