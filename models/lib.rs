pub use ensemble::{
    relationships::{BelongsTo, HasMany, Relationship},
    types::{DateTime, Hashed},
    Model,
};
use ensemble::types::Uuid;

#[derive(Debug, Model, Clone)]
pub struct User {
    #[model(uuid)]
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: Hashed<String>,
    pub created_at: DateTime,
    pub updated_at: DateTime,

    #[model(foreign_key = "user_id")]
    pub posts: HasMany<User, Post>,
}

#[derive(Debug, Model, Clone)]
pub struct Post {
    #[model(uuid)]
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,

    #[model(foreign_key = "user_id")]
    pub user: BelongsTo<Post, User>,
}
