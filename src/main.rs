use anyhow::Context;
use axum::{error_handling::HandleErrorLayer, http::StatusCode, BoxError, Extension, Router};
use axum_login::{
    login_required,
    tower_sessions::{Expiry, RedisStore, SessionManagerLayer},
    AuthManagerLayerBuilder,
};
use flyio_rust::{api, data::user::Backend};
use fred::prelude::*;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::{env, net::SocketAddr};
use time::Duration;
use tower::ServiceBuilder;
use tower_http::{
    services::ServeDir,
    trace::{DefaultOnRequest, DefaultOnResponse},
};
use tracing::{info, Level};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

const DEFAULT_PORT: u16 = 8080;

mod auth;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "flyio_rust=debug,tower_http=debug,axum::rejection=trace,sqlx=info".into()
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

    let redis_url = env::var("REDIS_URL").unwrap();

    let redis_config = RedisConfig::from_url(redis_url.as_str()).unwrap();
    let redis_client = RedisClient::new(redis_config, None, None, None);

    redis_client.connect();
    redis_client.wait_for_connect().await?;

    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    info!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app(db, redis_client).into_make_service())
        .await
        .context("failed to serve")
}

pub fn app(db: PgPool, redis: RedisClient) -> Router {
    // Session layer.
    //
    // This uses `tower-sessions` to establish a layer that will provide the session
    // as a request extension.
    let session_store = RedisStore::new(redis);
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::days(1)));

    // Auth service.
    //
    // This combines the session layer with our backend to establish the auth
    // service which will provide the auth session as a request extension.
    let backend = flyio_rust::data::user::Backend::new(db.clone());
    let auth_service = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(|_: BoxError| async {
            StatusCode::BAD_REQUEST
        }))
        .layer(AuthManagerLayerBuilder::new(backend, session_layer).build());

    Router::new()
        .merge(api::auth::router())
        .merge(api::todos::router().route_layer(login_required!(Backend, login_url = "/login")))
        .layer(auth_service)
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
