use crate::data::{todo::Todo, user::User};
use askama::Template;

#[derive(Template)]
#[template(path = "not_found.html")]
pub struct NotFoundTemplate<'a> {
    pub user: &'a Option<User>,
}

#[derive(Template)]
#[template(path = "todos.html")]
pub struct TodosTemplate<'a> {
    pub user: &'a Option<User>,
    pub todos: &'a Vec<Todo>,
}

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate<'a> {
    pub user: &'a Option<User>,
}

#[derive(Template)]
#[template(path = "signup.html")]
pub struct SignupTemplate<'a> {
    pub user: &'a Option<User>,
}

#[derive(Template)]
#[template(path = "partial/todos.html")]
pub struct PartialTodosTemplate<'a> {
    pub todos: &'a Vec<Todo>,
}

#[derive(Template)]
#[template(path = "partial/todo.html")]
pub struct SingleTodoTemplate<'a> {
    pub todo: &'a Todo,
}

#[derive(Template)]
#[template(path = "partial/todo_edit.html")]
pub struct EditTodoTemplate<'a> {
    pub todo: &'a Todo,
}
