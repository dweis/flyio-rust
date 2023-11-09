use std::env;
use std::net::SocketAddr;
use anyhow::Context;
use axum::{response::Html, routing::get, Router, Extension, http::StatusCode, Form};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;
use uuid::Uuid;
use askama::Template;
use askama_axum::*;
use validator::Validate;

mod error;

use error::Error;

const DEFAULT_PORT:u16 = 8080;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let port = env::var("PORT")
        .map(|x| x.parse::<u16>())
        .unwrap_or(Ok(DEFAULT_PORT))
        .unwrap();

    let database_url = env::var("DATABASE_URL")
        .unwrap();

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
        .route("/", get(get_items).post(create_item))
        .layer(Extension(db))
}

#[serde_with::serde_as]
#[derive(Serialize,Debug)]
#[serde(rename_all = "camelCase")]
struct Item {
    item_id: Uuid,
    content: String,
    // `OffsetDateTime`'s default serialization format is not standard.
    #[serde_as(as = "Rfc3339")]
    created_at: OffsetDateTime,
}

pub type Result<T, E = Error> = ::std::result::Result<T, E>;

#[derive(Template)]
#[template(path="items.html")]
struct ItemsTemplate<'a> {
    items: &'a Vec<Item>
}

#[axum::debug_handler]
async fn get_items(db: Extension<PgPool>) -> impl IntoResponse { //Result<Html<&'static str>> {
    let items = sqlx::query_as!(Item,
        // language=PostgreSQL
        "
            select item_id, content, created_at
            from item
            order by created_at
        ",
        )
        .fetch_all(&*db)
        .await;

    match items {
        Ok(items) => {

            let tmpl = ItemsTemplate { items: &items };

            (StatusCode::OK, Html(tmpl.render().unwrap()).into_response())

        },
        _ =>  (StatusCode::INTERNAL_SERVER_ERROR, "foo".into_response())
    }
}

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
struct CreateItemRequest {
    #[validate(length(min = 1, max = 1000))]
    content: String,
}

#[axum::debug_handler]
async fn create_item(
    db: Extension<PgPool>,
    Form(req): Form<CreateItemRequest>) -> impl IntoResponse {
    req.validate().unwrap();


    let item = sqlx::query_as!(
        Item,
        r#"
            with inserted_item as (
                insert into item(content)
                values($1)
                returning item_id, content, created_at
            ) 
            select item_id, content, created_at from inserted_item
        "#,
        req.content
    )
        .fetch_one(&*db)
        .await;

    let items = sqlx::query_as!(Item,
        // language=PostgreSQL
        "
            select item_id, content, created_at
            from item
            order by created_at
        ",
        )
        .fetch_all(&*db)
        .await;

    match items {
        Ok(items) => {

            let tmpl = ItemsTemplate { items: &items };

            (StatusCode::OK, Html(tmpl.render().unwrap()).into_response())

        },
        _ =>  (StatusCode::INTERNAL_SERVER_ERROR, "foo".into_response())
    }
}
