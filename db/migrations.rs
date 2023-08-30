// TODO: generate this file automatically

mod m_2023_09_30_000000_create_users_table;
mod m_2023_09_30_000001_create_posts_table;

use std::collections::HashMap;

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

pub async fn wipe() -> anyhow::Result<()> {
    let mut conn = ensemble::get_connection().await?;

    // SET FOREIGN_KEY_CHECKS = 0

    let tables =  conn.get_values("SELECT TABLE_NAME as name FROM information_schema.TABLES WHERE TABLE_SCHEMA = DATABASE()", vec![]).await?;

    let tables = tables
        .into_iter()
        .map(rbs::from_value::<HashMap<String, String>>);

    for table in tables {
        let table = table?;

        if let Some(name) = table.get("name") {
            println!("Dropping table {}", name);
            
            conn.exec("DROP TABLE IF EXISTS ?", vec![rbs::to_value!(name)]).await?;
        }
    }

    Ok(())
}

pub async fn fresh() -> anyhow::Result<()> {
    wipe().await?;
    migrate().await
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
