// TODO: generate this file automatically

mod m_2023_09_30_000000_create_users_table;
mod m_2023_09_30_000001_create_posts_table;

use std::collections::HashMap;

use anyhow::Ok;
use ensemble::{migrations::Migrator, query::Builder};

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
    let tables =  unsafe { Builder::raw_sql("SELECT TABLE_NAME as name FROM information_schema.TABLES WHERE TABLE_SCHEMA = DATABASE()", vec![]) }.await?;

    let tables = tables
        .into_iter()
        .map(rbs::from_value::<HashMap<String, String>>);

    for table in tables {
        let table = table?;

        if let Some(name) = table.get("name") {
            println!("Dropping table {}", name);

            unsafe {
                // Builder::raw_sql("DROP TABLE IF EXISTS ?", vec![ensemble::value::for_db(name)?]).await?;
                Builder::raw_sql(&format!("DROP TABLE IF EXISTS {}", name), vec![]).await?;
            }
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
    let m  = migrator().await?;
    let pending_migrations = m.pending();
    let store = m.status();

    println!(
        "{:<70} {:<30}",
        "Migration name",
        "Batch / Status",
    );

    for migration in store {
        println!(
            "{:<70} {:<30}",
            migration.migration,
            format!("[{}] Ran", migration.batch),
        );
    }

    for migration in pending_migrations {
        println!(
            "{:<70} {:<30}",
            migration.0,
            "Pending".to_string(),
        );
    }

    Ok(())
}
