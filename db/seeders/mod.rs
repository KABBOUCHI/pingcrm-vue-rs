use anyhow::Ok;
use models::{Hashed, Model, Post, User};

use faker_rand::en_us::internet::Email;
use faker_rand::en_us::names::FullName;
use faker_rand::lorem::Word;
use faker_rand::lorem::Sentence;

pub async fn seed() -> anyhow::Result<()> {
    for _ in 0..10 {
        let user = User::create(User {
            name: rand::random::<FullName>().to_string(),
            email: rand::random::<Email>().to_string(),
            password: Hashed::new(rand::random::<Word>().to_string()),
            ..Default::default()
        })
        .await?;

        for _ in 0..10 {
            Post::create(Post {
                title: rand::random::<Word>().to_string(),
                content: rand::random::<Sentence>().to_string(),
                user_id: user.id,
                ..Default::default()
            })
            .await?;
        }
    }

    Ok(())
}
