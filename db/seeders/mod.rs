use models::BelongsTo;
use models::{Hashed, Model, Post, User};

use faker_rand::en_us::internet::Email;
use faker_rand::en_us::names::FullName;
use faker_rand::lorem::Sentence;
use faker_rand::lorem::Word;

pub async fn seed() -> anyhow::Result<()> {
    for _ in 0..10 {
        let user = User::create(User {
            name: rand::random::<FullName>().to_string(),
            email: rand::random::<Email>().to_string(),
            password: Hashed::new(rand::random::<Word>().to_string()),
            ..Default::default()
        })
        .await?;
        let mut post_user: BelongsTo<Post, User> = BelongsTo::default();

        post_user.value = user.id.clone();

        for _ in 0..10 {
            Post::create(Post {
                user: post_user.clone(),
                title: rand::random::<Word>().to_string(),
                content: rand::random::<Sentence>().to_string(),
                ..Default::default()
            })
            .await?;
        }
    }

    Ok(())
}
