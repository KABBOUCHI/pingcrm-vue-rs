pub use ensemble::{
    relationships::{BelongsTo, HasMany, Relationship},
    types::{DateTime, Hashed},
    Model,
};

#[derive(Debug, Model)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub email: String,
    pub password: Hashed<String>,
    pub created_at: DateTime,
    pub updated_at: DateTime,

    #[model(foreign_key = "user_id")]
    pub posts: HasMany<User, Post>,
}

#[derive(Debug, Model)]
pub struct Post {
    #[model(incrementing)]
    pub id: u64,
    pub title: String,
    pub content: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,

    #[model(foreign_key = "user_id")]
    pub user: BelongsTo<Post, User>,
}
