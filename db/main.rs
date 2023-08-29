use clap::{Parser, Subcommand};
use dotenv::dotenv;
use std::env;

// TODO: automatically generate this mod
mod migrations;
mod seeders;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Seed the database with records
    Seed {},

    /// Run database migrations
    Migrate {
        #[command(subcommand)]
        command: MigrateCommands,
    },
}

#[derive(Debug, Subcommand)]
enum MigrateCommands {
    /// Run the next database migration
    Up,
    /// Rollback the last database migration
    Down,
    /// Show the status of each migration
    Status,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    ensemble::setup(&env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
        .await
        .expect("Failed to set up database pool.");

    let cli = Cli::parse();

    match &cli.command {
        Commands::Migrate { command } => match command {
            MigrateCommands::Up => {
                migrations::migrate().await?;
            }
            MigrateCommands::Down => {
                migrations::rollback().await?;
            }
            MigrateCommands::Status => {
                migrations::status().await?;
            }
        },
        Commands::Seed {} => {
            seeders::seed().await?;

            println!("Seeded database.")
        }
    }

    Ok(())
}