use anyhow::Result;
use axum::{response::IntoResponse, routing::get, Router, Json};
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
        .route("/users", get(list_users));

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

async fn list_users() -> Json<Vec<User>> {
    let users = User::all().await.unwrap();

    Json(users)
}
