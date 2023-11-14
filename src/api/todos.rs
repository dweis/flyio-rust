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
pub async fn handle_get_todos(db: Extension<PgPool>) -> impl IntoResponse {
    //Result<Html<&'static str>> {
    let todos = data::todo::get_todos(&*db).await;

    match todos {
        Ok(todos) => {
            let tmpl = TodosTemplate { todos: &todos };

            (StatusCode::OK, Html(tmpl.render().unwrap()).into_response())
        }
        _ => (StatusCode::INTERNAL_SERVER_ERROR, "foo".into_response()),
    }
}

#[axum::debug_handler]
pub async fn handle_create_todo(
    db: Extension<PgPool>,
    Form(req): Form<CreateTodoRequest>,
) -> impl IntoResponse {
    req.validate().unwrap();

    data::todo::create_todo(&*db, req.content).await;

    let todos = data::todo::get_todos(&*db).await;

    match todos {
        Ok(todos) => {
            let tmpl = TodosTemplate { todos: &todos };

            (StatusCode::OK, Html(tmpl.render().unwrap()).into_response())
        }
        _ => (StatusCode::INTERNAL_SERVER_ERROR, "foo".into_response()),
    }
}
