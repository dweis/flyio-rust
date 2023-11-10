use anyhow::Context;
use axum::{Extension, Router};
use flyio_rust::api;
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::{env, net::SocketAddr};
use tower_http::services::ServeDir;

const DEFAULT_PORT: u16 = 8080;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let port = env::var("PORT")
        .map(|x| x.parse::<u16>())
        .unwrap_or(Ok(DEFAULT_PORT))
        .unwrap();

    let database_url = env::var("DATABASE_URL").unwrap();

    let db = PgPoolOptions::new()
        .max_connections(20)
        .connect(&database_url)
        .await
        .context("failed to connect to DATABASE_URL")?;

    sqlx::migrate!().run(&db).await?;

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app(db).into_make_service())
        .await
        .context("failed to serve")
}

pub fn app(db: PgPool) -> Router {
    Router::new()
        .merge(api::items::router())
        .nest_service("/static", ServeDir::new("static"))
        .fallback(api::handle_404)
        .layer(Extension(db))
}
