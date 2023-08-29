use dotenv::dotenv;
use models::{Hashed, Model, Post, User};
use std::env;

use faker_rand::en_us::internet::Email;
use faker_rand::en_us::names::FullName;
use faker_rand::lorem::Word;
use faker_rand::lorem::Sentence;

#[tokio::main]
async fn main() {
    dotenv().ok();

    ensemble::setup(&env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
        .await
        .expect("Failed to set up database pool.");

    for _ in 0..10 {
        let user = User::create(User {
            name: rand::random::<FullName>().to_string(),
            email: rand::random::<Email>().to_string(),
            password: Hashed::new(rand::random::<Word>().to_string()),
            ..Default::default()
        })
        .await
        .unwrap();

        for _ in 0..10 {
            Post::create(Post {
                title: rand::random::<Word>().to_string(),
                content: rand::random::<Sentence>().to_string(),
                user_id: user.id,
                ..Default::default()
            })
            .await
            .unwrap();
        }
    }
}
