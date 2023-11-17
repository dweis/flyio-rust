use anyhow::Context;
use axum::{Extension, Router};
use flyio_rust::api;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::{env, net::SocketAddr};
use tower_http::{
    services::ServeDir,
    trace::{DefaultOnRequest, DefaultOnResponse},
};
use tracing::{info, Level};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

const DEFAULT_PORT: u16 = 8080;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "flyio_rust=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

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

    info!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app(db).into_make_service())
        .await
        .context("failed to serve")
}

pub fn app(db: PgPool) -> Router {
    Router::new()
        .merge(api::todos::router())
        .nest_service(
            "/static",
            ServeDir::new("static")
                .precompressed_br()
                .precompressed_gzip(),
        )
        .fallback(api::handle_404)
        .layer(Extension(db))
        .layer(
            tower_http::trace::TraceLayer::new_for_http()
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        )
}
