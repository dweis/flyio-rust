use askama::Template;
use askama_axum::IntoResponse;

use crate::data::user::AuthSession;
use axum::{
    http::StatusCode,
    response::{Html, Redirect},
};

use crate::templates::*;

pub mod auth;
pub mod todos;

pub async fn handle_index(auth_session: AuthSession) -> impl IntoResponse {
    match auth_session.user {
        Some(_) => Redirect::to("/todos").into_response(),
        None => Redirect::to("/login").into_response(),
    }
}

pub async fn handle_404(auth_session: AuthSession) -> impl IntoResponse {
    let user = auth_session.user;
    let tmpl = NotFoundTemplate { user: &user };
    (
        StatusCode::NOT_FOUND,
        Html(tmpl.render().unwrap()).into_response(),
    )
}
