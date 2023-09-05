use models::{Hashed, Model, Post, User};

use faker_rand::en_us::internet::Email;
use faker_rand::en_us::names::FullName;
use faker_rand::lorem::Sentence;
use faker_rand::lorem::Word;

pub async fn seed() -> anyhow::Result<()> {
    for _ in 0..10 {
        let mut user = User::create(User {
            name: rand::random::<FullName>().to_string(),
            email: rand::random::<Email>().to_string(),
            password: Hashed::new(rand::random::<Word>().to_string()),
            ..Default::default()
        })
        .await?;

        for _ in 0..10 {
            user.posts.create(Post {
                title: rand::random::<Word>().to_string(),
                content: rand::random::<Sentence>().to_string(),
                ..Default::default()
            }).await?;
        }
    }

    Ok(())
}
