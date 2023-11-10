use askama::Template;
use askama_axum::IntoResponse;
use axum::{
   http::StatusCode, response::Html, routing::*, Extension, Form,
};
use serde::Deserialize;
use sqlx::PgPool;
use validator::Validate;

use crate::data;

#[derive(Template)]
#[template(path = "items.html")]
struct ItemsTemplate<'a> {
    items: &'a Vec<data::item::Item>,
}

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateItemRequest {
    #[validate(length(min = 1, max = 1000))]
    pub content: String,
}

pub fn router() -> Router {
    Router::new().route("/items", get(handle_get_items).post(handle_create_item))
}

#[axum::debug_handler]
pub async fn handle_get_items(db: Extension<PgPool>) -> impl IntoResponse {
    //Result<Html<&'static str>> {
    let items = data::item::get_items(&*db).await;

    match items {
        Ok(items) => {
            let tmpl = ItemsTemplate { items: &items };

            (StatusCode::OK, Html(tmpl.render().unwrap()).into_response())
        }
        _ => (StatusCode::INTERNAL_SERVER_ERROR, "foo".into_response()),
    }
}

#[axum::debug_handler]
pub async fn handle_create_item(
    db: Extension<PgPool>,
    Form(req): Form<CreateItemRequest>,
) -> impl IntoResponse {
    req.validate().unwrap();

    data::item::create_item(&*db, req.content).await;

    let items = data::item::get_items(&*db).await;

    match items {
        Ok(items) => {
            let tmpl = ItemsTemplate { items: &items };

            (StatusCode::OK, Html(tmpl.render().unwrap()).into_response())
        }
        _ => (StatusCode::INTERNAL_SERVER_ERROR, "foo".into_response()),
    }
}
