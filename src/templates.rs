use crate::data::todo::Todo;
use askama::Template;

#[derive(Template)]
#[template(path = "not_found.html")]
pub struct NotFoundTemplate;

#[derive(Template)]
#[template(path = "todos.html")]
pub struct TodosTemplate<'a> {
    pub todos: &'a Vec<Todo>,
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
