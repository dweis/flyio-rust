use askama::Template;
use askama_axum::IntoResponse;
use axum::{http::StatusCode, response::Html, routing::*, Extension, Form};
use serde::Deserialize;
use sqlx::PgPool;
use validator::Validate;

use crate::{data, error::Error};

#[derive(Template)]
#[template(path = "todos.html")]
struct TodosTemplate<'a> {
    todos: &'a Vec<data::todo::Todo>,
}

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateTodoRequest {
    #[validate(length(min = 1, max = 1000))]
    pub content: String,
}

pub fn router() -> Router {
    Router::new().route("/", get(handle_get_todos).post(handle_create_todo))
}

#[axum::debug_handler]
pub async fn handle_get_todos(db: Extension<PgPool>) -> Result<impl IntoResponse, Error> {
    //Result<Html<&'static str>> {
    let todos = data::todo::get_todos(&db).await?;

    let tmpl = TodosTemplate { todos: &todos };

    Ok((StatusCode::OK, Html(tmpl.render().unwrap()).into_response()))
}

#[axum::debug_handler]
pub async fn handle_create_todo(
    db: Extension<PgPool>,
    Form(req): Form<CreateTodoRequest>,
) -> Result<impl IntoResponse, Error> {
    req.validate()?;

    data::todo::create_todo(&db, req.content).await?;

    let todos = data::todo::get_todos(&db).await?;
    let tmpl = TodosTemplate { todos: &todos };

    Ok((StatusCode::OK, Html(tmpl.render().unwrap()).into_response()))
}
