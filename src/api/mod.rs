use askama::Template;
use askama_axum::IntoResponse;

use axum::{http::StatusCode, response::Html};

use crate::templates::*;

pub mod auth;
pub mod todos;

pub async fn handle_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        Html(NotFoundTemplate.render().unwrap()).into_response(),
    )
}
