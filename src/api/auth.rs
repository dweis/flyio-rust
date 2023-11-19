use askama::Template;
use askama_axum::IntoResponse;
use axum::{
    http::StatusCode,
    response::{Html, Redirect},
    routing::*,
    Extension, Form,
};
use serde::Deserialize;
use sqlx::PgPool;
use tracing::warn;
use validator::Validate;

use crate::data::user::AuthSession;
use crate::{data, error::Error, templates::*};

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateTodoRequest {
    #[validate(length(min = 1, max = 1000))]
    pub content: String,
}

pub fn router() -> Router {
    Router::new()
        .route("/login", get(handle_login).post(handle_login_post))
        .route("/signup", get(handle_signup).post(handle_signup_post))
        .route("/logout", get(handle_logout))
}

#[axum::debug_handler]
pub async fn handle_login() -> Result<impl IntoResponse, Error> {
    let tmpl = LoginTemplate {};

    Ok((StatusCode::OK, Html(tmpl.render().unwrap()).into_response()))
}

#[axum::debug_handler]
pub async fn handle_signup() -> Result<impl IntoResponse, Error> {
    let tmpl = SignupTemplate {};

    Ok((StatusCode::OK, Html(tmpl.render().unwrap()).into_response()))
}

#[axum::debug_handler]
pub async fn handle_logout(mut auth_session: AuthSession) -> impl IntoResponse {
    match auth_session.logout() {
        Ok(_) => Redirect::to("/login").into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SignupForm {
    #[validate(length(min = 1, max = 1000))]
    pub email: String,
    #[validate(length(min = 1, max = 1000))]
    pub password: String,
}

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct LoginForm {
    #[validate(length(min = 4, max = 1000))]
    pub email: String,
    #[validate(length(min = 1, max = 1000))]
    pub password: String,
}

#[axum::debug_handler]
pub async fn handle_login_post(
    mut auth_session: AuthSession,
    Form(creds): Form<data::user::Credentials>,
) -> impl IntoResponse {
    let user = match auth_session.authenticate(creds).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            return LoginTemplate {}.into_response();
        }
        Err(e) => {
            warn!("Error authenticating user: {:?}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    if auth_session.login(&user).await.is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    Redirect::to("/").into_response()
}

#[axum::debug_handler]
pub async fn handle_signup_post(
    db: Extension<PgPool>,
    Form(signup_form): Form<SignupForm>,
) -> Result<impl IntoResponse, Error> {
    signup_form.validate()?;

    data::user::create_user(&db, &signup_form.email, &signup_form.password).await?;

    Ok(Redirect::to("/login").into_response())
}
