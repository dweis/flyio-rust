use askama::Template;
use askama_axum::IntoResponse;
use axum::{extract::Path, http::StatusCode, response::Html, routing::*, Extension, Form};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use crate::{data, error::Error};

#[derive(Template)]
#[template(path = "todos.html")]
struct TodosTemplate<'a> {
    todos: &'a Vec<data::todo::Todo>,
}

#[derive(Template)]
#[template(path = "todos_partial.html")]
struct PartialTodosTemplate<'a> {
    todos: &'a Vec<data::todo::Todo>,
}

#[derive(Template)]
#[template(path = "single_todo.html")]
struct SingleTodoTemplate<'a> {
    todo: &'a data::todo::Todo,
}

#[derive(Template)]
#[template(path = "edit_todo.html")]
struct EditTodoTemplate<'a> {
    todo: &'a data::todo::Todo,
}

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateTodoRequest {
    #[validate(length(min = 1, max = 1000))]
    pub content: String,
}

pub fn router() -> Router {
    Router::new()
        .route("/", get(handle_get_todos).post(handle_create_todo))
        .route("/todos", post(handle_create_todo_htmx))
        .route(
            "/todos/:todo_id",
            put(handle_update_todo_htmx).delete(handle_delete_todo_htmx),
        )
        .route("/todos/:todo_id/edit", get(handle_edit_todo_htmx))
        .route("/todos/:todo_id/toggle", post(handle_toggle_todo_htmx))
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

#[axum::debug_handler]
pub async fn handle_create_todo_htmx(
    db: Extension<PgPool>,
    Form(req): Form<CreateTodoRequest>,
) -> Result<impl IntoResponse, Error> {
    req.validate()?;

    data::todo::create_todo(&db, req.content).await?;

    let todos = data::todo::get_todos(&db).await?;
    let tmpl = PartialTodosTemplate { todos: &todos };

    Ok((StatusCode::OK, Html(tmpl.render().unwrap()).into_response()))
}

#[axum::debug_handler]
pub async fn handle_delete_todo_htmx(
    db: Extension<PgPool>,
    Path(todo_id): Path<Uuid>,
) -> Result<impl IntoResponse, Error> {
    data::todo::delete_todo_by_id(&db, todo_id).await?;

    Ok((StatusCode::OK, Html("").into_response()))
}

#[axum::debug_handler]
pub async fn handle_toggle_todo_htmx(
    db: Extension<PgPool>,
    Path(todo_id): Path<Uuid>,
) -> Result<impl IntoResponse, Error> {
    data::todo::toggle_todo_by_id(&db, todo_id).await?;

    let todo = data::todo::get_todo_by_id(&db, todo_id).await?;

    let tmpl = SingleTodoTemplate { todo: &todo };

    Ok((StatusCode::OK, Html(tmpl.render().unwrap()).into_response()))
}

#[axum::debug_handler]
pub async fn handle_edit_todo_htmx(
    db: Extension<PgPool>,
    Path(todo_id): Path<Uuid>,
) -> Result<impl IntoResponse, Error> {
    let todo = data::todo::get_todo_by_id(&db, todo_id).await?;

    let tmpl = EditTodoTemplate { todo: &todo };

    Ok((StatusCode::OK, Html(tmpl.render().unwrap()).into_response()))
}

#[axum::debug_handler]
pub async fn handle_update_todo_htmx(
    db: Extension<PgPool>,
    Path(todo_id): Path<Uuid>,
    Form(req): Form<CreateTodoRequest>,
) -> Result<impl IntoResponse, Error> {
    req.validate()?;

    data::todo::update_todo_by_id(&db, todo_id, req.content).await?;

    let todo = data::todo::get_todo_by_id(&db, todo_id).await?;

    let tmpl = SingleTodoTemplate { todo: &todo };

    Ok((StatusCode::OK, Html(tmpl.render().unwrap()).into_response()))
}
