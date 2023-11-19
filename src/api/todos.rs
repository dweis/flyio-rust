use askama::Template;
use askama_axum::IntoResponse;
use axum::{extract::Path, http::StatusCode, response::Html, routing::*, Extension, Form};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;
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
        .route("/todos", get(handle_get_todos))
        .route("/todos", post(handle_create_todo_htmx))
        .route(
            "/todos/:todo_id",
            put(handle_update_todo_htmx).delete(handle_delete_todo_htmx),
        )
        .route("/todos/:todo_id/edit", get(handle_edit_todo_htmx))
        .route("/todos/:todo_id/toggle", post(handle_toggle_todo_htmx))
}

#[axum::debug_handler]
pub async fn handle_get_todos(
    auth_session: AuthSession,
    db: Extension<PgPool>,
) -> Result<impl IntoResponse, Error> {
    let user = auth_session.user.unwrap();

    //Result<Html<&'static str>> {
    let todos = data::todo::get_todos(&db, user.user_id).await?;

    let tmpl = TodosTemplate {
        user: &Some(user),
        todos: &todos,
    };

    Ok((StatusCode::OK, Html(tmpl.render().unwrap()).into_response()))
}

#[axum::debug_handler]
pub async fn handle_create_todo_htmx(
    auth_session: AuthSession,
    db: Extension<PgPool>,
    Form(req): Form<CreateTodoRequest>,
) -> Result<impl IntoResponse, Error> {
    let user = auth_session.user.unwrap();

    req.validate()?;

    data::todo::create_todo(&db, user.user_id, req.content).await?;

    let todos = data::todo::get_todos(&db, user.user_id).await?;
    let tmpl = PartialTodosTemplate { todos: &todos };

    Ok((StatusCode::OK, Html(tmpl.render().unwrap()).into_response()))
}

#[axum::debug_handler]
pub async fn handle_delete_todo_htmx(
    auth_session: AuthSession,
    db: Extension<PgPool>,
    Path(todo_id): Path<Uuid>,
) -> Result<impl IntoResponse, Error> {
    let user = auth_session.user.unwrap();

    data::todo::delete_todo_by_id(&db, user.user_id, todo_id).await?;

    Ok((StatusCode::OK, Html("").into_response()))
}

#[axum::debug_handler]
pub async fn handle_toggle_todo_htmx(
    auth_session: AuthSession,
    db: Extension<PgPool>,
    Path(todo_id): Path<Uuid>,
) -> Result<impl IntoResponse, Error> {
    let user = auth_session.user.unwrap();

    data::todo::toggle_todo_by_id(&db, user.user_id, todo_id).await?;

    let todo = data::todo::get_todo_by_id(&db, user.user_id, todo_id).await?;

    let tmpl = SingleTodoTemplate { todo: &todo };

    Ok((StatusCode::OK, Html(tmpl.render().unwrap()).into_response()))
}

#[axum::debug_handler]
pub async fn handle_edit_todo_htmx(
    auth_session: AuthSession,
    db: Extension<PgPool>,
    Path(todo_id): Path<Uuid>,
) -> Result<impl IntoResponse, Error> {
    let user = auth_session.user.unwrap();

    let todo = data::todo::get_todo_by_id(&db, user.user_id, todo_id).await?;

    let tmpl = EditTodoTemplate { todo: &todo };

    Ok((StatusCode::OK, Html(tmpl.render().unwrap()).into_response()))
}

#[axum::debug_handler]
pub async fn handle_update_todo_htmx(
    auth_session: AuthSession,
    db: Extension<PgPool>,
    Path(todo_id): Path<Uuid>,
    Form(req): Form<CreateTodoRequest>,
) -> Result<impl IntoResponse, Error> {
    let user = auth_session.user.unwrap();

    req.validate()?;

    data::todo::update_todo_by_id(&db, user.user_id, todo_id, req.content).await?;

    let todo = data::todo::get_todo_by_id(&db, user.user_id, todo_id).await?;

    let tmpl = SingleTodoTemplate { todo: &todo };

    Ok((StatusCode::OK, Html(tmpl.render().unwrap()).into_response()))
}
