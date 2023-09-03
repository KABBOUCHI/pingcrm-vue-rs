use anyhow::Result;
use axum::{
    extract::{Path, Query},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use dotenv::dotenv;
use models::*;
use std::{env, net::SocketAddr};
use tracing::info;
use tracing_subscriber::{
    prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt, EnvFilter, Layer,
};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().with_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| "pingcrm=info".into()),
        ))
        .init();

    ensemble::setup(&env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
        .await
        .expect("Failed to set up database pool.");

    let app = Router::new()
        .route("/", get(hello_world))
        .route("/users", get(list_users))
        .route("/users/:user_id/posts", get(list_user_posts))
        .route("/posts", get(list_posts));

    let address = SocketAddr::from((
        [0, 0, 0, 0],
        env::var("PORT").map_or(8000, |p| p.parse().unwrap()),
    ));
    info!("⚡ PingCRM started on http://{address}");

    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn hello_world() -> impl IntoResponse {
    "Hello, World!"
}

#[derive(serde::Deserialize)]
struct ListUsersQueryParams {
    #[serde(default)]
    include_posts: bool,
}

async fn list_users(params: Query<ListUsersQueryParams>) -> Json<Vec<User>> {
    let users = User::query()
        .when(params.include_posts, |q| q.with("posts"))
        .get()
        .await
        .unwrap();

    Json(users)
}

async fn list_user_posts(Path(user_id): Path<u64>) -> Json<Vec<Post>> {
    let posts = Post::query()
        .r#where("user_id", "=", user_id)
        .get()
        .await
        .unwrap();

    Json(posts)
}

async fn list_posts() -> Json<Vec<Post>> {
    let posts = Post::all().await.unwrap();

    Json(posts)
}
