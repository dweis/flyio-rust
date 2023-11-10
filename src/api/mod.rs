use askama::Template;
use askama_axum::IntoResponse;

use axum::{http::StatusCode, response::Html};

pub mod items;

#[derive(Template)]
#[template(path = "not_found.html")]
struct NotFoundTemplate;

pub async fn handle_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        Html(NotFoundTemplate.render().unwrap()).into_response(),
    )
}
