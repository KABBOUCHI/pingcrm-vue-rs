use dotenv::dotenv;
use models::{Model, User, Hashed};
use std::env;

use faker_rand::en_us::names::FullName;
use faker_rand::en_us::internet::Email;
use faker_rand::lorem::Word;

#[tokio::main]
async fn main() {
    dotenv().ok();

    ensemble::setup(&env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
        .await
        .expect("Failed to set up database pool.");

    for _ in 0..10 {
        User::create(User {
            name: rand::random::<FullName>().to_string(),
            email: rand::random::<Email>().to_string(),
            password: Hashed::new(rand::random::<Word>().to_string()),
            ..Default::default()
         }).await.unwrap();
    }
}
