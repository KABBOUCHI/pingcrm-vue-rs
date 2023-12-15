use clap::{Parser, Subcommand};
use dotenv::dotenv;
use std::env;

mod migrations {
    use convert_case::{Case, Casing};
    use std::io::Write;

    macros::migrations!();

    pub async fn make(name: &String) -> anyhow::Result<()> {
        let snake_case = name.to_case(Case::Snake);
        let table_name = snake_case.replace("create_", "").replace("_table", "");
        let pascale_case = name.to_case(Case::Pascal);

        let mut file = std::fs::File::create(format!(
            "./db/migrations/m_{}_{}.rs",
            chrono::Utc::now().format("%Y_%m_%d_%H%M%S"),
            snake_case
        ))
        .expect("Error encountered while creating file!");

        let text = format!(
            r#"
use ensemble::migrations::{{Error, Migration, Schema}};

#[derive(Debug, Default)]
pub struct {pascale_case};

#[ensemble::async_trait]
impl Migration for {pascale_case} {{
    async fn up(&self) -> Result<(), Error> {{
        Schema::create("{table_name}", |table| {{
            table.id();
            table.timestamps();
        }})
        .await
    }}

    async fn down(&self) -> Result<(), Error> {{
        Schema::drop("{table_name}").await
    }}
}}"#
        );

        file.write_all(text.as_bytes())?;

        std::process::Command::new("cargo")
            .args(["clean", "-p", "macros"])
            .output()?;

        Ok(())
    }
}
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
    Down {
        /// Number of batches to rollback
        #[clap(default_value = "1")]
        batches: u64,
    },
    /// Drop all tables and re-run all migrations
    Fresh,
    /// Show the status of each migration
    Status,
    /// Create a new migration file
    Make { name: String },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    ensemble::setup(&env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
        .expect("Failed to set up database pool.");

    let cli = Cli::parse();

    match &cli.command {
        Commands::Migrate { command } => match command {
            MigrateCommands::Fresh => {
                migrations::fresh().await?;
            }
            MigrateCommands::Up => {
                migrations::migrate().await?;
            }
            MigrateCommands::Down { batches } => {
                migrations::rollback(*batches).await?;
            }
            MigrateCommands::Status => {
                migrations::status().await?;
            }
            MigrateCommands::Make { name } => {
                migrations::make(name).await?;
            }
        },
        Commands::Seed {} => {
            seeders::seed().await?;

            println!("Seeded database.")
        }
    }

    Ok(())
}
