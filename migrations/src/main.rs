use dotenv::dotenv;
use std::env;

mod migrations;

#[tokio::main]
async fn main() {
    dotenv().ok();

    ensemble::setup(&env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
        .await
        .expect("Failed to set up database pool.");

    ensemble::migrate!(migrations::CreateUsersTable)
        .await
        .expect("Failed to run migrations.");
}
